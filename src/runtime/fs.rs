use alloc::boxed::Box;
use std::{
    fs,
    io::{Write, stderr, stdin},
};

use java_runtime::{File, FileDescriptorId, FileStat, FileType, IOError, IOResult, RuntimeFs, filesystem_path_candidates};

use super::{
    RuntimeImpl,
    io::{FileImpl, InputStreamFile, MemoryFile, WriteStreamFile},
};
use core::sync::atomic::Ordering;

impl<T> RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    pub(crate) fn register_file(&self, file: Box<dyn File>) -> FileDescriptorId {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
        self.file_table.lock().insert(fd, file);
        FileDescriptorId::new(fd)
    }

    pub fn add_virtual_file(&self, path: impl Into<String>, data: Vec<u8>) {
        self.virtual_files.lock().insert(path.into(), data);
    }
}

#[async_trait::async_trait]
impl<T> RuntimeFs for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    fn stdin(&self) -> IOResult<FileDescriptorId> {
        let file = Box::new(InputStreamFile::new(stdin()));
        Ok(self.register_file(file))
    }

    fn stdout(&self) -> IOResult<FileDescriptorId> {
        let file = Box::new(WriteStreamFile::new(self.stdout.clone()));
        Ok(self.register_file(file))
    }

    fn stderr(&self) -> IOResult<FileDescriptorId> {
        let file = Box::new(WriteStreamFile::new(stderr()));
        Ok(self.register_file(file))
    }

    async fn open(&self, path: &str, write: bool) -> IOResult<FileDescriptorId> {
        if !write {
            for candidate in filesystem_path_candidates(path) {
                if let Some(data) = self.virtual_files.lock().get(&candidate).cloned() {
                    let file = Box::new(MemoryFile::new(data));
                    return Ok(self.register_file(file));
                }
            }
        }
        let mut last_error = IOError::NotFound;
        for candidate in filesystem_path_candidates(path) {
            match FileImpl::open(&candidate, write) {
                Ok(file) => return Ok(self.register_file(Box::new(file))),
                Err(error) => last_error = error,
            }
        }
        Err(last_error)
    }

    fn get_file(&self, fd: FileDescriptorId) -> IOResult<Box<dyn File>> {
        self.file_table.lock().get(&fd.id()).cloned().ok_or(IOError::NotFound)
    }

    fn close_file(&self, fd: FileDescriptorId) {
        self.file_table.lock().remove(&fd.id());
    }

    async fn unlink(&self, path: &str) -> IOResult<()> {
        for candidate in filesystem_path_candidates(path) {
            if fs::remove_file(&candidate).is_ok() {
                return Ok(());
            }
        }
        Err(IOError::NotFound)
    }

    async fn metadata(&self, path: &str) -> IOResult<FileStat> {
        for candidate in filesystem_path_candidates(path) {
            if let Some(data) = self.virtual_files.lock().get(&candidate) {
                return Ok(FileStat {
                    size: data.len() as u64,
                    r#type: FileType::File,
                });
            }
        }

        for candidate in filesystem_path_candidates(path) {
            if let Ok(metadata) = fs::metadata(&candidate) {
                let file_type = if metadata.is_dir() { FileType::Directory } else { FileType::File };
                return Ok(FileStat {
                    size: metadata.len(),
                    r#type: file_type,
                });
            }
        }
        Err(IOError::NotFound)
    }
    fn list_directory(&self, path: &str) -> Vec<String> {
        fs::read_dir(path)
            .map(|entries| {
                entries
                    .filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

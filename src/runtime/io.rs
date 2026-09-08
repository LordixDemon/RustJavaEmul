use alloc::sync::Arc;
use parking_lot::Mutex;
use std::{
    fs::{self, OpenOptions},
    io::{self, Read, Seek, Write},
};

use java_runtime::{File, FileSize, FileStat, FileType, IOError, IOResult};

pub struct WriteStreamFile<W>
where
    W: Write + Send + Sync + 'static,
{
    write: Arc<Mutex<W>>,
}

impl<W> WriteStreamFile<W>
where
    W: Write + Send + Sync + 'static,
{
    pub fn new(write: W) -> Self {
        Self {
            write: Arc::new(Mutex::new(write)),
        }
    }
}

#[async_trait::async_trait]
impl<W> File for WriteStreamFile<W>
where
    W: Write + Send + Sync + 'static,
{
    async fn read(&mut self, _buf: &mut [u8]) -> IOResult<usize> {
        Err(IOError::Unsupported)
    }

    async fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        self.write.lock().write(buf).map_err(|_| IOError::Unsupported)
    }

    async fn seek(&mut self, _pos: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn tell(&self) -> IOResult<FileSize> {
        Err(IOError::Unsupported)
    }

    async fn set_len(&mut self, _len: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn metadata(&self) -> IOResult<FileStat> {
        Err(IOError::Unsupported)
    }
}

impl<W> Clone for WriteStreamFile<W>
where
    W: Write + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self { write: self.write.clone() }
    }
}

pub struct InputStreamFile<R>
where
    R: Read + Send + Sync + 'static,
{
    read: Arc<Mutex<R>>,
}

impl<R> InputStreamFile<R>
where
    R: Read + Send + Sync + 'static,
{
    pub fn new(read: R) -> Self {
        Self {
            read: Arc::new(Mutex::new(read)),
        }
    }
}

#[async_trait::async_trait]
impl<R> File for InputStreamFile<R>
where
    R: Read + Send + Sync + 'static,
{
    async fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        self.read.lock().read(buf).map_err(|_| IOError::Unsupported)
    }

    async fn write(&mut self, _buf: &[u8]) -> IOResult<usize> {
        Err(IOError::Unsupported)
    }

    async fn seek(&mut self, _pos: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn tell(&self) -> IOResult<FileSize> {
        Err(IOError::Unsupported)
    }

    async fn set_len(&mut self, _len: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn metadata(&self) -> IOResult<FileStat> {
        Err(IOError::Unsupported)
    }
}

impl<R> Clone for InputStreamFile<R>
where
    R: Read + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self { read: self.read.clone() }
    }
}

#[derive(Clone)]
pub struct MemoryFile {
    data: Arc<Vec<u8>>,
    pos: Arc<Mutex<FileSize>>,
}

impl MemoryFile {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: Arc::new(data),
            pos: Arc::new(Mutex::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl File for MemoryFile {
    async fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        let mut pos = self.pos.lock();
        let offset = (*pos as usize).min(self.data.len());
        let remaining = &self.data[offset..];
        let len = remaining.len().min(buf.len());
        buf[..len].copy_from_slice(&remaining[..len]);
        *pos += len as FileSize;
        Ok(len)
    }

    async fn write(&mut self, _buf: &[u8]) -> IOResult<usize> {
        Err(IOError::Unsupported)
    }

    async fn seek(&mut self, pos: FileSize) -> IOResult<()> {
        *self.pos.lock() = pos;
        Ok(())
    }

    async fn tell(&self) -> IOResult<FileSize> {
        Ok(*self.pos.lock())
    }

    async fn set_len(&mut self, _len: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn metadata(&self) -> IOResult<FileStat> {
        Ok(FileStat {
            size: self.data.len() as FileSize,
            r#type: FileType::File,
        })
    }
}

#[derive(Clone)]
pub struct FileImpl {
    file: Arc<Mutex<fs::File>>,
}

impl FileImpl {
    pub fn open(path: &str, write: bool) -> IOResult<Self> {
        let mut options = OpenOptions::new();
        let file = options.read(true).write(write).create(write).open(path).map_err(|_| IOError::NotFound)?;

        Ok(Self {
            file: Arc::new(Mutex::new(file)),
        })
    }
}

#[async_trait::async_trait]
impl File for FileImpl {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError> {
        self.file.lock().read(buf).map_err(|_| IOError::Unsupported)
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, IOError> {
        self.file.lock().write(buf).map_err(|_| IOError::Unsupported)
    }

    async fn seek(&mut self, pos: FileSize) -> Result<(), IOError> {
        self.file
            .lock()
            .seek(io::SeekFrom::Start(pos))
            .map(|_| ())
            .map_err(|_| IOError::Unsupported)
    }

    async fn tell(&self) -> Result<FileSize, IOError> {
        self.file
            .lock()
            .seek(io::SeekFrom::Current(0))
            .map(|pos| pos as FileSize)
            .map_err(|_| IOError::Unsupported)
    }

    async fn set_len(&mut self, len: FileSize) -> Result<(), IOError> {
        self.file.lock().set_len(len).map_err(|_| IOError::Unsupported)
    }

    async fn metadata(&self) -> Result<FileStat, IOError> {
        let metadata = self.file.lock().metadata().map_err(|_| IOError::NotFound)?;
        Ok(FileStat {
            size: metadata.len(),
            r#type: FileType::File,
        })
    }
}

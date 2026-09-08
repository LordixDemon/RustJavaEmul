use alloc::{collections::BTreeMap, string::String as RustString, sync::Arc, vec, vec::Vec};
use core::{
    iter,
    sync::atomic::{AtomicU64, Ordering},
};

use hashbrown::HashMap;
use parking_lot::{Mutex, RwLock};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{File, InputStream},
        lang::String,
        util::{Enumeration, zip::ZipEntry},
    },
    zip::ZipArchive,
};

static NEXT_ZIP_HANDLE: AtomicU64 = AtomicU64::new(1);

struct CachedZip {
    archive: ZipArchive<Vec<u8>>,
    lookup_exact: HashMap<RustString, usize>,
    lookup_lower: HashMap<RustString, usize>,
    manifest_mf: Option<usize>,
    manifest_any: Option<usize>,
    entry_cache: RwLock<HashMap<usize, Arc<[u8]>>>,
}

static ZIP_CACHE: Mutex<BTreeMap<u64, Arc<CachedZip>>> = Mutex::new(BTreeMap::new());

impl CachedZip {
    fn new(raw: Vec<u8>) -> Option<Self> {
        let archive = ZipArchive::new(raw).ok()?;
        let mut lookup_exact = HashMap::with_capacity(archive.entries().len());
        let mut lookup_lower = HashMap::with_capacity(archive.entries().len());
        let mut manifest_mf = None;
        let mut manifest_any = None;

        for (index, entry) in archive.entries().iter().enumerate() {
            let norm = normalize_zip_name(&entry.name);
            lookup_exact.insert(norm.clone(), index);
            lookup_lower.entry(norm.to_ascii_lowercase()).or_insert(index);

            let entry_name = &entry.name;
            if is_manifest_name(entry_name) {
                if entry_name.to_ascii_uppercase().ends_with("MANIFEST.MF") {
                    manifest_mf = Some(index);
                } else if manifest_any.is_none() {
                    manifest_any = Some(index);
                }
            }
        }

        Some(Self {
            archive,
            lookup_exact,
            lookup_lower,
            manifest_mf,
            manifest_any,
            entry_cache: RwLock::new(HashMap::new()),
        })
    }

    fn find_index(&self, name: &str) -> Option<usize> {
        let norm = normalize_zip_name(name);
        if let Some(&idx) = self.lookup_exact.get(&norm) {
            return Some(idx);
        }
        let lower = norm.to_ascii_lowercase();
        if let Some(&idx) = self.lookup_lower.get(&lower) {
            return Some(idx);
        }
        if is_manifest_name(name) {
            return self.manifest_mf.or(self.manifest_any);
        }
        None
    }

    fn extract_entry(&self, index: usize) -> Option<Arc<[u8]>> {
        {
            let cache = self.entry_cache.read();
            if let Some(cached) = cache.get(&index) {
                return Some(cached.clone());
            }
        }
        let file = self.archive.by_index(index).ok()?;
        let data = file.extract().ok()?;
        let arc: Arc<[u8]> = data.into();
        self.entry_cache.write().insert(index, arc.clone());
        Some(arc)
    }
}

// class java.util.zip.ZipFile
pub struct ZipFile;

impl ZipFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/zip/ZipFile",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getEntry",
                    "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;",
                    Self::get_entry,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getInputStream",
                    "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;",
                    Self::get_input_stream,
                    Default::default(),
                ),
                JavaMethodProto::new("entries", "()Ljava/util/Enumeration;", Self::entries, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("zipData", "[B", Default::default()),
                JavaFieldProto::new("zipHandle", "J", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn get_cached_zip(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Arc<CachedZip>> {
        let handle: i64 = jvm.get_field(this, "zipHandle", "J").await.unwrap_or(0);
        if handle > 0 {
            if let Some(cached) = ZIP_CACHE.lock().get(&(handle as u64)) {
                return Ok(cached.clone());
            }
        }

        let zip_data: ClassInstanceRef<Array<i8>> = jvm.get_field(this, "zipData", "[B").await?;
        if zip_data.is_null() {
            return Err(jvm.exception("java/util/zip/ZipException", "empty zip").await);
        }
        let length = jvm.array_length(&zip_data).await?;
        let mut buf = vec![0u8; length];
        if jvm.array_raw_buffer(&zip_data).await?.read(0, &mut buf).is_err() {
            return Err(jvm.exception("java/util/zip/ZipException", "failed to read zip").await);
        }

        let cached = match CachedZip::new(buf) {
            Some(cached) => cached,
            None => return Err(jvm.exception("java/util/zip/ZipException", "invalid zip").await),
        };
        let arc = Arc::new(cached);
        let new_handle = NEXT_ZIP_HANDLE.fetch_add(1, Ordering::Relaxed);
        ZIP_CACHE.lock().insert(new_handle, arc.clone());

        let mut this_mut = this.clone();
        let _ = jvm.put_field(&mut this_mut, "zipHandle", "J", new_handle as i64).await;

        Ok(arc)
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipFile::<init>({:?}, {:?})", &this, &file);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let length: i64 = jvm.invoke_virtual(&file, "length", "()J", ()).await?;
        let is = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;

        let buf = jvm.instantiate_array("B", length.max(0) as _).await?;
        let mut offset = 0i32;
        let total = length.max(0) as i32;
        while offset < total {
            let read: i32 = jvm.invoke_virtual(&is, "read", "([BII)I", (buf.clone(), offset, total - offset)).await?;
            if read <= 0 {
                break;
            }
            offset += read;
        }

        jvm.put_field(&mut this, "zipData", "[B", buf.clone()).await?;

        let mut raw = vec![0u8; length.max(0) as usize];
        if jvm.array_raw_buffer(&buf).await?.read(0, &mut raw).is_ok() {
            if let Some(cached) = CachedZip::new(raw) {
                let handle = NEXT_ZIP_HANDLE.fetch_add(1, Ordering::Relaxed);
                ZIP_CACHE.lock().insert(handle, Arc::new(cached));
                let _ = jvm.put_field(&mut this, "zipHandle", "J", handle as i64).await;
            }
        }

        Ok(())
    }

    async fn get_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<ZipEntry>> {
        tracing::debug!("java.util.zip.ZipFile::getEntry({:?}, {:?})", &this, &name);

        if name.is_null() {
            return Ok(None.into());
        }

        let name_str = JavaLangString::to_rust_string(jvm, &name).await?;
        let cached = Self::get_cached_zip(jvm, &this).await?;
        let Some(index) = cached.find_index(&name_str) else {
            return Ok(None.into());
        };

        let file_size = cached.archive.by_index(index).map(|x| x.size());
        if let Ok(x) = file_size {
            let entry = jvm.new_class("java/util/zip/ZipEntry", "(Ljava/lang/String;)V", (name,)).await?;
            let _: () = jvm.invoke_virtual(&entry, "setSize", "(J)V", (x as i64,)).await?;
            Ok(entry.into())
        } else {
            Ok(None.into())
        }
    }

    async fn entries(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Enumeration>> {
        tracing::debug!("java.util.zip.ZipFile::entries({:?})", &this);

        let cached = Self::get_cached_zip(jvm, &this).await?;
        let names = zip_entry_names(&cached.archive);

        let mut name_array = jvm.instantiate_array("Ljava/lang/String;", names.len() as _).await?;
        for (i, name) in names.iter().enumerate() {
            let name = JavaLangString::from_rust_string(jvm, name).await?;
            jvm.store_array(&mut name_array, i as _, iter::once(name)).await?;
        }

        let entries = jvm
            .new_class(
                "java/util/zip/ZipFile$Entries",
                "(Ljava/util/zip/ZipFile;[Ljava/lang/String;)V",
                (this, name_array),
            )
            .await?;

        Ok(entries.into())
    }

    async fn get_input_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        entry: ClassInstanceRef<ZipEntry>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.util.zip.ZipFile::getInputStream({:?}, {:?})", &this, &entry);

        if entry.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }

        let entry_name: ClassInstanceRef<String> = jvm.invoke_virtual(&entry, "getName", "()Ljava/lang/String;", ()).await?;
        if entry_name.is_null() {
            return Err(jvm.exception("java/util/zip/ZipException", "entry name is null").await);
        }
        let entry_name = JavaLangString::to_rust_string(jvm, &entry_name).await?;

        let cached = Self::get_cached_zip(jvm, &this).await?;
        let Some(index) = cached.find_index(&entry_name) else {
            return Err(jvm.exception("java/util/zip/ZipException", "entry not found").await);
        };

        let data = match cached.extract_entry(index) {
            Some(data) => data,
            None => return Err(jvm.exception("java/io/IOException", "failed to extract zip entry").await),
        };

        let mut java_buf = jvm.instantiate_array("B", data.len() as _).await?;
        if jvm.array_raw_buffer_mut(&mut java_buf).await?.write(0, &data).is_err() {
            return Err(jvm.exception("java/io/IOException", "failed to extract zip entry").await);
        }

        let input_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (java_buf,)).await?;

        Ok(input_stream.into())
    }
}

fn zip_entry_names(zip: &ZipArchive<Vec<u8>>) -> Vec<RustString> {
    zip.entries().iter().map(|e| e.name.clone()).collect()
}

fn normalize_zip_name(name: &str) -> RustString {
    name.trim_start_matches('/').replace('\\', "/")
}

fn is_manifest_name(name: &str) -> bool {
    let name = normalize_zip_name(name);
    let base = name.rsplit('/').next().unwrap_or(&name);
    base.eq_ignore_ascii_case("MANIFEST.MF") || base.eq_ignore_ascii_case("MANIFEST.TXT") || base.eq_ignore_ascii_case("MANIFEST")
}

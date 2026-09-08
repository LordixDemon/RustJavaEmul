mod zip_entry;
mod zip_file;
mod zip_file_entries;

pub use {zip_entry::ZipEntry, zip_file::ZipFile, zip_file_entries::ZipFileEntries};

simple_exception!(
    ZipException,
    "java/util/zip/ZipException",
    "java/io/IOException",
    "java.util.zip.ZipException"
);

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![ZipEntry, ZipFile, ZipFileEntries, ZipException]
}

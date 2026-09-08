pub struct RecordStore;
pub struct RecordEnumeration;
pub struct RecordFilter;
pub struct RecordComparator;
pub struct RecordListener;

mod enumeration;
mod exceptions;
mod record_store;

pub use self::exceptions::*;

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        InvalidRecordIDException,
        RecordComparator,
        RecordEnumeration,
        RecordFilter,
        RecordListener,
        RecordStore,
        RecordStoreException,
        RecordStoreFullException,
        RecordStoreNotFoundException,
        RecordStoreNotOpenException,
    ]
}

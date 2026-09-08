#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use alloc::{string::String as RustString, vec, vec::Vec};

simple_exception!(
    RecordStoreException,
    "javax/microedition/rms/RecordStoreException",
    "java/lang/Exception",
    "javax.microedition.rms.RecordStoreException"
);
simple_exception!(
    RecordStoreNotFoundException,
    "javax/microedition/rms/RecordStoreNotFoundException",
    "javax/microedition/rms/RecordStoreException",
    "javax.microedition.rms.RecordStoreNotFoundException"
);
simple_exception!(
    RecordStoreFullException,
    "javax/microedition/rms/RecordStoreFullException",
    "javax/microedition/rms/RecordStoreException",
    "javax.microedition.rms.RecordStoreFullException"
);
simple_exception!(
    RecordStoreNotOpenException,
    "javax/microedition/rms/RecordStoreNotOpenException",
    "javax/microedition/rms/RecordStoreException",
    "javax.microedition.rms.RecordStoreNotOpenException"
);
simple_exception!(
    InvalidRecordIDException,
    "javax/microedition/rms/InvalidRecordIDException",
    "javax/microedition/rms/RecordStoreException",
    "javax.microedition.rms.InvalidRecordIDException"
);

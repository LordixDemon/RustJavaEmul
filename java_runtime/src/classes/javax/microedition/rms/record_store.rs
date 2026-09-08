#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::{string::String as RustString, vec, vec::Vec};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

impl RecordStore {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/rms/RecordStore",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "openRecordStore",
                    "(Ljava/lang/String;Z)Ljavax/microedition/rms/RecordStore;",
                    Self::open_record_store,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "openRecordStore",
                    "(Ljava/lang/String;ZIZ)Ljavax/microedition/rms/RecordStore;",
                    Self::open_record_store_with_auth,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("closeRecordStore", "()V", Self::close_record_store, Default::default()),
                JavaMethodProto::new("getNumRecords", "()I", Self::get_num_records, Default::default()),
                JavaMethodProto::new("getNextRecordID", "()I", Self::get_next_record_id, Default::default()),
                JavaMethodProto::new("addRecord", "([BII)I", Self::add_record, Default::default()),
                JavaMethodProto::new("deleteRecord", "(I)V", Self::delete_record, Default::default()),
                JavaMethodProto::new(
                    "deleteRecordStore",
                    "(Ljava/lang/String;)V",
                    Self::delete_record_store,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "enumerateRecords",
                    "(Ljavax/microedition/rms/RecordFilter;Ljavax/microedition/rms/RecordComparator;Z)Ljavax/microedition/rms/RecordEnumeration;",
                    Self::enumerate_records,
                    Default::default(),
                ),
                JavaMethodProto::new("getLastModified", "()J", Self::get_last_modified, Default::default()),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new("getRecord", "(I)[B", Self::get_record_array, Default::default()),
                JavaMethodProto::new("getRecordSize", "(I)I", Self::get_record_size, Default::default()),
                JavaMethodProto::new("setRecord", "(I[BII)V", Self::set_record, Default::default()),
                JavaMethodProto::new("getRecord", "(I[BI)I", Self::get_record, Default::default()),
                JavaMethodProto::new("getVersion", "()I", Self::get_version, Default::default()),
                JavaMethodProto::new("getSize", "()I", Self::get_size, Default::default()),
                JavaMethodProto::new("getSizeAvailable", "()I", Self::get_size_available, Default::default()),
                JavaMethodProto::new(
                    "addRecordListener",
                    "(Ljavax/microedition/rms/RecordListener;)V",
                    Self::add_record_listener,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "removeRecordListener",
                    "(Ljavax/microedition/rms/RecordListener;)V",
                    Self::remove_record_listener,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "listRecordStores",
                    "()[Ljava/lang/String;",
                    Self::list_record_stores,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "openRecordStore",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljavax/microedition/rms/RecordStore;",
                    Self::open_record_store_named,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("setMode", "(IZ)V", Self::set_mode, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("name", "Ljava/lang/String;", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("javax.microedition.rms.RecordStore::<init>({this:?}, {name:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "name", "Ljava/lang/String;", name).await?;

        Ok(())
    }

    pub(super) async fn open_record_store(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
        create_if_necessary: bool,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.rms.RecordStore::openRecordStore({name:?}, {create_if_necessary:?})");

        let name_str = JavaLangString::to_rust_string(jvm, &name).await?;
        if !context.rms_open_record_store(&name_str, create_if_necessary) {
            return Err(jvm
                .exception("javax/microedition/rms/RecordStoreNotFoundException", "record store not found")
                .await);
        }

        Ok(jvm
            .new_class("javax/microedition/rms/RecordStore", "(Ljava/lang/String;)V", (name,))
            .await?
            .into())
    }

    pub(super) async fn open_record_store_with_auth(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
        create_if_necessary: bool,
        _authmode: i32,
        _writable: bool,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.rms.RecordStore::openRecordStore({name:?}, {create_if_necessary:?}, auth)");
        Self::open_record_store(jvm, context, name, create_if_necessary).await
    }

    pub(super) async fn open_record_store_named(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
        _vendor: ClassInstanceRef<String>,
        _suite: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::open_record_store(jvm, context, name, true).await
    }

    pub(super) async fn set_mode(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _authmode: i32, _writable: bool) -> Result<()> {
        Ok(())
    }

    pub(super) async fn close_record_store(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.rms.RecordStore::closeRecordStore({this:?})");

        Ok(())
    }

    pub(super) async fn get_num_records(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.rms.RecordStore::getNumRecords({this:?})");

        let name = Self::store_name(jvm, &this).await?;
        Ok(context.rms_num_records(&name))
    }

    pub(super) async fn get_next_record_id(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.rms.RecordStore::getNextRecordID({this:?})");

        let name = Self::store_name(jvm, &this).await?;
        Ok(context.rms_next_record_id(&name))
    }

    pub(super) async fn add_record(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
        offset: i32,
        num_bytes: i32,
    ) -> Result<i32> {
        tracing::debug!("javax.microedition.rms.RecordStore::addRecord({this:?}, {data:?}, {offset:?}, {num_bytes:?})");

        let record = Self::copy_record(jvm, data, offset, num_bytes).await?;
        let name = Self::store_name(jvm, &this).await?;

        Ok(context.rms_add_record(&name, &record))
    }

    pub(super) async fn delete_record(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, record_id: i32) -> Result<()> {
        tracing::debug!("javax.microedition.rms.RecordStore::deleteRecord({this:?}, {record_id:?})");

        if record_id <= 0 {
            return Err(jvm
                .exception("javax/microedition/rms/InvalidRecordIDException", "recordId must be >= 1")
                .await);
        }

        let name = Self::store_name(jvm, &this).await?;
        context.rms_delete_record(&name, record_id);

        Ok(())
    }

    pub(super) async fn delete_record_store(jvm: &Jvm, context: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<()> {
        let name_str = JavaLangString::to_rust_string(jvm, &name).await?;
        let next_id = context.rms_next_record_id(&name_str);
        for record_id in 1..next_id {
            context.rms_delete_record(&name_str, record_id);
        }
        Ok(())
    }

    pub(super) async fn enumerate_records(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        filter: ClassInstanceRef<RecordFilter>,
        _comparator: ClassInstanceRef<RecordComparator>,
        keep_updated: bool,
    ) -> Result<ClassInstanceRef<RecordEnumeration>> {
        tracing::debug!("javax.microedition.rms.RecordStore::enumerateRecords({this:?}, {filter:?}, {keep_updated:?})");

        let name = Self::store_name(jvm, &this).await?;
        let mut ids = Vec::new();
        for record_id in 1..context.rms_next_record_id(&name) {
            let Some(data) = context.rms_get_record(&name, record_id) else {
                continue;
            };
            if !filter.is_null() {
                let mut array = jvm.instantiate_array("B", data.len()).await?;
                jvm.store_array(&mut array, 0, data.clone()).await?;
                let accepted: bool = jvm.invoke_virtual(&filter, "matches", "([B)Z", (array,)).await?;
                if !accepted {
                    continue;
                }
            }
            ids.push(record_id);
        }

        Ok(jvm
            .new_class(
                "javax/microedition/rms/RecordEnumeration",
                "(Ljavax/microedition/rms/RecordStore;[IZ)V",
                (this, Self::int_array(jvm, ids).await?, keep_updated),
            )
            .await?
            .into())
    }

    pub(super) async fn get_last_modified(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::trace!("javax.microedition.rms.RecordStore::getLastModified({this:?})");
        Ok(context.now() as i64)
    }

    pub(super) async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "name", "Ljava/lang/String;").await
    }

    pub(super) async fn set_record(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        record_id: i32,
        new_data: ClassInstanceRef<Array<i8>>,
        offset: i32,
        num_bytes: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.rms.RecordStore::setRecord({this:?}, {record_id:?}, {new_data:?}, {offset:?}, {num_bytes:?})");

        if record_id <= 0 {
            return Err(jvm
                .exception("javax/microedition/rms/InvalidRecordIDException", "recordId must be >= 1")
                .await);
        }

        let record = Self::copy_record(jvm, new_data, offset, num_bytes).await?;
        let name = Self::store_name(jvm, &this).await?;
        context.rms_set_record(&name, record_id, &record);

        Ok(())
    }

    pub(super) async fn get_record_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        record_id: i32,
    ) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("javax.microedition.rms.RecordStore::getRecord({this:?}, {record_id:?})");

        if record_id <= 0 {
            return Err(jvm
                .exception("javax/microedition/rms/InvalidRecordIDException", "recordId must be >= 1")
                .await);
        }

        let name = Self::store_name(jvm, &this).await?;
        let Some(bytes) = context.rms_get_record(&name, record_id) else {
            return Err(jvm.exception("javax/microedition/rms/InvalidRecordIDException", "record not found").await);
        };
        let mut copy = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_array(&mut copy, 0, bytes).await?;

        Ok(copy.into())
    }

    pub(super) async fn get_record(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        record_id: i32,
        mut buffer: ClassInstanceRef<Array<i8>>,
        offset: i32,
    ) -> Result<i32> {
        tracing::debug!("javax.microedition.rms.RecordStore::getRecord({this:?}, {record_id:?}, {buffer:?}, {offset:?})");

        if record_id <= 0 {
            return Err(jvm
                .exception("javax/microedition/rms/InvalidRecordIDException", "recordId must be >= 1")
                .await);
        }

        let name = Self::store_name(jvm, &this).await?;
        let Some(data) = context.rms_get_record(&name, record_id) else {
            return Err(jvm.exception("javax/microedition/rms/InvalidRecordIDException", "record not found").await);
        };
        let length = data.len();
        jvm.store_array(&mut buffer, offset as usize, data).await?;

        Ok(length as i32)
    }

    pub(super) async fn get_record_size(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, record_id: i32) -> Result<i32> {
        if record_id <= 0 {
            return Err(jvm
                .exception("javax/microedition/rms/InvalidRecordIDException", "recordId must be >= 1")
                .await);
        }

        let name = Self::store_name(jvm, &this).await?;
        let Some(data) = context.rms_get_record(&name, record_id) else {
            return Err(jvm.exception("javax/microedition/rms/InvalidRecordIDException", "record not found").await);
        };
        Ok(data.len() as i32)
    }

    pub(super) async fn get_size(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let name = Self::store_name(jvm, &this).await?;
        Ok(context.rms_get_size(&name) as i32)
    }

    pub(super) async fn get_size_available(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.rms.RecordStore::getSizeAvailable({this:?})");
        let name = Self::store_name(jvm, &this).await?;
        Ok(context.rms_get_size_available(&name))
    }

    pub(super) async fn add_record_listener(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<RecordListener>,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.rms.RecordStore::addRecordListener({this:?}, {listener:?})");
        Ok(())
    }

    pub(super) async fn remove_record_listener(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<RecordListener>,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.rms.RecordStore::removeRecordListener({this:?}, {listener:?})");
        Ok(())
    }

    pub(super) async fn get_version(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.rms.RecordStore::getVersion({this:?})");
        Ok(1)
    }

    pub(super) async fn list_record_stores(jvm: &Jvm, context: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<ClassInstanceRef<String>>>> {
        let names = context.rms_list_record_stores();
        let mut array = jvm.instantiate_array("Ljava/lang/String;", names.len()).await?;
        let values = names.iter().map(|name| JavaLangString::from_rust_string(jvm, name)).collect::<Vec<_>>();
        let mut strings = Vec::with_capacity(values.len());
        for value in values {
            strings.push(value.await?);
        }
        jvm.store_array(&mut array, 0, strings).await?;
        Ok(array.into())
    }

    pub(super) async fn copy_record(jvm: &Jvm, data: ClassInstanceRef<Array<i8>>, offset: i32, num_bytes: i32) -> Result<Vec<i8>> {
        if offset < 0 || num_bytes < 0 {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid record range").await);
        }

        let length = num_bytes as usize;
        if data.is_null() {
            if length == 0 {
                return Ok(Vec::new());
            }
            return Err(jvm.exception("java/lang/NullPointerException", "record data").await);
        }

        jvm.load_array(&data, offset as usize, length).await
    }

    pub(super) async fn store_name(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<RustString> {
        let name: ClassInstanceRef<String> = jvm.get_field(this, "name", "Ljava/lang/String;").await?;
        JavaLangString::to_rust_string(jvm, &name).await
    }

    pub(super) async fn int_array(jvm: &Jvm, values: Vec<i32>) -> Result<ClassInstanceRef<Array<i32>>> {
        let mut array = jvm.instantiate_array("I", values.len()).await?;
        jvm.store_array(&mut array, 0, values).await?;
        Ok(array.into())
    }
}

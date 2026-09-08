#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext};
#[allow(unused_imports)]
use alloc::{string::String as RustString, vec, vec::Vec};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl RecordListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/rms/RecordListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("recordAdded", "(Ljavax/microedition/rms/RecordStore;I)V", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("recordChanged", "(Ljavax/microedition/rms/RecordStore;I)V", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("recordDeleted", "(Ljavax/microedition/rms/RecordStore;I)V", MethodAccessFlags::ABSTRACT),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl RecordEnumeration {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/rms/RecordEnumeration",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljavax/microedition/rms/RecordStore;[IZ)V", Self::init, Default::default()),
                JavaMethodProto::new("destroy", "()V", Self::destroy, Default::default()),
                JavaMethodProto::new("hasNextElement", "()Z", Self::has_next_element, Default::default()),
                JavaMethodProto::new("hasPreviousElement", "()Z", Self::has_previous_element, Default::default()),
                JavaMethodProto::new("isKeptUpdated", "()Z", Self::is_kept_updated, Default::default()),
                JavaMethodProto::new("keepUpdated", "(Z)V", Self::keep_updated, Default::default()),
                JavaMethodProto::new("nextRecord", "()[B", Self::next_record, Default::default()),
                JavaMethodProto::new("nextRecordId", "()I", Self::next_record_id, Default::default()),
                JavaMethodProto::new("numRecords", "()I", Self::num_records, Default::default()),
                JavaMethodProto::new("previousRecord", "()[B", Self::previous_record, Default::default()),
                JavaMethodProto::new("previousRecordId", "()I", Self::previous_record_id, Default::default()),
                JavaMethodProto::new("rebuild", "()V", Self::rebuild, Default::default()),
                JavaMethodProto::new("reset", "()V", Self::reset, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("store", "Ljavax/microedition/rms/RecordStore;", Default::default()),
                JavaFieldProto::new("ids", "[I", Default::default()),
                JavaFieldProto::new("position", "I", Default::default()),
                JavaFieldProto::new("keepUpdated", "Z", Default::default()),
                JavaFieldProto::new("destroyed", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        store: ClassInstanceRef<RecordStore>,
        ids: ClassInstanceRef<Array<i32>>,
        keep_updated: bool,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "store", "Ljavax/microedition/rms/RecordStore;", store).await?;
        jvm.put_field(&mut this, "ids", "[I", ids).await?;
        jvm.put_field(&mut this, "position", "I", 0).await?;
        jvm.put_field(&mut this, "keepUpdated", "Z", keep_updated).await?;
        jvm.put_field(&mut this, "destroyed", "Z", false).await
    }

    pub(super) async fn destroy(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "destroyed", "Z", true).await
    }

    pub(super) async fn has_next_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let position: i32 = jvm.get_field(&this, "position", "I").await?;
        Ok(position < Self::len(jvm, &this).await? as i32)
    }

    pub(super) async fn has_previous_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let position: i32 = jvm.get_field(&this, "position", "I").await?;
        Ok(position > 0)
    }

    pub(super) async fn is_kept_updated(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "keepUpdated", "Z").await
    }

    pub(super) async fn keep_updated(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, keep_updated: bool) -> Result<()> {
        jvm.put_field(&mut this, "keepUpdated", "Z", keep_updated).await
    }

    pub(super) async fn next_record(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        let record_id = Self::next_record_id(jvm, context, this.clone()).await?;
        Self::record_bytes(jvm, context, &this, record_id).await
    }

    pub(super) async fn next_record_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        let position: i32 = jvm.get_field(&this, "position", "I").await?;
        let ids = Self::ids(jvm, &this).await?;
        if position < 0 || position as usize >= ids.len() {
            return Ok(-1);
        }
        jvm.put_field(&mut this, "position", "I", position + 1).await?;
        Ok(ids[position as usize])
    }

    pub(super) async fn num_records(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(Self::len(jvm, &this).await? as i32)
    }

    pub(super) async fn previous_record(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Array<i8>>> {
        let record_id = Self::previous_record_id(jvm, context, this.clone()).await?;
        Self::record_bytes(jvm, context, &this, record_id).await
    }

    pub(super) async fn previous_record_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        let position: i32 = jvm.get_field(&this, "position", "I").await?;
        let ids = Self::ids(jvm, &this).await?;
        if position <= 0 || ids.is_empty() {
            return Ok(-1);
        }
        let new_position = position - 1;
        jvm.put_field(&mut this, "position", "I", new_position).await?;
        Ok(ids[new_position as usize])
    }

    pub(super) async fn rebuild(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    pub(super) async fn reset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "position", "I", 0).await
    }

    pub(super) async fn ids(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<i32>> {
        let ids: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "ids", "[I").await?;
        let len = jvm.array_length(&ids).await?;
        jvm.load_array(&ids, 0, len).await
    }

    pub(super) async fn len(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<usize> {
        let ids: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "ids", "[I").await?;
        jvm.array_length(&ids).await
    }

    pub(super) async fn record_bytes(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        record_id: i32,
    ) -> Result<ClassInstanceRef<Array<i8>>> {
        let store: ClassInstanceRef<RecordStore> = jvm.get_field(this, "store", "Ljavax/microedition/rms/RecordStore;").await?;
        let name = RecordStore::store_name(jvm, &store).await?;
        let data = context.rms_get_record(&name, record_id).unwrap_or_default();
        let mut array = jvm.instantiate_array("B", data.len()).await?;
        jvm.store_array(&mut array, 0, data).await?;
        Ok(array.into())
    }
}

impl RecordFilter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/rms/RecordFilter",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new_abstract("matches", "([B)Z", MethodAccessFlags::ABSTRACT)],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl RecordComparator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/rms/RecordComparator",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new_abstract("compare", "([B[B)I", MethodAccessFlags::ABSTRACT)],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

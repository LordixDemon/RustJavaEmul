use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::HashtableEntry;

const DEFAULT_INITIAL_CAPACITY: i32 = 11;
const DEFAULT_LOAD_FACTOR: f32 = 0.75;

// class java.util.Hashtable
pub struct Hashtable;

impl Hashtable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Hashtable",
            parent_class: Some("java/util/Dictionary"),
            interfaces: vec!["java/lang/Cloneable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_capacity, Default::default()),
                JavaMethodProto::new("clear", "()V", Self::clear, Default::default()),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, Default::default()),
                JavaMethodProto::new("containsKey", "(Ljava/lang/Object;)Z", Self::contains_key, Default::default()),
                JavaMethodProto::new(
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    Self::put,
                    Default::default(),
                ),
                JavaMethodProto::new("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::get, Default::default()),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::remove, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("keys", "()Ljava/util/Enumeration;", Self::keys, Default::default()),
                JavaMethodProto::new("elements", "()Ljava/util/Enumeration;", Self::elements, Default::default()),
                JavaMethodProto::new("values", "()Ljava/util/Collection;", Self::values, Default::default()),
                JavaMethodProto::new("rehash", "()V", Self::rehash_public, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("table", "[Ljava/util/Hashtable$Entry;", Default::default()),
                JavaFieldProto::new("count", "I", Default::default()),
                JavaFieldProto::new("threshold", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/Dictionary", "<init>", "()V", ()).await?;

        let table = jvm
            .instantiate_array("Ljava/util/Hashtable$Entry;", DEFAULT_INITIAL_CAPACITY as _)
            .await?;
        jvm.put_field(&mut this, "table", "[Ljava/util/Hashtable$Entry;", table).await?;
        jvm.put_field(&mut this, "count", "I", 0).await?;
        jvm.put_field(
            &mut this,
            "threshold",
            "I",
            (DEFAULT_INITIAL_CAPACITY as f32 * DEFAULT_LOAD_FACTOR) as i32,
        )
        .await?;

        Ok(())
    }

    async fn init_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.util.Hashtable::<init>({this:?}, {capacity:?})");
        let _: () = jvm.invoke_special(&this, "java/util/Dictionary", "<init>", "()V", ()).await?;
        let capacity = capacity.max(1);
        let table = jvm.instantiate_array("Ljava/util/Hashtable$Entry;", capacity as _).await?;
        jvm.put_field(&mut this, "table", "[Ljava/util/Hashtable$Entry;", table).await?;
        jvm.put_field(&mut this, "count", "I", 0).await?;
        jvm.put_field(&mut this, "threshold", "I", (capacity as f32 * DEFAULT_LOAD_FACTOR) as i32)
            .await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "count", "I").await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(jvm.get_field::<i32>(&this, "count", "I").await? == 0)
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let capacity = jvm.array_length(&table).await?;
        for i in 0..capacity {
            let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, i, 1).await?.into_iter().next().unwrap();
            while !entry.is_null() {
                let entry_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
                if value.is_null() && entry_value.is_null() {
                    return Ok(true);
                }
                if !value.is_null() && !entry_value.is_null() {
                    let equals: bool = jvm
                        .invoke_virtual(&entry_value, "equals", "(Ljava/lang/Object;)Z", (value.clone(),))
                        .await?;
                    if equals {
                        return Ok(true);
                    }
                }
                entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
            }
        }
        Ok(false)
    }

    async fn keys(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/Hashtable$Enumerator", "(Ljava/util/Hashtable;Z)V", (this, true))
            .await?
            .into())
    }

    async fn elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/Hashtable$Enumerator", "(Ljava/util/Hashtable;Z)V", (this, false))
            .await?
            .into())
    }

    async fn values(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
        let elements: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "elements", "()Ljava/util/Enumeration;", ()).await?;
        while jvm.invoke_virtual(&elements, "hasMoreElements", "()Z", ()).await? {
            let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&elements, "nextElement", "()Ljava/lang/Object;", ()).await?;
            let _: () = jvm.invoke_virtual(&vector, "addElement", "(Ljava/lang/Object;)V", (value,)).await?;
        }
        Ok(vector.into())
    }

    async fn rehash_public(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        Self::rehash(jvm, &mut this).await
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable::clear({this:?})");

        let table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let capacity = jvm.array_length(&table).await?;
        let new_table = jvm.instantiate_array("Ljava/util/Hashtable$Entry;", capacity).await?;
        jvm.put_field(&mut this, "table", "[Ljava/util/Hashtable$Entry;", new_table).await?;
        jvm.put_field(&mut this, "count", "I", 0).await
    }

    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable::containsKey({this:?}, {key:?})");

        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;
        let table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let bucket_index = ((key_hash & 0x7FFFFFFF) % table_len) as usize;

        let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                let equals: bool = jvm.invoke_virtual(&entry_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    return Ok(true);
                }
            }
            entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
        }

        Ok(false)
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        let key_str = JavaLangString::to_rust_string(jvm, &key).await.unwrap_or_default();
        if key_str.starts_with("JAD_") || key_str.starts_with("CFG_") {
            tracing::info!(target: "rustjava_hash", "Hashtable::get key='{key_str}'");
        }
        tracing::debug!("java.util.Hashtable::get({this:?}, {key:?})");

        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;
        let table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let bucket_index = ((key_hash & 0x7FFFFFFF) % table_len) as usize;

        let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                let equals: bool = jvm.invoke_virtual(&entry_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    return jvm.get_field(&entry, "value", "Ljava/lang/Object;").await;
                }
            }
            entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
        }

        Ok(None.into())
    }

    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::remove({this:?}, {key:?})");

        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;
        let mut table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let bucket_index = ((key_hash & 0x7FFFFFFF) % table_len) as usize;

        let mut prev: ClassInstanceRef<HashtableEntry> = None.into();
        let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();

        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                let equals: bool = jvm.invoke_virtual(&entry_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    let next: ClassInstanceRef<HashtableEntry> = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
                    if prev.is_null() {
                        jvm.store_array(&mut table, bucket_index, core::iter::once(next)).await?;
                    } else {
                        jvm.put_field(&mut prev, "next", "Ljava/util/Hashtable$Entry;", next).await?;
                    }

                    let count: i32 = jvm.get_field(&this, "count", "I").await?;
                    jvm.put_field(&mut this, "count", "I", count - 1).await?;

                    return jvm.get_field(&entry, "value", "Ljava/lang/Object;").await;
                }
            }
            prev = entry;
            entry = jvm.get_field(&prev, "next", "Ljava/util/Hashtable$Entry;").await?;
        }

        Ok(None.into())
    }

    async fn put(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let key_str = JavaLangString::to_rust_string(jvm, &key).await.unwrap_or_default();
        if key_str.starts_with("JAD_") || key_str.starts_with("CFG_") {
            tracing::info!(target: "rustjava_hash", "Hashtable::put key='{key_str}'");
        }
        tracing::debug!("java.util.Hashtable::put({this:?}, {key:?}, {value:?})");

        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;
        let mut table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let bucket_index = ((key_hash & 0x7FFFFFFF) % table_len) as usize;

        let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                let equals: bool = jvm.invoke_virtual(&entry_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    let old_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
                    jvm.put_field(&mut entry, "value", "Ljava/lang/Object;", value).await?;
                    return Ok(old_value);
                }
            }
            entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
        }

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let threshold: i32 = jvm.get_field(&this, "threshold", "I").await?;

        if count >= threshold {
            Self::rehash(jvm, &mut this).await?;
            table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
            let new_table_len = jvm.array_length(&table).await? as i32;
            let new_bucket_index = ((key_hash & 0x7FFFFFFF) % new_table_len) as usize;

            let existing: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, new_bucket_index, 1).await?.into_iter().next().unwrap();
            let new_entry = jvm
                .new_class(
                    "java/util/Hashtable$Entry",
                    "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/Hashtable$Entry;)V",
                    (key_hash, key, value, existing),
                )
                .await?;
            jvm.store_array(&mut table, new_bucket_index, core::iter::once(new_entry)).await?;
        } else {
            let existing: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();
            let new_entry = jvm
                .new_class(
                    "java/util/Hashtable$Entry",
                    "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/Hashtable$Entry;)V",
                    (key_hash, key, value, existing),
                )
                .await?;
            jvm.store_array(&mut table, bucket_index, core::iter::once(new_entry)).await?;
        }

        jvm.put_field(&mut this, "count", "I", count + 1).await?;

        Ok(None.into())
    }

    async fn rehash(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let old_table = jvm.get_field(this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let old_capacity = jvm.array_length(&old_table).await?;
        let new_capacity = old_capacity * 2 + 1;

        let mut new_table = jvm.instantiate_array("Ljava/util/Hashtable$Entry;", new_capacity).await?;

        for i in 0..old_capacity {
            let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&old_table, i, 1).await?.into_iter().next().unwrap();
            while !entry.is_null() {
                let next: ClassInstanceRef<HashtableEntry> = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
                let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
                let new_index = ((entry_hash & 0x7FFFFFFF) % new_capacity as i32) as usize;

                let existing: ClassInstanceRef<HashtableEntry> = jvm.load_array(&new_table, new_index, 1).await?.into_iter().next().unwrap();
                jvm.put_field(&mut entry, "next", "Ljava/util/Hashtable$Entry;", existing).await?;
                jvm.store_array(&mut new_table, new_index, core::iter::once(entry)).await?;

                entry = next;
            }
        }

        jvm.put_field(this, "table", "[Ljava/util/Hashtable$Entry;", new_table).await?;
        jvm.put_field(this, "threshold", "I", (new_capacity as f32 * DEFAULT_LOAD_FACTOR) as i32)
            .await?;

        Ok(())
    }
}

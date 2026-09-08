use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Hashtable$Enumerator
pub struct HashtableEnumerator;

impl HashtableEnumerator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Hashtable$Enumerator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Enumeration"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Hashtable;Z)V", Self::init, Default::default()),
                JavaMethodProto::new("hasMoreElements", "()Z", Self::has_more_elements, Default::default()),
                JavaMethodProto::new("nextElement", "()Ljava/lang/Object;", Self::next_element, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("table", "[Ljava/util/Hashtable$Entry;", Default::default()),
                JavaFieldProto::new("index", "I", Default::default()),
                JavaFieldProto::new("entry", "Ljava/util/Hashtable$Entry;", Default::default()),
                JavaFieldProto::new("keys", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        hashtable: ClassInstanceRef<Object>,
        keys: bool,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let table = jvm.get_field(&hashtable, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let length = jvm.array_length(&table).await? as i32;
        jvm.put_field(&mut this, "table", "[Ljava/util/Hashtable$Entry;", table).await?;
        jvm.put_field(&mut this, "index", "I", length).await?;
        jvm.put_field(&mut this, "entry", "Ljava/util/Hashtable$Entry;", ClassInstanceRef::<Object>::new(None))
            .await?;
        jvm.put_field(&mut this, "keys", "Z", keys).await
    }

    async fn has_more_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(Self::next_entry(jvm, this, false).await?.is_some())
    }

    async fn next_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        match Self::next_entry(jvm, this, true).await? {
            Some(value) => Ok(value),
            None => Err(jvm.exception("java/util/NoSuchElementException", "Hashtable Enumerator").await),
        }
    }

    async fn next_entry(jvm: &Jvm, mut this: ClassInstanceRef<Self>, consume: bool) -> Result<Option<ClassInstanceRef<Object>>> {
        let mut entry: ClassInstanceRef<Object> = jvm.get_field(&this, "entry", "Ljava/util/Hashtable$Entry;").await?;
        let table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let mut index: i32 = jvm.get_field(&this, "index", "I").await?;
        while entry.is_null() && index > 0 {
            index -= 1;
            entry = jvm.load_array(&table, index as usize, 1).await?.into_iter().next().unwrap();
        }
        if entry.is_null() {
            return Ok(None);
        }
        if consume {
            let next: ClassInstanceRef<Object> = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
            jvm.put_field(&mut this, "entry", "Ljava/util/Hashtable$Entry;", next).await?;
            jvm.put_field(&mut this, "index", "I", index).await?;
            let keys: bool = jvm.get_field(&this, "keys", "Z").await?;
            let field = if keys { "key" } else { "value" };
            let value: ClassInstanceRef<Object> = jvm.get_field(&entry, field, "Ljava/lang/Object;").await?;
            return Ok(Some(value));
        }
        Ok(Some(entry))
    }
}

// class java.util.Vector$Enumerator
pub struct VectorEnumerator;

impl VectorEnumerator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Vector$Enumerator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Enumeration"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Vector;)V", Self::init, Default::default()),
                JavaMethodProto::new("hasMoreElements", "()Z", Self::has_more_elements, Default::default()),
                JavaMethodProto::new("nextElement", "()Ljava/lang/Object;", Self::next_element, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("vector", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("count", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, vector: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "vector", "Ljava/util/Vector;", vector).await?;
        jvm.put_field(&mut this, "count", "I", 0).await
    }

    async fn has_more_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let vector: ClassInstanceRef<Object> = jvm.get_field(&this, "vector", "Ljava/util/Vector;").await?;
        let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Ok(count < size)
    }

    async fn next_element(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let vector: ClassInstanceRef<Object> = jvm.get_field(&this, "vector", "Ljava/util/Vector;").await?;
        let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if count >= size {
            return Err(jvm.exception("java/util/NoSuchElementException", "Vector Enumerator").await);
        }
        jvm.put_field(&mut this, "count", "I", count + 1).await?;
        jvm.invoke_virtual(&vector, "elementAt", "(I)Ljava/lang/Object;", (count,)).await
    }
}

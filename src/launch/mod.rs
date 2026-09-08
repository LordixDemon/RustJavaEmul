use std::path::Path;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

mod manifest;
mod screen;

pub(crate) use manifest::{read_jar_manifest, read_jar_manifest_bytes};
pub(crate) use screen::screen_size_for_start_type;

pub enum StartType<'a> {
    Jar(&'a Path),
    JarBytes { name: &'a Path, bytes: &'a [u8] },
    Class(&'a Path),
}

pub(crate) enum EntryPoint {
    MainClass(String),
    Midlet(String),
}

async fn system_property_string(jvm: &Jvm, key: &str) -> Result<Option<String>> {
    let key = JavaLangString::from_rust_string(jvm, key).await?;
    let value: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
        .await?;
    if value.is_null() {
        Ok(None)
    } else {
        Ok(Some(JavaLangString::to_rust_string(jvm, &value).await?))
    }
}

pub(crate) async fn get_jar_entry_point(jvm: &Jvm, jar_path: &Path) -> Result<EntryPoint> {
    if let Some(main_class) = system_property_string(jvm, "Main-Class").await? {
        return Ok(EntryPoint::MainClass(main_class));
    }
    if let Some(midlet) = system_property_string(jvm, "MIDlet-1").await? {
        if let Some(class_name) = manifest::parse_midlet_1(&midlet) {
            return Ok(EntryPoint::Midlet(class_name));
        }
    }

    let filename = JavaLangString::from_rust_string(jvm, &jar_path.to_string_lossy()).await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (filename,)).await?;
    let jar_file = jvm.new_class("java/util/jar/JarFile", "(Ljava/io/File;)V", (file,)).await?;

    let manifest = jvm.invoke_virtual(&jar_file, "getManifest", "()Ljava/util/jar/Manifest;", ()).await?;
    let attributes = jvm
        .invoke_virtual(&manifest, "getMainAttributes", "()Ljava/util/jar/Attributes;", ())
        .await?;

    if let Some(main_class) = manifest_value(jvm, &attributes, "Main-Class").await? {
        return Ok(EntryPoint::MainClass(main_class));
    }

    if let Some(midlet) = manifest_value(jvm, &attributes, "MIDlet-1").await? {
        let Some(class_name) = manifest::parse_midlet_1(&midlet) else {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "Malformed MIDlet-1 manifest entry")
                .await);
        };

        return Ok(EntryPoint::Midlet(class_name));
    }

    if let Ok(values) = read_jar_manifest(jar_path) {
        if let Some(entry) = manifest::entry_point_from_manifest(&values) {
            return Ok(entry);
        }
    }

    Err(jvm
        .exception("java/lang/IllegalArgumentException", "Jar manifest has neither Main-Class nor MIDlet-1")
        .await)
}

async fn manifest_value(
    jvm: &Jvm,
    attributes: &ClassInstanceRef<java_runtime::classes::java::util::jar::Attributes>,
    name: &str,
) -> Result<Option<String>> {
    let value: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(
            attributes,
            "getValue",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (JavaLangString::from_rust_string(jvm, name).await?,),
        )
        .await?;

    if value.is_null() {
        Ok(None)
    } else {
        Ok(Some(JavaLangString::to_rust_string(jvm, &value).await?))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::manifest::{entry_point_from_manifest, parse_manifest, parse_midlet_1};
    use super::*;

    #[test]
    fn parse_manifest_skips_hack_lines_without_colon() {
        let values = parse_manifest("MIDlet-Name: Foo\n{Hack by Alexscorp}\nMIDlet-1: a,b,Game\n").unwrap();
        assert_eq!(values.get("MIDlet-Name").map(String::as_str), Some("Foo"));
        assert_eq!(values.get("MIDlet-1").map(String::as_str), Some("a,b,Game"));
    }

    #[test]
    fn reads_manifest_txt_when_mf_is_missing() {
        let zip_bytes = java_runtime::zip::create_simple_zip(&[("META-INF/MANIFEST.TXT", b"MIDlet-1: Clock, /icon.png, app.Clock\n")]);
        let values = read_jar_manifest_bytes(&zip_bytes).unwrap();
        assert_eq!(values.get("MIDlet-1").map(String::as_str), Some("Clock, /icon.png, app.Clock"));
    }

    #[test]
    fn parse_midlet_1_reads_class_name() {
        assert_eq!(parse_midlet_1("Clock, /res/icon.png, app.Clock").as_deref(), Some("app.Clock"));
        assert_eq!(parse_midlet_1(",,M").as_deref(), Some("M"));
        assert_eq!(parse_midlet_1("Clock, /icon.png"), None);
    }

    #[test]
    fn inferred_midlet_is_entry_point() {
        let mut values = BTreeMap::new();
        values.insert("MIDlet-1".to_string(), ",,game/Main".to_string());
        match entry_point_from_manifest(&values) {
            Some(EntryPoint::Midlet(name)) => assert_eq!(name, "game/Main"),
            Some(EntryPoint::MainClass(_)) => panic!("unexpected main class"),
            None => panic!("missing entry point"),
        }
    }
}

extern crate alloc;

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub mod browser;
mod compat;
mod config;
#[cfg(any(feature = "desktop-window", feature = "browser-window"))]
mod gpu;
mod launch;
mod profile;
pub(crate) mod runtime;

use std::{collections::BTreeMap, io::Write, path::Path};

use java_runtime::{DeviceProfile, RT_RUSTJAR, Runtime, get_bootstrap_class_loader};
use jvm::{ClassInstance, ClassInstanceRef, JavaError, JavaValue, Jvm, Result, runtime::JavaLangString};

pub use compat::{
    DeepJarScan, JarScan, MemberSet, boot_jar_headless, collect_jar_api_classes, collect_jars, dump_runtime_api, load_boot_column, proto_registry,
    real_proto_registry, run_compat_lab, scan_compat_docs, scan_corpus, scan_jar, scan_jar_deep, scan_jar_with_registry, write_compat_matrix,
    write_missing_json,
};
pub use config::{HostConfig, init as init_config, parse_screen_size};
pub use launch::StartType;
use launch::{EntryPoint, get_jar_entry_point, read_jar_manifest, read_jar_manifest_bytes, screen_size_for_start_type};
use runtime::RuntimeImpl;

pub async fn run<T, S>(stdout: T, start_type: StartType<'_>, args: &[S], class_path: &[&Path]) -> anyhow::Result<()>
where
    T: Sync + Send + Write + 'static,
    S: AsRef<str>,
{
    let (jvm, runtime) = create_jvm(stdout, &start_type, class_path).await?;

    let midlet = match invoke_entrypoint(&jvm, &start_type, args).await {
        Ok(midlet) => midlet,
        Err(err) => {
            runtime.abort_spawned();
            tokio::task::yield_now().await;
            return Err(java_error_to_anyhow(&jvm, err).await);
        }
    };

    if let Some(midlet) = midlet {
        #[cfg(feature = "desktop-window")]
        {
            if let Err(err) = runtime.run_window(&jvm).await {
                runtime.abort_spawned();
                tokio::task::yield_now().await;
                return Err(err);
            }

            runtime.abort_spawned();
            tokio::task::yield_now().await;

            let _ = jvm.invoke_virtual::<_, ()>(&midlet, "destroyApp", "(Z)V", (true,)).await;
        }

        #[cfg(not(feature = "desktop-window"))]
        {
            let _ = midlet;
        }
    }

    runtime.abort_spawned();
    tokio::task::yield_now().await;

    Ok(())
}

async fn create_jvm<T>(stdout: T, start_type: &StartType<'_>, class_path: &[&Path]) -> anyhow::Result<(Jvm, RuntimeImpl<T>)>
where
    T: Sync + Send + Write + 'static,
{
    create_jvm_with_screen(stdout, start_type, class_path, None).await
}

pub async fn create_jvm_with_screen<T>(
    stdout: T,
    start_type: &StartType<'_>,
    class_path: &[&Path],
    screen_size: Option<(usize, usize)>,
) -> anyhow::Result<(Jvm, RuntimeImpl<T>)>
where
    T: Sync + Send + Write + 'static,
{
    let (screen_width, screen_height) = match screen_size {
        Some(size) => size,
        None => screen_size_for_start_type(start_type)?,
    };
    tracing::info!("using J2ME screen size {screen_width}x{screen_height}");

    let runtime_impl = RuntimeImpl::new(stdout, screen_width, screen_height);
    let runtime = Box::new(runtime_impl.clone()) as Box<dyn Runtime>;

    let bootstrap_class_loader = get_bootstrap_class_loader(runtime.clone());

    let class_path_separator = if cfg!(windows) { ";" } else { ":" };
    let mut class_path_str = class_path
        .iter()
        .map(|path| path.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(class_path_separator);
    match start_type {
        StartType::Jar(x) => {
            class_path_str = format!("{}{}{}", x.to_string_lossy(), class_path_separator, class_path_str);
        }
        StartType::JarBytes { name, bytes } => {
            runtime_impl.add_virtual_file(name.to_string_lossy(), bytes.to_vec());
            class_path_str = format!("{}{}{}", name.to_string_lossy(), class_path_separator, class_path_str);
        }
        StartType::Class(_) => {}
    }

    // add rt.rustjar
    // TODO do we need boot class path?
    let class_path_str = format!("{RT_RUSTJAR}{class_path_separator}{class_path_str}");
    let mut property_values = BTreeMap::new();
    property_values.insert("java.class.path".to_string(), class_path_str);
    property_values.insert("microedition.configuration".to_string(), "CLDC-1.1".to_string());
    property_values.insert("microedition.encoding".to_string(), "UTF-8".to_string());
    property_values.insert("microedition.locale".to_string(), "en-US".to_string());
    property_values.insert("microedition.platform".to_string(), "RustJava".to_string());
    property_values.insert("microedition.profiles".to_string(), "MIDP-2.0".to_string());
    property_values.insert("microedition.m3g.version".to_string(), "1.1".to_string());

    let jar_hint = match start_type {
        StartType::Jar(path) => path.to_string_lossy().into_owned(),
        StartType::JarBytes { name, .. } => name.to_string_lossy().into_owned(),
        StartType::Class(path) => path.to_string_lossy().into_owned(),
    };
    let mut profile_text = jar_hint.clone();

    let manifest = match start_type {
        StartType::Jar(jar_path) => Some(read_jar_manifest(jar_path)?),
        StartType::JarBytes { bytes, .. } => Some(read_jar_manifest_bytes(bytes)?),
        StartType::Class(_) => None,
    };
    if let Some(manifest) = manifest {
        for (key, value) in manifest {
            if key == "MicroEdition-Configuration" {
                property_values.insert("microedition.configuration".to_string(), value.clone());
            } else if key == "MicroEdition-Profile" {
                property_values.insert("microedition.profiles".to_string(), value.clone());
            }

            property_values.insert(key, value);
        }
        if let Some(midlet) = property_values.get("MIDlet-1") {
            profile_text.push(' ');
            profile_text.push_str(midlet);
        }
    }

    let api_classes = match start_type {
        StartType::Jar(path) => std::fs::read(path)
            .ok()
            .map(|bytes| compat::collect_jar_api_classes(&bytes))
            .unwrap_or_default(),
        StartType::JarBytes { bytes, .. } => compat::collect_jar_api_classes(bytes),
        StartType::Class(_) => Default::default(),
    };
    let profile = if let Some(forced) = config::get().device_profile.as_deref() {
        DeviceProfile::parse(forced)
    } else {
        DeviceProfile::detect_from_parts(&profile_text, api_classes.iter().map(String::as_str))
    };
    runtime_impl.set_device_profile(profile);
    let game_name = property_values
        .get("MIDlet-Name")
        .cloned()
        .or_else(|| {
            std::path::Path::new(&jar_hint)
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "RustJava".to_string());
    let session_label = format!("{game_name} [{profile}]");
    runtime_impl.set_session_label(session_label.clone());
    tracing::info!("auto device profile {profile} ({}) for {game_name}", profile.keyboard_hint());
    property_values.insert("microedition.platform".to_string(), profile.platform_name().to_string());
    if matches!(profile, DeviceProfile::Nokia | DeviceProfile::Generic) {
        property_values.insert("com.nokia.mid.ui.version".to_string(), "1.1".to_string());
    }

    let properties = property_values.iter().map(|(key, value)| (key.as_str(), value.as_str())).collect();

    let jvm = Jvm::new(bootstrap_class_loader, move || runtime.current_task_id(), properties).await?;
    jvm.set_now_millis(|| crate::runtime::clock::current_time_millis() as i64);
    Ok((jvm, runtime_impl))
}

pub async fn invoke_entrypoint<S>(jvm: &Jvm, start_type: &StartType<'_>, args: &[S]) -> Result<Option<Box<dyn ClassInstance>>>
where
    S: AsRef<str>,
{
    let entry_point = match start_type {
        StartType::Jar(x) => get_jar_entry_point(jvm, x).await?,
        StartType::JarBytes { name, .. } => get_jar_entry_point(jvm, name).await?,
        StartType::Class(x) => {
            let Some(name) = x.file_stem().and_then(|stem| stem.to_str()) else {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "main class path is not valid UTF-8")
                    .await);
            };
            EntryPoint::MainClass(name.to_string())
        }
    };

    match entry_point {
        EntryPoint::MainClass(main_class_name) => {
            let mut java_args = Vec::with_capacity(args.len());
            for arg in args {
                java_args.push(JavaLangString::from_rust_string(jvm, arg.as_ref()).await?);
            }
            let mut array = jvm.instantiate_array("Ljava/lang/String;", args.len()).await?;
            jvm.store_array(&mut array, 0, java_args).await?;

            let normalized_name = main_class_name.replace('.', "/");
            let _: () = jvm
                .invoke_static(&normalized_name, "main", "([Ljava/lang/String;)V", [JavaValue::Object(Some(array))])
                .await?;

            Ok(None)
        }
        EntryPoint::Midlet(midlet_class_name) => {
            let normalized_name = midlet_class_name.replace('.', "/");
            let midlet = jvm.new_class(&normalized_name, "()V", ()).await?;
            let _: () = jvm.invoke_virtual(&midlet, "startApp", "()V", ()).await?;
            Ok(Some(midlet))
        }
    }
}

pub async fn java_error_to_anyhow(jvm: &Jvm, err: JavaError) -> anyhow::Error {
    let JavaError::JavaException(x) = err;
    let class_name = x.class_definition().name();
    let mut detail = class_name.clone();

    if let Ok(Some(message)) = jvm
        .invoke_virtual::<_, Option<Box<dyn ClassInstance>>>(&x, "getMessage", "()Ljava/lang/String;", [])
        .await
    {
        if let Ok(text) = JavaLangString::to_rust_string(jvm, &message).await {
            if !text.is_empty() {
                detail = format!("{class_name}: {text}");
            }
        }
    }

    let trace = exception_stack_trace(jvm, &x).await.unwrap_or_default();
    if trace.trim().is_empty() {
        anyhow::anyhow!("Java Exception: {detail}")
    } else {
        anyhow::anyhow!("Java Exception: {detail}\n{trace}")
    }
}

async fn exception_stack_trace(jvm: &Jvm, exception: &Box<dyn ClassInstance>) -> anyhow::Result<String> {
    let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let print_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
        .await?;
    let _: () = jvm
        .invoke_virtual(exception, "printStackTrace", "(Ljava/io/PrintWriter;)V", (print_writer,))
        .await?;
    let trace: ClassInstanceRef<()> = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", []).await?;
    let Some(trace) = trace.instance.as_ref() else {
        return Ok(String::new());
    };
    Ok(JavaLangString::to_rust_string(jvm, trace).await?)
}

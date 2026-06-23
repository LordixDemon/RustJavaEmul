extern crate alloc;

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub mod browser;
mod launch;
mod profile;
pub mod runtime;

use std::{collections::BTreeMap, io::Write, path::Path};

use java_runtime::{RT_RUSTJAR, Runtime, get_bootstrap_class_loader};
use jvm::{ClassInstance, JavaError, JavaValue, Jvm, Result, runtime::JavaLangString};

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

            if let Err(err) = jvm.invoke_virtual::<_, ()>(&midlet, "destroyApp", "(Z)V", (true,)).await {
                runtime.abort_spawned();
                tokio::task::yield_now().await;
                return Err(java_error_to_anyhow(&jvm, err).await);
            }
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
        .map(|x| x.to_str().unwrap())
        .collect::<Vec<_>>()
        .join(class_path_separator);
    match start_type {
        StartType::Jar(x) => {
            class_path_str = format!("{}{}{}", x.to_str().unwrap(), class_path_separator, class_path_str);
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
    property_values.insert("microedition.configuration".to_string(), "CLDC-1.0".to_string());
    property_values.insert("microedition.encoding".to_string(), "UTF-8".to_string());
    property_values.insert("microedition.locale".to_string(), "en-US".to_string());
    property_values.insert("microedition.platform".to_string(), "RustJava".to_string());
    property_values.insert("microedition.profiles".to_string(), "MIDP-2.0".to_string());

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
    }

    let properties = property_values.iter().map(|(key, value)| (key.as_str(), value.as_str())).collect();

    Ok((
        Jvm::new(bootstrap_class_loader, move || runtime.current_task_id(), properties).await?,
        runtime_impl,
    ))
}

pub async fn invoke_entrypoint<S>(jvm: &Jvm, start_type: &StartType<'_>, args: &[S]) -> Result<Option<Box<dyn ClassInstance>>>
where
    S: AsRef<str>,
{
    let entry_point = match start_type {
        StartType::Jar(x) => get_jar_entry_point(jvm, x).await?,
        StartType::JarBytes { name, .. } => get_jar_entry_point(jvm, name).await?,
        StartType::Class(x) => EntryPoint::MainClass(x.file_stem().unwrap().to_str().unwrap().to_string()),
    };

    match entry_point {
        EntryPoint::MainClass(main_class_name) => {
            let mut java_args = Vec::with_capacity(args.len());
            for arg in args {
                java_args.push(JavaLangString::from_rust_string(jvm, arg.as_ref()).await?);
            }
            let mut array = jvm.instantiate_array("Ljava/lang/String;", args.len()).await?;
            jvm.store_array(&mut array, 0, java_args).await.unwrap();

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

    let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await.unwrap();
    let print_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
        .await
        .unwrap();

    let _: () = jvm
        .invoke_virtual(&x, "printStackTrace", "(Ljava/io/PrintWriter;)V", (print_writer,))
        .await
        .unwrap();

    let trace = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", []).await.unwrap();

    anyhow::anyhow!("Java Exception:\n{}", JavaLangString::to_rust_string(jvm, &trace).await.unwrap())
}

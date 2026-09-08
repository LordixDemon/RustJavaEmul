use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use classfile::{ClassInfo, ConstantPoolItem, FieldMethodref};
use java_runtime::{DeviceProfile, all_runtime_class_protos, zip::ZipArchive};

use crate::{StartType, create_jvm_with_screen, invoke_entrypoint, java_error_to_anyhow};

#[derive(Clone, Debug, Default)]
pub struct MemberSet {
    pub methods: BTreeSet<(String, String)>,
    pub fields: BTreeSet<(String, String)>,
}

#[derive(Clone, Debug)]
pub struct MissingMember {
    pub class: String,
    pub kind: &'static str,
    pub name: String,
    pub descriptor: String,
}

#[derive(Clone, Debug)]
pub struct JarScan {
    pub jar: PathBuf,
    pub profile: DeviceProfile,
    pub referenced_classes: BTreeSet<String>,
    pub missing: Vec<MissingMember>,
    pub implemented: usize,
    pub referenced: usize,
}

impl JarScan {
    pub fn static_percent(&self) -> f32 {
        if self.referenced == 0 {
            100.0
        } else {
            (self.implemented as f32) * 100.0 / self.referenced as f32
        }
    }
}

pub fn build_proto_registry(protos: Vec<java_runtime::RuntimeClassProto>) -> BTreeMap<String, MemberSet> {
    let mut by_name: BTreeMap<String, (Option<String>, Vec<String>, MemberSet)> = BTreeMap::new();
    for proto in protos {
        let mut set = MemberSet::default();
        for method in proto.methods {
            set.methods.insert((method.name, method.descriptor));
        }
        for field in proto.fields {
            set.fields.insert((field.name, field.descriptor));
        }
        by_name.insert(
            proto.name.to_string(),
            (
                proto.parent_class.map(str::to_string),
                proto.interfaces.iter().map(|iface| (*iface).to_string()).collect(),
                set,
            ),
        );
    }
    let names: Vec<String> = by_name.keys().cloned().collect();
    let mut map = BTreeMap::new();
    for name in names {
        map.insert(name.clone(), collect_inherited_members(&by_name, &name));
    }
    map
}

pub fn proto_registry() -> BTreeMap<String, MemberSet> {
    build_proto_registry(all_runtime_class_protos())
}

pub fn real_proto_registry() -> BTreeMap<String, MemberSet> {
    build_proto_registry(java_runtime::all_real_runtime_class_protos())
}

fn collect_inherited_members(by_name: &BTreeMap<String, (Option<String>, Vec<String>, MemberSet)>, name: &str) -> MemberSet {
    let mut out = MemberSet::default();
    let mut stack = vec![name.to_string()];
    let mut seen = BTreeSet::new();
    let mut first = true;
    while let Some(current) = stack.pop() {
        if !seen.insert(current.clone()) {
            continue;
        }
        let Some((parent, interfaces, set)) = by_name.get(&current) else {
            first = false;
            continue;
        };
        for (method_name, descriptor) in &set.methods {
            if !first && (method_name == "<init>" || method_name == "<clinit>") {
                continue;
            }
            out.methods.insert((method_name.clone(), descriptor.clone()));
        }
        out.fields.extend(set.fields.iter().cloned());
        if let Some(parent) = parent {
            stack.push(parent.clone());
        }
        stack.extend(interfaces.iter().cloned());
        first = false;
    }
    out
}

fn is_api_class(name: &str) -> bool {
    if name.starts_with('[') {
        return false;
    }
    name.starts_with("java/")
        || name.starts_with("javax/")
        || name.starts_with("com/nokia/")
        || name.starts_with("com/mascotcapsule/")
        || name.starts_with("com/siemens/")
        || name.starts_with("com/samsung/")
        || name.starts_with("com/motorola/")
        || name.starts_with("com/lg/")
        || name.starts_with("com/lge/")
        || name.starts_with("com/sonyericsson/")
        || name.starts_with("com/vodafone/")
        || name.starts_with("com/sprintpcs/")
}

pub fn collect_jar_api_classes(bytes: &[u8]) -> BTreeSet<String> {
    let Ok(zip) = ZipArchive::new(bytes) else {
        return BTreeSet::new();
    };
    let mut classes = BTreeSet::new();
    for index in 0..zip.len() {
        let entry = match zip.by_index(index) {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let Ok(name) = entry.name() else {
            continue;
        };
        if let Some(class) = name.strip_suffix(".class") {
            if is_api_class(class) {
                classes.insert(class.to_string());
            }
        }
        if !name.ends_with(".class") {
            continue;
        }
        let Ok(class_bytes) = entry.extract() else {
            continue;
        };
        let Some((constant_pool, this_class)) = ClassInfo::parse_constants(&class_bytes) else {
            continue;
        };
        if let Some(this_class) = this_class
            && is_api_class(this_class.as_str())
        {
            classes.insert(this_class.to_string());
        }
        for item in constant_pool.values() {
            if let ConstantPoolItem::Class { name_index } = item {
                if let Some(ConstantPoolItem::Utf8(value)) = constant_pool.get(name_index) {
                    if is_api_class(value.as_str()) {
                        classes.insert(value.to_string());
                    }
                }
            }
        }
    }
    classes
}

pub fn scan_jar(path: &Path) -> anyhow::Result<JarScan> {
    scan_jar_with_registry(path, &proto_registry())
}

pub fn scan_jar_with_registry(path: &Path, registry: &BTreeMap<String, MemberSet>) -> anyhow::Result<JarScan> {
    let raw = fs::read(path)?;
    let zip = ZipArchive::new(raw).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut jar_classes = BTreeSet::new();
    let mut pools = Vec::new();
    for index in 0..zip.len() {
        let entry = match zip.by_index(index) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let Ok(name) = entry.name() else {
            continue;
        };
        if !name.ends_with(".class") {
            continue;
        }
        let Ok(bytes) = entry.extract() else {
            continue;
        };
        let Some((constant_pool, this_class)) = ClassInfo::parse_constants(&bytes) else {
            continue;
        };
        if let Some(name) = this_class {
            jar_classes.insert(name.to_string());
        }
        pools.push(constant_pool);
    }

    let mut referenced_classes = BTreeSet::new();
    let mut referenced_methods: BTreeSet<(String, String, String)> = BTreeSet::new();
    let mut referenced_fields: BTreeSet<(String, String, String)> = BTreeSet::new();
    for constant_pool in &pools {
        for item in constant_pool.values() {
            match item {
                ConstantPoolItem::Class { name_index } => {
                    if let Some(name) = constant_pool.get(name_index).and_then(|item| match item {
                        ConstantPoolItem::Utf8(value) => Some(value.as_str().to_string()),
                        _ => None,
                    }) {
                        if is_api_class(&name) && !jar_classes.contains(&name) {
                            referenced_classes.insert(name);
                        }
                    }
                }
                ConstantPoolItem::Methodref {
                    class_index,
                    name_and_type_index,
                }
                | ConstantPoolItem::InterfaceMethodref {
                    class_index,
                    name_and_type_index,
                } => {
                    let Some(r) = FieldMethodref::try_from_reference_info(constant_pool, *class_index, *name_and_type_index) else {
                        continue;
                    };
                    if is_api_class(&r.class) && !jar_classes.contains(r.class.as_str()) {
                        referenced_classes.insert(r.class.to_string());
                        referenced_methods.insert((r.class.to_string(), r.name.to_string(), r.descriptor.to_string()));
                    }
                }
                ConstantPoolItem::Fieldref {
                    class_index,
                    name_and_type_index,
                } => {
                    let Some(r) = FieldMethodref::try_from_reference_info(constant_pool, *class_index, *name_and_type_index) else {
                        continue;
                    };
                    if is_api_class(&r.class) && !jar_classes.contains(r.class.as_str()) {
                        referenced_classes.insert(r.class.to_string());
                        referenced_fields.insert((r.class.to_string(), r.name.to_string(), r.descriptor.to_string()));
                    }
                }
                _ => {}
            }
        }
    }

    let profile = DeviceProfile::detect_from_parts(
        &path.to_string_lossy(),
        referenced_classes
            .iter()
            .map(String::as_str)
            .chain(jar_classes.iter().map(String::as_str)),
    );
    let mut missing = Vec::new();
    let mut implemented = 0usize;
    let mut referenced = 0usize;
    for class in &referenced_classes {
        if !profile.allows_class(class) {
            continue;
        }
        referenced += 1;
        if registry.contains_key(class) {
            implemented += 1;
        } else {
            missing.push(MissingMember {
                class: class.clone(),
                kind: "class",
                name: String::new(),
                descriptor: String::new(),
            });
        }
    }
    for (class, name, descriptor) in referenced_methods {
        if !profile.allows_class(&class) {
            continue;
        }
        referenced += 1;
        if registry
            .get(&class)
            .is_some_and(|set| set.methods.contains(&(name.clone(), descriptor.clone())))
        {
            implemented += 1;
        } else {
            missing.push(MissingMember {
                class,
                kind: "method",
                name,
                descriptor,
            });
        }
    }
    for (class, name, descriptor) in referenced_fields {
        if !profile.allows_class(&class) {
            continue;
        }
        referenced += 1;
        if registry
            .get(&class)
            .is_some_and(|set| set.fields.contains(&(name.clone(), descriptor.clone())))
        {
            implemented += 1;
        } else {
            missing.push(MissingMember {
                class,
                kind: "field",
                name,
                descriptor,
            });
        }
    }

    Ok(JarScan {
        jar: path.to_path_buf(),
        profile,
        referenced_classes,
        missing,
        implemented,
        referenced,
    })
}

#[derive(Clone, Debug, Default)]
pub struct DeepJarScan {
    pub jar: PathBuf,
    pub total_classes: usize,
    pub total_methods: usize,
    pub total_fields: usize,
    pub missing_classes: Vec<String>,
    pub missing_methods: Vec<(String, String, String)>,
    pub missing_fields: Vec<(String, String, String)>,
    pub stubbed_classes: Vec<String>,
    pub stubbed_methods: Vec<(String, String, String)>,
    pub stubbed_fields: Vec<(String, String, String)>,
    pub real_classes: usize,
    pub real_methods: usize,
    pub real_fields: usize,
}

impl DeepJarScan {
    pub fn is_100_percent_covered(&self) -> bool {
        self.missing_classes.is_empty() && self.missing_methods.is_empty() && self.missing_fields.is_empty()
    }

    pub fn total_referenced(&self) -> usize {
        self.total_classes + self.total_methods + self.total_fields
    }

    pub fn total_missing(&self) -> usize {
        self.missing_classes.len() + self.missing_methods.len() + self.missing_fields.len()
    }

    pub fn total_stubbed(&self) -> usize {
        self.stubbed_classes.len() + self.stubbed_methods.len() + self.stubbed_fields.len()
    }

    pub fn total_real(&self) -> usize {
        self.real_classes + self.real_methods + self.real_fields
    }
}

pub fn scan_jar_deep(
    path: &Path,
    real_registry: &BTreeMap<String, MemberSet>,
    full_registry: &BTreeMap<String, MemberSet>,
) -> anyhow::Result<DeepJarScan> {
    let raw = fs::read(path)?;
    let zip = ZipArchive::new(raw).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut jar_classes = BTreeSet::new();
    let mut pools = Vec::new();
    for index in 0..zip.len() {
        let entry = match zip.by_index(index) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let Ok(name) = entry.name() else {
            continue;
        };
        if !name.ends_with(".class") {
            continue;
        }
        let Ok(bytes) = entry.extract() else {
            continue;
        };
        let Some((constant_pool, this_class)) = ClassInfo::parse_constants(&bytes) else {
            continue;
        };
        if let Some(name) = this_class {
            jar_classes.insert(name.to_string());
        }
        pools.push(constant_pool);
    }

    let mut referenced_classes = BTreeSet::new();
    let mut referenced_methods = BTreeSet::new();
    let mut referenced_fields = BTreeSet::new();

    for constant_pool in &pools {
        for item in constant_pool.values() {
            match item {
                ConstantPoolItem::Class { name_index } => {
                    if let Some(name) = constant_pool.get(name_index).and_then(|item| match item {
                        ConstantPoolItem::Utf8(value) => Some(value.as_str().to_string()),
                        _ => None,
                    }) {
                        if !name.starts_with('[') && !jar_classes.contains(&name) {
                            referenced_classes.insert(name);
                        }
                    }
                }
                ConstantPoolItem::Methodref {
                    class_index,
                    name_and_type_index,
                }
                | ConstantPoolItem::InterfaceMethodref {
                    class_index,
                    name_and_type_index,
                } => {
                    let Some(r) = FieldMethodref::try_from_reference_info(constant_pool, *class_index, *name_and_type_index) else {
                        continue;
                    };
                    if !r.class.starts_with('[') && !jar_classes.contains(r.class.as_str()) {
                        referenced_classes.insert(r.class.to_string());
                        referenced_methods.insert((r.class.to_string(), r.name.to_string(), r.descriptor.to_string()));
                    }
                }
                ConstantPoolItem::Fieldref {
                    class_index,
                    name_and_type_index,
                } => {
                    let Some(r) = FieldMethodref::try_from_reference_info(constant_pool, *class_index, *name_and_type_index) else {
                        continue;
                    };
                    if !r.class.starts_with('[') && !jar_classes.contains(r.class.as_str()) {
                        referenced_classes.insert(r.class.to_string());
                        referenced_fields.insert((r.class.to_string(), r.name.to_string(), r.descriptor.to_string()));
                    }
                }
                _ => {}
            }
        }
    }

    let mut scan = DeepJarScan {
        jar: path.to_path_buf(),
        total_classes: referenced_classes.len(),
        total_methods: referenced_methods.len(),
        total_fields: referenced_fields.len(),
        ..Default::default()
    };

    for class in referenced_classes {
        if real_registry.contains_key(&class) {
            scan.real_classes += 1;
        } else if full_registry.contains_key(&class) {
            scan.stubbed_classes.push(class);
        } else {
            scan.missing_classes.push(class);
        }
    }

    for (class, name, desc) in referenced_methods {
        if real_registry
            .get(&class)
            .is_some_and(|s| s.methods.contains(&(name.clone(), desc.clone())))
        {
            scan.real_methods += 1;
        } else if full_registry
            .get(&class)
            .is_some_and(|s| s.methods.contains(&(name.clone(), desc.clone())))
        {
            scan.stubbed_methods.push((class, name, desc));
        } else {
            scan.missing_methods.push((class, name, desc));
        }
    }

    for (class, name, desc) in referenced_fields {
        if real_registry
            .get(&class)
            .is_some_and(|s| s.fields.contains(&(name.clone(), desc.clone())))
        {
            scan.real_fields += 1;
        } else if full_registry
            .get(&class)
            .is_some_and(|s| s.fields.contains(&(name.clone(), desc.clone())))
        {
            scan.stubbed_fields.push((class, name, desc));
        } else {
            scan.missing_fields.push((class, name, desc));
        }
    }

    Ok(scan)
}

pub fn collect_jars(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut jars = Vec::new();
    collect_jars_inner(root, &mut jars)?;
    jars.sort();
    Ok(jars)
}

fn collect_jars_inner(root: &Path, jars: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    if root.is_file() {
        if root.extension().and_then(|ext| ext.to_str()) == Some("jar") {
            jars.push(root.to_path_buf());
        }
        return Ok(());
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_jars_inner(&path, jars)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("jar") {
            jars.push(path);
        }
    }
    Ok(())
}

pub fn scan_corpus(root: &Path) -> anyhow::Result<Vec<JarScan>> {
    let registry = proto_registry();
    let jars = collect_jars(root)?;
    Ok(scan_jars_parallel(&jars, &registry))
}

fn scan_jars_parallel(jars: &[PathBuf], registry: &BTreeMap<String, MemberSet>) -> Vec<JarScan> {
    if jars.is_empty() {
        return Vec::new();
    }

    #[cfg(target_arch = "wasm32")]
    {
        let mut scans = Vec::with_capacity(jars.len());
        for jar in jars {
            match scan_jar_with_registry(jar, registry) {
                Ok(scan) => scans.push(scan),
                Err(err) => eprintln!("skip {}: {err}", jar.display()),
            }
        }
        scans
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let threads = std::thread::available_parallelism()
            .map(|n| n.get().saturating_mul(2).clamp(2, 32))
            .unwrap_or(8)
            .min(jars.len())
            .max(1);
        let chunk_size = jars.len().div_ceil(threads);
        std::thread::scope(|scope| {
            let mut workers = Vec::with_capacity(threads);
            for chunk in jars.chunks(chunk_size) {
                workers.push(scope.spawn(move || {
                    let mut out = Vec::with_capacity(chunk.len());
                    for jar in chunk {
                        match scan_jar_with_registry(jar, registry) {
                            Ok(scan) => out.push(scan),
                            Err(err) => eprintln!("skip {}: {err}", jar.display()),
                        }
                    }
                    out
                }));
            }
            let mut scans = Vec::with_capacity(jars.len());
            for worker in workers {
                scans.extend(worker.join().unwrap_or_else(|_| Vec::new()));
            }
            scans.sort_by(|a, b| a.jar.cmp(&b.jar));
            scans
        })
    }
}

pub fn write_missing_json(scans: &[JarScan], path: &Path) -> anyhow::Result<()> {
    let mut out = String::from("{\n");
    for (index, scan) in scans.iter().enumerate() {
        out.push_str(&format!("  \"{}\": [\n", scan.jar.display().to_string().replace('\\', "/")));
        for (member_index, member) in scan.missing.iter().enumerate() {
            out.push_str(&format!(
                "    {{\"kind\":\"{}\",\"class\":\"{}\",\"name\":\"{}\",\"descriptor\":\"{}\"}}{}\n",
                member.kind,
                member.class,
                member.name,
                member.descriptor,
                if member_index + 1 == scan.missing.len() { "" } else { "," }
            ));
        }
        out.push_str(if index + 1 == scans.len() { "  ]\n" } else { "  ],\n" });
    }
    out.push_str("}\n");
    fs::write(path, out)?;
    Ok(())
}

pub fn write_compat_matrix(scans: &[JarScan], boot: &[(String, String)], path: &Path) -> anyhow::Result<()> {
    let mut table = String::from("# J2ME compatibility matrix\n\n");
    table.push_str("| Game | Profile | Static % | Missing | Boot |\n| --- | --- | ---: | ---: | --- |\n");
    for scan in scans {
        let name = scan.jar.file_name().and_then(|n| n.to_str()).unwrap_or("?");
        let boot_status = boot
            .iter()
            .find(|(jar, _)| jar.ends_with(name))
            .map(|(_, status)| status.as_str())
            .unwrap_or("not-run");
        table.push_str(&format!(
            "| {} | {} | {:.1} | {} | {} |\n",
            name,
            scan.profile.as_str(),
            scan.static_percent(),
            scan.missing.len(),
            boot_status
        ));
    }
    fs::write(path, table)?;
    Ok(())
}

pub fn dump_runtime_api(path: &Path) -> anyhow::Result<()> {
    let mut out = String::from("# Runtime public API dump\n\nGenerated from registered JavaClassProto members.\n\n");
    for proto in all_runtime_class_protos() {
        out.push_str(&format!("## {}\n", proto.name));
        for field in &proto.fields {
            out.push_str(&format!("- field {} {}\n", field.name, field.descriptor));
        }
        for method in &proto.methods {
            out.push_str(&format!("- method {} {}\n", method.name, method.descriptor));
        }
        out.push('\n');
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, out)?;
    Ok(())
}

pub async fn boot_jar_headless(jar: &Path, timeout: Duration) -> (String, String) {
    let name = jar.display().to_string();
    let (jvm, runtime) = match create_jvm_with_screen(std::io::sink(), &StartType::Jar(jar), &[Path::new(".")], None).await {
        Ok(res) => res,
        Err(err) => return (name, first_line(&err.to_string())),
    };
    let result = tokio::time::timeout(timeout, invoke_entrypoint(&jvm, &StartType::Jar(jar), &[] as &[&str])).await;
    runtime.abort_spawned();
    tokio::task::yield_now().await;
    match result {
        Ok(Ok(_)) => (name, "ok".to_string()),
        Ok(Err(err)) => {
            let full_err = java_error_to_anyhow(&jvm, err).await.to_string();
            eprintln!("BOOT ERROR:\n{full_err}");
            (name, first_line(&full_err))
        }
        Err(_) => (name, "running".to_string()),
    }
}

fn first_line(text: &str) -> String {
    text.lines().next().unwrap_or(text).chars().take(160).collect()
}

pub async fn boot_jar_isolated(jar: &Path, timeout: Duration) -> (String, String) {
    #[cfg(target_arch = "wasm32")]
    {
        boot_jar_headless(jar, timeout).await
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let name = jar.display().to_string();
        let exe = match std::env::current_exe() {
            Ok(path) => path,
            Err(err) => return (name, format!("spawn-error: {err}")),
        };
        let mut command = tokio::process::Command::new(exe);
        command
            .arg("--compat-boot")
            .arg(jar)
            .env("RUST_MIN_STACK", "16777216")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);
        let child = match command.spawn() {
            Ok(child) => child,
            Err(err) => return (name, format!("spawn-error: {err}")),
        };
        match tokio::time::timeout(timeout + Duration::from_secs(2), child.wait_with_output()).await {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let status_line = first_line(stdout.trim());
                if output.status.success() {
                    (name, if status_line.is_empty() { "ok".to_string() } else { status_line })
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let kind = if output.status.code().is_none() { "crash" } else { "error" };
                    let detail = [status_line.as_str(), stderr.trim()]
                        .into_iter()
                        .find(|part| !part.is_empty())
                        .unwrap_or("process failed");
                    (name, first_line(&format!("{kind}: {detail}")))
                }
            }
            Ok(Err(err)) => (name, format!("io-error: {err}")),
            Err(_) => (name, "running".to_string()),
        }
    }
}

pub fn load_boot_column(path: &Path) -> Vec<(String, String)> {
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut boot = Vec::new();
    for line in text.lines() {
        if !line.starts_with('|') || line.contains("---") || line.contains("Game") {
            continue;
        }
        let cols: Vec<&str> = line.split('|').map(str::trim).filter(|part| !part.is_empty()).collect();
        if cols.len() >= 5 {
            boot.push((cols[0].to_string(), cols[4..].join(" | ")));
        }
    }
    boot
}

pub fn scan_compat_docs(root: &Path) -> anyhow::Result<()> {
    let scans = scan_corpus(root)?;
    fs::create_dir_all("docs/spec")?;
    dump_runtime_api(Path::new("docs/spec/runtime_api.md"))?;
    write_missing_json(&scans, Path::new("docs/missing.json"))?;
    let boot = load_boot_column(Path::new("docs/compat_matrix.md"));
    write_compat_matrix(&scans, &boot, Path::new("docs/compat_matrix.md"))?;
    let missing_jars = scans.iter().filter(|scan| !scan.missing.is_empty()).count();
    let missing_members: usize = scans.iter().map(|scan| scan.missing.len()).sum();
    println!(
        "scanned {} jars, {} jars with missing members, {} missing members total",
        scans.len(),
        missing_jars,
        missing_members
    );
    Ok(())
}

pub async fn run_compat_lab(root: &Path) -> anyhow::Result<()> {
    let scans = scan_corpus(root)?;
    fs::create_dir_all("docs/spec")?;
    dump_runtime_api(Path::new("docs/spec/runtime_api.md"))?;
    write_missing_json(&scans, Path::new("docs/missing.json"))?;
    let mut boot = Vec::new();
    for scan in &scans {
        boot.push(boot_jar_isolated(&scan.jar, Duration::from_secs(8)).await);
        println!(
            "{} profile={} static={:.1}% missing={} boot={}",
            scan.jar.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
            scan.profile,
            scan.static_percent(),
            scan.missing.len(),
            boot.last().map(|(_, status)| status.as_str()).unwrap_or("?")
        );
    }
    write_compat_matrix(&scans, &boot, Path::new("docs/compat_matrix.md"))?;
    Ok(())
}

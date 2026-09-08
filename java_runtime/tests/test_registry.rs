use std::collections::HashSet;

use java_runtime::{all_runtime_class_protos, classes, get_runtime_class_proto};

#[test]
fn runtime_class_proto_names_are_unique() {
    let protos = all_runtime_class_protos();
    let mut names = HashSet::new();
    for proto in &protos {
        assert!(names.insert(proto.name), "duplicate runtime class proto name: {}", proto.name);
    }
}

#[test]
fn runtime_class_proto_count_matches_package_lists() {
    let package_count = classes::com::class_protos().len()
        + classes::java::class_protos().len()
        + classes::javax::class_protos().len()
        + classes::org::class_protos().len()
        + classes::root::class_protos().len();

    let real_protos = java_runtime::all_real_runtime_class_protos();
    let proto_count = all_runtime_class_protos().len();
    println!("real_protos count: {}, all_runtime_class_protos: {}, package_count: {}", real_protos.len(), proto_count, package_count);
    assert_eq!(
        real_protos.len(),
        package_count,
        "real proto count {} does not match package factory count {}",
        real_protos.len(),
        package_count
    );
    assert!(
        proto_count >= package_count,
        "runtime proto count {proto_count} is smaller than package factory count {package_count}"
    );
}

#[test]
fn get_runtime_class_proto_finds_core_and_vendor_classes() {
    for name in [
        "java/lang/Object",
        "java/lang/String",
        "java/lang/NullPointerException",
        "javax/microedition/lcdui/Display",
        "com/nokia/mid/ui/DeviceControl",
        "com/samsung/util/Vibration",
        "com/siemens/mp/game/Vibrator",
        "org/xml/sax/helpers/DefaultHandler",
        "javax/xml/parsers/SAXParser",
        "java/util/zip/ZipException",
        "org/xml/sax/InputSource",
        "net/rim/device/api/system/KeyListener",
    ] {
        let proto = get_runtime_class_proto(name).unwrap_or_else(|| panic!("missing runtime class proto: {name}"));
        assert_eq!(proto.name, name);
    }

    assert!(get_runtime_class_proto("java/lang/DoesNotExist").is_none());
}

#[tokio::test]
async fn test_all_runtime_classes_resolve_and_instantiate() -> jvm::Result<()> {
    let jvm = test_utils::test_jvm().await?;
    let protos = all_runtime_class_protos();
    let mut resolved_count = 0;
    let mut heap_instantiated_count = 0;
    let mut constructor_invoked_count = 0;
    let mut interfaces_count = 0;
    let mut total_methods_verified = 0;
    let mut total_fields_verified = 0;

    for proto in &protos {
        let class = jvm.resolve_class(proto.name).await?;
        resolved_count += 1;

        let is_interface = class.definition.access_flags().contains(java_constants::ClassAccessFlags::INTERFACE);
        let is_abstract = class.definition.access_flags().contains(java_constants::ClassAccessFlags::ABSTRACT);

        // Verify parent class is resolvable if present
        if let Some(super_name) = class.definition.super_class_name() {
            let super_class = jvm.resolve_class(&super_name).await;
            assert!(
                super_class.is_ok(),
                "class {} declared superclass {} which cannot be resolved: {:?}",
                proto.name,
                super_name,
                super_class.err()
            );
        }

        // Verify all implemented interfaces are resolvable
        for iface_name in class.definition.interface_names() {
            let iface = jvm.resolve_class(&iface_name).await;
            assert!(
                iface.is_ok(),
                "class {} declared interface {} which cannot be resolved: {:?}",
                proto.name,
                iface_name,
                iface.err()
            );
        }

        // Verify all declared methods exist on definition
        for m in &proto.methods {
            let is_static = m.access_flags.contains(java_constants::MethodAccessFlags::STATIC);
            let method = class.definition.method(&m.name, &m.descriptor, is_static);
            assert!(
                method.is_some(),
                "class {} method {}{} (static={}) not found on definition",
                proto.name,
                m.name,
                m.descriptor,
                is_static
            );
            total_methods_verified += 1;
        }

        // Verify all declared fields exist on definition
        for f in &proto.fields {
            let is_static = f.access_flags.contains(java_constants::FieldAccessFlags::STATIC);
            let field = class.definition.field(&f.name, &f.descriptor, is_static);
            assert!(
                field.is_some(),
                "class {} field {}:{} (static={}) not found on definition",
                proto.name,
                f.name,
                f.descriptor,
                is_static
            );
            total_fields_verified += 1;
        }

        if is_interface {
            interfaces_count += 1;
            continue;
        }

        // Exercise heap instantiation for every non-interface class
        let heap_instance = class.definition.instantiate(&jvm).await;
        assert!(
            heap_instance.is_ok(),
            "failed to allocate heap instance for class {}: {:?}",
            proto.name,
            heap_instance.err()
        );
        heap_instantiated_count += 1;

        // Exercise constructor execution for non-abstract classes with <init>()V
        let has_no_arg_init = proto.methods.iter().any(|m| m.name == "<init>" && m.descriptor == "()V");
        if has_no_arg_init && !is_abstract {
            let instance = jvm.new_class(proto.name, "()V", ()).await;
            assert!(
                instance.is_ok(),
                "failed to invoke <init>()V on class {}: {:?}",
                proto.name,
                instance.err()
            );
            constructor_invoked_count += 1;
        }
    }

    println!(
        "TEST RESULTS: Total Classes: {}, Resolved: {}, Heap-Instantiated: {}, Constructors Invoked: {}, Interfaces: {}, Methods Verified: {}, Fields Verified: {}",
        protos.len(),
        resolved_count,
        heap_instantiated_count,
        constructor_invoked_count,
        interfaces_count,
        total_methods_verified,
        total_fields_verified
    );

    assert_eq!(resolved_count, protos.len());
    assert_eq!(heap_instantiated_count + interfaces_count, protos.len());
    assert!(constructor_invoked_count > 1000);

    Ok(())
}

#[tokio::test]
async fn test_all_real_runtime_classes_thoroughly() -> jvm::Result<()> {
    let jvm = test_utils::test_jvm().await?;
    let real_protos = java_runtime::all_real_runtime_class_protos();
    let mut real_instantiated = 0;
    let mut real_resolved = 0;

    for proto in &real_protos {
        let class = jvm.resolve_class(proto.name).await?;
        real_resolved += 1;

        let is_interface = class.definition.access_flags().contains(java_constants::ClassAccessFlags::INTERFACE);
        let is_abstract = class.definition.access_flags().contains(java_constants::ClassAccessFlags::ABSTRACT);

        if is_interface {
            continue;
        }

        // Heap instantiate
        let _ = class.definition.instantiate(&jvm).await?;

        // If it has <init>()V, invoke it
        if !is_abstract && proto.methods.iter().any(|m| m.name == "<init>" && m.descriptor == "()V") {
            jvm.new_class(proto.name, "()V", ()).await?;
            real_instantiated += 1;
        }
    }

    println!(
        "REAL CLASSES TEST: {} real classes resolved, {} real classes directly instantiated with <init>()V",
        real_resolved,
        real_instantiated
    );
    assert_eq!(real_resolved, real_protos.len());
    assert_eq!(real_instantiated, 201);

    Ok(())
}



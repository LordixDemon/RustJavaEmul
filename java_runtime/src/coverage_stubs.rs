use alloc::{collections::BTreeMap, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use parking_lot::Mutex;

use crate::RuntimeClassProto;

#[path = "coverage_stub_data.rs"]
mod coverage_stub_data;
use coverage_stub_data::COVERAGE_STUBS;

#[derive(Clone, Copy)]
struct Member {
    kind: u8,
    name: &'static str,
    descriptor: &'static str,
}

type Index = BTreeMap<&'static str, Vec<Member>>;

fn build_index() -> Index {
    let mut map = Index::new();
    for &(class, kind, name, descriptor) in COVERAGE_STUBS {
        map.entry(class).or_default().push(Member { kind, name, descriptor });
    }
    map
}

fn with_index<R>(f: impl FnOnce(&Index) -> R) -> R {
    static INDEX: Mutex<Option<Index>> = Mutex::new(None);
    let mut index = INDEX.lock();
    if index.is_none() {
        *index = Some(build_index());
    }
    f(index.as_ref().unwrap())
}

pub fn class_proto(name: &str) -> Option<RuntimeClassProto> {
    with_index(|index| index.get_key_value(name).map(|(class, _)| empty_class(*class)))
}

fn empty_class(name: &'static str) -> RuntimeClassProto {
    let parent = if name.ends_with("Error") {
        "java/lang/Error"
    } else if name.ends_with("Exception") {
        "java/lang/Exception"
    } else if name.ends_with("MIDlet") || name.ends_with("Midlet") {
        "javax/microedition/midlet/MIDlet"
    } else if name.ends_with("InputStream") {
        "java/io/InputStream"
    } else if name.ends_with("OutputStream") {
        "java/io/OutputStream"
    } else {
        "java/lang/Object"
    };
    RuntimeClassProto {
        name,
        parent_class: Some(parent),
        interfaces: vec![],
        methods: vec![JavaMethodProto::new_stub("<init>", "()V", Default::default())],
        fields: vec![],
        access_flags: Default::default(),
    }
}

pub fn merge_into(protos: &mut Vec<RuntimeClassProto>) {
    with_index(|index| {
        let mut have: BTreeMap<&str, ()> = BTreeMap::new();
        for proto in protos.iter() {
            have.insert(proto.name, ());
        }
        for class in index.keys().copied() {
            if !have.contains_key(class) {
                protos.push(empty_class(class));
                have.insert(class, ());
            }
        }
        for proto in protos.iter_mut() {
            if let Some(members) = index.get(proto.name) {
                apply_members(proto, members);
            }
        }
    });
}

pub fn apply_to(proto: &mut RuntimeClassProto) {
    with_index(|index| {
        if let Some(members) = index.get(proto.name) {
            apply_members(proto, members);
        }
    });
}

fn is_object_method(name: &str, descriptor: &str) -> bool {
    matches!(
        (name, descriptor),
        ("getClass", "()Ljava/lang/Class;")
            | ("hashCode", "()I")
            | ("equals", "(Ljava/lang/Object;)Z")
            | ("toString", "()Ljava/lang/String;")
            | ("notify", "()V")
            | ("notifyAll", "()V")
            | ("wait", "()V")
            | ("wait", "(J)V")
            | ("wait", "(JI)V")
            | ("clone", "()Ljava/lang/Object;")
            | ("finalize", "()V")
    )
}

fn is_throwable_method(name: &str, descriptor: &str) -> bool {
    matches!(
        (name, descriptor),
        ("fillInStackTrace", "()Ljava/lang/Throwable;")
            | ("getMessage", "()Ljava/lang/String;")
            | ("getLocalizedMessage", "()Ljava/lang/String;")
            | ("getCause", "()Ljava/lang/Throwable;")
            | ("initCause", "(Ljava/lang/Throwable;)Ljava/lang/Throwable;")
            | ("printStackTrace", "()V")
            | ("printStackTrace", "(Ljava/io/PrintStream;)V")
            | ("printStackTrace", "(Ljava/io/PrintWriter;)V")
    )
}

fn is_throwable_family(proto: &RuntimeClassProto) -> bool {
    proto.name == "java/lang/Throwable"
        || proto.name.ends_with("Exception")
        || proto.name.ends_with("Error")
        || matches!(
            proto.parent_class,
            Some("java/lang/Throwable" | "java/lang/Exception" | "java/lang/Error" | "java/lang/RuntimeException")
        )
}

fn skip_inherited_method(proto: &RuntimeClassProto, name: &str, descriptor: &str) -> bool {
    is_object_method(name, descriptor) || (is_throwable_family(proto) && is_throwable_method(name, descriptor))
}

fn apply_members(proto: &mut RuntimeClassProto, members: &[Member]) {
    for member in members {
        match member.kind {
            1 => {
                if skip_inherited_method(proto, member.name, member.descriptor) {
                    continue;
                }
                if proto
                    .methods
                    .iter()
                    .any(|method| method.name == member.name && method.descriptor == member.descriptor)
                {
                    continue;
                }
                if member.name == "<clinit>" {
                    proto
                        .methods
                        .push(JavaMethodProto::new_stub(member.name, member.descriptor, MethodAccessFlags::STATIC));
                } else {
                    proto
                        .methods
                        .push(JavaMethodProto::new_stub(member.name, member.descriptor, Default::default()));
                    if member.name != "<init>" {
                        proto
                            .methods
                            .push(JavaMethodProto::new_stub(member.name, member.descriptor, MethodAccessFlags::STATIC));
                    }
                }
            }
            2 => {
                if proto
                    .fields
                    .iter()
                    .any(|field| field.name == member.name && field.descriptor == member.descriptor)
                {
                    continue;
                }
                proto.fields.push(JavaFieldProto::new(member.name, member.descriptor, Default::default()));
                proto
                    .fields
                    .push(JavaFieldProto::new(member.name, member.descriptor, FieldAccessFlags::STATIC));
            }
            _ => {}
        }
    }
}

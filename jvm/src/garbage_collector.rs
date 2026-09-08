use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use java_constants::FieldAccessFlags;

use hashbrown::{HashMap, HashSet};

use crate::{ClassDefinition, ClassInstance, Field, JavaValue, Jvm, class_loader::Class, thread::JvmThread};

pub fn determine_reachable_objects(
    jvm: &Jvm,
    threads: &BTreeMap<u64, JvmThread>,
    classes: &BTreeMap<String, Class>,
    interned_strings: &BTreeMap<String, Box<dyn ClassInstance>>,
) -> HashSet<Box<dyn ClassInstance>> {
    let mut reachable = HashSet::new();
    let mut worklist: Vec<Box<dyn ClassInstance>> = Vec::new();
    let mut class_ref_fields_cache: HashMap<String, Vec<Box<dyn Field>>> = HashMap::new();

    let mut push_root = |obj: &Box<dyn ClassInstance>| {
        if !reachable.contains(obj) {
            reachable.insert(obj.clone());
            worklist.push(obj.clone());
        }
    };

    // 1. Classes & static roots
    for class in classes.values() {
        push_root(&class.java_class());

        let fields = find_all_fields(jvm, &*class.definition);
        for field in fields {
            if !field.access_flags().contains(FieldAccessFlags::STATIC) {
                continue;
            }

            let descriptor = field.descriptor();
            if is_ref_descriptor(&descriptor) {
                if let Ok(JavaValue::Object(Some(value))) = class.definition.get_static_field(&*field) {
                    push_root(&value);
                }
            }
        }
    }

    // 2. Interned strings
    for value in interned_strings.values() {
        push_root(value);
    }

    // 3. Thread stack frames and extra roots
    for thread in threads.values() {
        for stack in thread.iter_frame() {
            for x in stack.local_variables().iter().chain(stack.extra_roots()) {
                push_root(x);
            }
        }
    }

    // 4. Iterative object graph traversal
    while let Some(current) = worklist.pop() {
        if let Some(array) = current.as_array_instance() {
            if array.is_object_array() {
                if let Ok(values) = array.load(0, array.length()) {
                    for value in values {
                        if let JavaValue::Object(Some(child)) = value {
                            if !reachable.contains(&child) {
                                reachable.insert(child.clone());
                                worklist.push(child);
                            }
                        }
                    }
                }
            }
            // Primitive arrays contain no references
        } else {
            let class_def = current.class_definition();
            let class_name = class_def.name();

            let ref_fields = class_ref_fields_cache.entry(class_name).or_insert_with(|| {
                let all_fields = find_all_fields(jvm, &*class_def);
                all_fields
                    .into_iter()
                    .filter(|f| !f.access_flags().contains(FieldAccessFlags::STATIC))
                    .filter(|f| is_ref_descriptor(&f.descriptor()))
                    .collect()
            });

            for field in ref_fields.iter() {
                if let Ok(JavaValue::Object(Some(child))) = current.get_field(&**field) {
                    if !reachable.contains(&child) {
                        reachable.insert(child.clone());
                        worklist.push(child);
                    }
                }
            }
        }
    }

    reachable
}

#[allow(dead_code)]
pub fn determine_garbage(
    jvm: &Jvm,
    threads: &BTreeMap<u64, JvmThread>,
    all_class_instances: &HashSet<Box<dyn ClassInstance>>,
    classes: &BTreeMap<String, Class>,
    interned_strings: &BTreeMap<String, Box<dyn ClassInstance>>,
) -> Vec<Box<dyn ClassInstance>> {
    let reachable_objects = determine_reachable_objects(jvm, threads, classes, interned_strings);
    all_class_instances.difference(&reachable_objects).cloned().collect()
}

#[inline]
fn is_ref_descriptor(descriptor: &str) -> bool {
    (descriptor.starts_with('L') && descriptor.ends_with(';')) || descriptor.starts_with('[')
}

fn find_all_fields(jvm: &Jvm, class_definition: &dyn ClassDefinition) -> Vec<Box<dyn Field>> {
    let result = class_definition.fields();
    let super_class_name = class_definition.super_class_name();

    if let Some(x) = super_class_name {
        if let Some(super_class) = jvm.get_class(&x) {
            let super_fields = find_all_fields(jvm, &*super_class.definition);
            result.into_iter().chain(super_fields).collect()
        } else {
            result
        }
    } else {
        result
    }
}

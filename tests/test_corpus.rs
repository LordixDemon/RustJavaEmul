use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    time::Duration,
};

use rust_java::{boot_jar_headless, scan_corpus};

const CORPUS_ROOT: &str = "apk/corpus";
const ORIGINAL_VENDOR_GAMES: usize = 81;
const EXTRA_API_GAMES: usize = 100;
const API_FAMILIES: [&str; 8] = ["canvas", "gamecanvas", "sprite", "rms", "form", "list", "fontgfx", "tone"];
const EXPECTED_FAMILY_CLASS: [(&str, &str); 7] = [
    ("gamecanvas", "javax/microedition/lcdui/game/GameCanvas"),
    ("sprite", "javax/microedition/lcdui/game/Sprite"),
    ("rms", "javax/microedition/rms/RecordStore"),
    ("form", "javax/microedition/lcdui/Form"),
    ("list", "javax/microedition/lcdui/List"),
    ("fontgfx", "javax/microedition/lcdui/Font"),
    ("tone", "javax/microedition/media/Manager"),
];

fn corpus_jars() -> Option<Vec<rust_java::JarScan>> {
    if !Path::new(CORPUS_ROOT).is_dir() {
        eprintln!("skipping: {CORPUS_ROOT} is not present");
        return None;
    }
    Some(scan_corpus(Path::new(CORPUS_ROOT)).unwrap_or_else(|err| panic!("failed to scan {CORPUS_ROOT}: {err}")))
}

fn apk_jars() -> Option<Vec<rust_java::JarScan>> {
    if !Path::new("apk").is_dir() {
        eprintln!("skipping: apk/ is not present");
        return None;
    }
    Some(scan_corpus(Path::new("apk")).unwrap_or_else(|err| panic!("failed to scan apk/: {err}")))
}

fn file_name(scan: &rust_java::JarScan) -> String {
    scan.jar.file_name().and_then(|name| name.to_str()).unwrap_or("?").to_string()
}

#[test]
fn corpus_contains_original_and_api_waves() {
    let Some(scans) = corpus_jars() else {
        return;
    };
    let names: Vec<String> = scans.iter().map(file_name).collect();
    let original = names.iter().filter(|name| name.contains("_game_")).count();
    let api = names.iter().filter(|name| name.contains("_api_")).count();

    assert!(
        original >= ORIGINAL_VENDOR_GAMES,
        "expected at least {ORIGINAL_VENDOR_GAMES} original vendor games, found {original}"
    );
    assert_eq!(api, EXTRA_API_GAMES, "expected {EXTRA_API_GAMES} extra API corpus jars, found {api}");
}

#[test]
fn all_local_jars_are_scannable() {
    let Some(scans) = apk_jars() else {
        return;
    };
    assert!(
        scans.len() >= ORIGINAL_VENDOR_GAMES + EXTRA_API_GAMES,
        "expected the local apk tree to include the generated corpus, found {} jars",
        scans.len()
    );

    let mut seen = BTreeSet::new();
    for scan in &scans {
        let path = scan.jar.display().to_string();
        assert!(seen.insert(path.clone()), "duplicate jar path under apk/: {path}");
        if scan.referenced == 0 {
            continue;
        }
    }
}

#[test]
fn all_local_jars_have_full_static_coverage() {
    let Some(scans) = apk_jars() else {
        return;
    };
    let leftovers: Vec<String> = scans
        .iter()
        .filter(|scan| !scan.missing.is_empty())
        .map(|scan| {
            let samples: Vec<String> = scan
                .missing
                .iter()
                .take(6)
                .map(|member| format!("{} {}{}", member.class, member.name, member.descriptor))
                .collect();
            format!("{} ({}): {}", file_name(scan), scan.missing.len(), samples.join(", "))
        })
        .collect();
    assert!(
        leftovers.is_empty(),
        "{} jars still miss API members:\n{}",
        leftovers.len(),
        leftovers.join("\n")
    );
}

#[test]
fn corpus_jar_names_are_unique() {
    let Some(scans) = corpus_jars() else {
        return;
    };
    let mut seen = BTreeSet::new();
    for scan in scans {
        let name = file_name(&scan);
        assert!(seen.insert(name.clone()), "duplicate corpus jar name: {name}");
    }
}

#[test]
fn generated_corpus_has_full_static_coverage() {
    let Some(scans) = corpus_jars() else {
        return;
    };
    for scan in scans {
        let name = file_name(&scan);
        assert!(
            scan.missing.is_empty(),
            "{name} has {} missing API members: {:?}",
            scan.missing.len(),
            scan.missing
                .iter()
                .take(8)
                .map(|member| format!("{} {}{}", member.class, member.name, member.descriptor))
                .collect::<Vec<_>>()
        );
        assert!(
            (scan.static_percent() - 100.0).abs() < f32::EPSILON,
            "{name} static coverage is {:.1}%",
            scan.static_percent()
        );
    }
}

#[test]
fn api_jars_cover_all_families_and_profiles() {
    let Some(scans) = corpus_jars() else {
        return;
    };
    let api: Vec<_> = scans.iter().filter(|scan| file_name(scan).contains("_api_")).collect();
    assert_eq!(api.len(), EXTRA_API_GAMES);

    let mut families = BTreeSet::new();
    let mut profiles = BTreeSet::new();
    for scan in &api {
        let name = file_name(scan);
        if let Some(family) = API_FAMILIES.iter().copied().find(|family| name.contains(&format!("_api_{family}_"))) {
            families.insert(family);
        }
        profiles.insert(scan.profile.as_str());
    }

    for family in API_FAMILIES {
        assert!(families.contains(family), "API corpus missing family {family}");
    }
    for profile in [
        "Generic",
        "Nokia",
        "SonyEricsson",
        "Siemens",
        "Samsung",
        "Motorola",
        "Lg",
        "Vodafone",
        "Sprint",
    ] {
        assert!(profiles.contains(profile), "API corpus missing profile {profile}");
    }
}

#[test]
fn api_jars_reference_expected_runtime_classes() {
    let Some(scans) = corpus_jars() else {
        return;
    };
    let mut counts = BTreeMap::from(EXPECTED_FAMILY_CLASS.map(|(family, _)| (family, 0usize)));
    for scan in scans {
        let name = file_name(&scan);
        for (family, class_name) in EXPECTED_FAMILY_CLASS {
            if !name.contains(&format!("_api_{family}_")) {
                continue;
            }
            assert!(
                scan.referenced_classes.contains(class_name),
                "{name} should reference {class_name}, got {:?}",
                scan.referenced_classes
            );
            *counts.get_mut(family).unwrap() += 1;
        }
    }
    for (family, count) in counts {
        assert!(count > 0, "no API jars found for family {family}");
    }
}

#[test]
fn vendor_filenames_map_to_device_profiles() {
    let Some(scans) = corpus_jars() else {
        return;
    };
    for scan in scans {
        let name = file_name(&scan);
        let expected = if name.contains("siemens") {
            "Siemens"
        } else if name.contains("samsung") {
            "Samsung"
        } else if name.contains("motorola") {
            "Motorola"
        } else if name.contains("vodafone") {
            "Vodafone"
        } else if name.contains("sprint") {
            "Sprint"
        } else if name.contains("_lg_") {
            "Lg"
        } else if name.contains("nokia") {
            "Nokia"
        } else if name.contains("sonyericsson") || name.contains("k800") {
            "SonyEricsson"
        } else {
            "Generic"
        };
        assert_eq!(scan.profile.as_str(), expected, "profile mismatch for {name}");
    }
}

#[tokio::test]
async fn boot_one_api_jar_per_family() {
    let Some(scans) = corpus_jars() else {
        return;
    };
    let mut samples = Vec::new();
    for family in API_FAMILIES {
        let marker = format!("_api_{family}_");
        let sample = scans
            .iter()
            .find(|scan| file_name(scan).contains(&marker))
            .unwrap_or_else(|| panic!("missing API jar for {family}"));
        samples.push((family, sample.jar.clone()));
    }

    for (family, jar) in samples {
        let (name, status) = boot_jar_headless(&jar, Duration::from_secs(8)).await;
        assert_eq!(status, "ok", "{family} failed to boot {name}: {status}");
    }
}

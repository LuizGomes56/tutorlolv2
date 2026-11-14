use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use types::Key;
use wiki::lock::{LockedAbility, LockedItemEffect, WikiLock, latest_archive};

fn temp_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "tutorlolv2-lock-{name}-{}-{unique}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(dir: &Path, file: &str, data: &str) {
    let path = dir.join(file);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, data).unwrap();
}

fn ability(effects: &[&str]) -> LockedAbility {
    LockedAbility {
        name: "Sapling Toss".into(),
        effects: effects.iter().map(|s| s.to_string()).collect(),
    }
}

fn champion_lock(effects: &[&str]) -> WikiLock {
    WikiLock {
        champions: BTreeMap::from([(
            "Maokai".to_string(),
            BTreeMap::from([(Key::E, vec![ability(effects)])]),
        )]),
        ..Default::default()
    }
}

const NEW_E: [&str; 5] = [
    "0: Magic Damage (bb754d6a)",
    "1: Magic Damage per Instance (a9ceee98)",
    "2: Non-Minion Total Magic Damage (6c5d5484)",
    "3: Total Attached Sapling Damage (f40ef3ed)",
    "4: Root Duration (84222325)",
];

#[test]
fn rune_and_item_changes_are_warnings() {
    let dir = temp_dir("runes-items");
    write(
        &dir,
        "runes/aftershock.rs",
        "impl Generator for Aftershock {}",
    );
    write(&dir, "items/lich_bane.rs", "impl Generator for LichBane {}");
    write(&dir, "items/mod.rs", "decl_items!(LichBane);");

    let item = |formula: &str, scalings: &[&str]| LockedItemEffect {
        formula: formula.into(),
        scalings: scalings.iter().map(|s| s.to_string()).collect(),
    };

    let lock = |runes: &[&str], pass: LockedItemEffect| WikiLock {
        runes: BTreeMap::from([(
            "Aftershock".to_string(),
            runes.iter().map(|s| s.to_string()).collect(),
        )]),
        items: BTreeMap::from([(
            "LichBane".to_string(),
            BTreeMap::from([("pass".to_string(), pass)]),
        )]),
        ..Default::default()
    };

    let old = lock(
        &["0: Damage (1)", "1: Resistances (2)"],
        item(
            "aa",
            &["0: Simple BaseAd (1)", "1: Per100 AbilityPower (2)"],
        ),
    );
    let new = lock(
        &[
            "0: Bonus Resistances (3)",
            "1: Damage (1)",
            "2: Resistances (2)",
        ],
        item(
            "bb",
            &["0: Simple BaseAd (9)", "1: Per100 AbilityPower (2)"],
        ),
    );

    let report = new.check(Some(&old), &dir).unwrap();

    // Lock files from before items were tracked have nothing to compare
    let untracked = WikiLock {
        items: BTreeMap::new(),
        ..lock(&["0: Damage (1)", "1: Resistances (2)"], item("", &[]))
    };
    let silent = new.check(Some(&untracked), &dir).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();

    assert!(report.errors.is_empty());
    assert_eq!(report.warnings.len(), 3, "{report:?}");
    assert!(report.warnings[0].starts_with("Aftershock: rune effect indexes changed"));
    assert!(report.warnings[0].contains("    + 0: Bonus Resistances"));
    assert_eq!(report.warnings[1], "LichBane pass: the formula changed");
    assert_eq!(
        report.warnings[2],
        "LichBane pass: 1 scaling values changed"
    );
    assert_eq!(silent.warnings.len(), 1, "{silent:?}");
}

#[test]
fn latest_archive_is_the_highest_other_version() {
    let dir = temp_dir("archive");
    for version in ["16.9.1", "16.21.1", "16.22.1"] {
        write(&dir, &format!("{version}/wiki.lock.json"), "{}");
    }
    write(&dir, "16.30.1/riot/items.json", "{}");

    let latest = latest_archive(&dir, "16.22.1");
    let none = latest_archive(dir.join("missing"), "16.22.1");
    std::fs::remove_dir_all(&dir).unwrap();

    assert_eq!(latest, Some(dir.join("16.21.1/wiki.lock.json")));
    assert_eq!(none, None);
}

#[test]
fn changed_and_missing_exceptions_are_reported() {
    let dir = temp_dir("exceptions");

    let lock = |hash: Option<&str>| WikiLock {
        champions: BTreeMap::from([("Gnar".to_string(), BTreeMap::new())]),
        exceptions: match hash {
            Some(hash) => BTreeMap::from([(
                "Gnar".to_string(),
                BTreeMap::from([("Mega Gnar".to_string(), hash.to_string())]),
            )]),
            None => BTreeMap::new(),
        },
        ..Default::default()
    };

    let unchanged = lock(Some("701997b6"))
        .check(Some(&lock(Some("701997b6"))), &dir)
        .unwrap();
    assert!(unchanged.warnings.is_empty() && unchanged.errors.is_empty());

    let changed = lock(Some("00000000"))
        .check(Some(&lock(Some("701997b6"))), &dir)
        .unwrap();
    assert_eq!(changed.warnings.len(), 1);
    assert!(changed.warnings[0].contains("Gnar exception \"Mega Gnar\": the text changed"));

    // Lock files written before exceptions were tracked have none to compare
    let untracked = lock(Some("701997b6"))
        .check(Some(&lock(None)), &dir)
        .unwrap();
    assert!(untracked.warnings.is_empty());

    // An exception missing from the lock file can't be read by its generator
    let missing = lock(None).check(None, &dir).unwrap();
    assert_eq!(missing.errors.len(), 1);
    assert!(missing.errors[0].contains("Gnar exception \"Mega Gnar\": missing"));

    // Champions that are not in the lock file are not checked
    let unrelated = WikiLock::default().check(None, &dir).unwrap();
    assert!(unrelated.errors.is_empty());

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn map_changes_are_warnings() {
    let dir = temp_dir("map_changes");

    let lock = |entries: &[(&str, &[&str])]| WikiLock {
        map_changes: BTreeMap::from([(
            "aram".to_string(),
            entries
                .iter()
                .map(|(key, changes)| {
                    (
                        key.to_string(),
                        changes.iter().map(|c| c.to_string()).collect(),
                    )
                })
                .collect(),
        )]),
        ..Default::default()
    };

    let old = lock(&[
        ("Akali Q", &["0: Ratio (11111111)"]),
        ("Hubris", &["0: Ignored (22222222)"]),
    ]);
    let new = lock(&[
        ("Akali Q", &["0: Ratio (33333333)"]),
        ("Teemo R", &["0: Base (44444444)", "1: Ignored (55555555)"]),
    ]);

    let report = new.check(Some(&old), &dir).unwrap();

    assert!(report.errors.is_empty());
    assert_eq!(
        report.warnings,
        [
            "aram: 2 damage-relevant changes, see `wiki map-changes-report` for the ones that are not applied",
            "aram Akali Q: 1 changes edited",
            "aram Hubris: changes removed",
            "aram Teemo R: new changes",
        ]
    );

    // Lock files written before map changes were tracked only get the count
    let untracked = new.check(Some(&WikiLock::default()), &dir).unwrap();
    assert_eq!(untracked.warnings.len(), 1);
}

#[test]
fn structured_generator_selections_are_checked_across_pages() {
    let dir = temp_dir("champions-v2");
    write(
        &dir,
        "champions/maokai.rs",
        r#"impl Generator for Maokai {
        fn generate(&mut self) -> MayFail {
            self.take("Magic Damage", E(Void))?
                .take_nth("Magic Damage", 1, E(Min))?
                .take_in("Sapling Toss", "Missing", E(Max))?;
            let _ = self.value("Magic Damage", Key::E)?;
            let _ = self.value_nth("Magic Damage", 0, Key::E)?;
            // self.take("Commented", E(Void))?;
        }
    }"#,
    );
    let report = champion_lock(&NEW_E).check(None, &dir).unwrap();
    assert_eq!(report.errors.len(), 2, "{:?}", report.errors);
    assert!(
        report
            .errors
            .iter()
            .any(|s| s.contains("requested occurrence 1"))
    );
    assert!(report.errors.iter().any(|s| s.contains("Missing")));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn repeated_labels_require_occurrence_or_ability_name() {
    let dir = temp_dir("ambiguous-v2");
    let source = r#"impl Generator for Maokai {
        fn generate(&mut self) -> MayFail {
            self.take("Magic Damage", E(Void))?
                .take_nth("Magic Damage", 1, E(Min))?
                .take_in("Sapling Toss", "Magic Damage", E(Max))?;
            let _ = self.value("Magic Damage", Key::E)?;
            let _ = self.value_nth("Magic Damage", 0, Key::E)?;
        }
    }"#;
    write(&dir, "champions/maokai.rs", source);
    let mut lock = champion_lock(&["0: Magic Damage (a)"]);
    lock.champions
        .get_mut("Maokai")
        .unwrap()
        .get_mut(&Key::E)
        .unwrap()
        .push(LockedAbility {
            name: "Other Form".into(),
            effects: vec!["0: Magic Damage (b)".into()],
        });
    let report = lock.check(None, &dir).unwrap();
    assert_eq!(report.errors.len(), 2, "{:?}", report.errors);
    assert!(
        report
            .errors
            .iter()
            .all(|s| s.contains("matches 2 effects"))
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("champions/maokai.rs")).unwrap(),
        source
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn structured_formula_changes_warn_without_rewriting_labels() {
    let dir = temp_dir("formula-v2");
    write(
        &dir,
        "champions/maokai.rs",
        r#"impl Generator for Maokai {
        fn generate(&mut self) -> MayFail { self.take("Magic Damage", E(Void))?; }
    }"#,
    );
    let old = champion_lock(&["0: Magic Damage (aaaaaaaa)"]);
    let changed = champion_lock(&["0: Magic Damage (bbbbbbbb)"]);
    let report = changed.check(Some(&old), &dir).unwrap();
    assert!(report.errors.is_empty());
    assert_eq!(report.warnings, ["Maokai: 1 champion formulas changed"]);
    let stable = changed.check(Some(&changed), &dir).unwrap();
    assert!(stable.errors.is_empty());
    assert!(stable.warnings.is_empty());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn native_source_changes_are_reported_without_scaling_indices() {
    let dir = temp_dir("native-source");
    let old = WikiLock {
        rune_sources: BTreeMap::from([("GraspOfTheUndying".into(), "old".into())]),
        item_sources: BTreeMap::from([(
            "Heartsteel".into(),
            BTreeMap::from([("Colossal Consumption".into(), "old".into())]),
        )]),
        ..Default::default()
    };
    let new = WikiLock {
        rune_sources: BTreeMap::from([("GraspOfTheUndying".into(), "new".into())]),
        item_sources: BTreeMap::from([(
            "Heartsteel".into(),
            BTreeMap::from([("Colossal Consumption".into(), "new".into())]),
        )]),
        ..Default::default()
    };
    let report = new.check(Some(&old), &dir).unwrap();
    assert!(
        report
            .warnings
            .iter()
            .any(|line| line.contains("GraspOfTheUndying") && line.contains("source"))
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|line| line.contains("Heartsteel") && line.contains("Colossal Consumption"))
    );
    assert!(report.errors.is_empty());
    std::fs::remove_dir_all(dir).unwrap();
}

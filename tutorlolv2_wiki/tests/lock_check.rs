use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tutorlolv2_types::Key;
use tutorlolv2_wiki::{
    lock::{
        LockedAbility, LockedItemEffect, WikiLock,
        check::{AbilityDiff, GeneratorFile},
        latest_archive,
    },
    read_to_string,
};

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

/// Effects of Maokai's E in 16.21.1 and 16.22.1, where `Total Magic Damage`
/// was renamed and `Minion Damage` was removed
const OLD_E: [&str; 5] = [
    "0: Magic Damage (bb754d6a)",
    "1: Magic Damage per Instance (a9ceee98)",
    "2: Minion Damage (bb754d6a)",
    "3: Total Attached Sapling Damage (f40ef3ed)",
    "4: Total Magic Damage (6c5d5484)",
];
const NEW_E: [&str; 5] = [
    "0: Magic Damage (bb754d6a)",
    "1: Magic Damage per Instance (a9ceee98)",
    "2: Non-Minion Total Magic Damage (6c5d5484)",
    "3: Total Attached Sapling Damage (f40ef3ed)",
    "4: Root Duration (84222325)",
];

#[test]
fn diff_matches_renames_by_unique_formula() {
    let diff = ability(&OLD_E).diff(&ability(&NEW_E));

    assert_eq!(
        diff,
        AbilityDiff {
            renames: BTreeMap::from([(
                "Total Magic Damage".into(),
                "Non-Minion Total Magic Damage".into()
            )]),
            removed: vec!["Minion Damage".into()],
            added: vec!["Root Duration".into()],
            formulas: 0,
        }
    );
}

#[test]
fn diff_does_not_rename_effects_without_formulas() {
    // `84222325` is the hash of an empty formula
    let old = ability(&["0: Duration (84222325)"]);
    let new = ability(&["0: Slow (84222325)"]);
    let diff = old.diff(&new);

    assert!(diff.renames.is_empty());
    assert_eq!(diff.removed, ["Duration"]);
    assert_eq!(diff.added, ["Slow"]);
}

#[test]
fn diff_does_not_rename_when_the_formula_is_ambiguous() {
    let old = ability(&["0: A (11111111)"]);
    let new = ability(&["0: B (11111111)", "1: C (11111111)"]);
    let diff = old.diff(&new);

    assert!(diff.renames.is_empty());
    assert_eq!(diff.removed, ["A"]);
}

#[test]
fn effect_refs_are_read_from_every_selection() {
    let source = r#"impl Generator for Smolder {
    fn generate(&mut self) -> MayFail {
        let p = self.formula(Key::P, "Innate [4]")?;
        let s = self.scaling_nth(Key::R, 1, "Bonus")?;
        self.ability(Key::Q, [("Physical Damage", Min), ("Say \"hi\"", Max)])
            .ability_nth(
                1,
                Key::E,
                [
                    ("Magic Damage", Void),
                    // ("Unused Damage", _1),
                ],
            )
            .end()
    }
}"#;

    let file = GeneratorFile {
        path: PathBuf::new(),
        id: "Smolder".into(),
        source: source.into(),
    };

    let refs = file
        .effect_refs()
        .into_iter()
        .map(|r| {
            assert_eq!(&source[r.span.clone()][..1], "\"");
            (r.key, r.nth, r.name)
        })
        .collect::<Vec<_>>();

    assert_eq!(
        refs,
        [
            ("P".to_string(), 0, "Innate [4]".to_string()),
            ("R".into(), 1, "Bonus".into()),
            ("Q".into(), 0, "Physical Damage".into()),
            ("Q".into(), 0, "Say \"hi\"".into()),
            ("E".into(), 1, "Magic Damage".into()),
        ]
    );
}

#[test]
fn renamed_effects_are_fixed_and_removed_ones_reported() {
    let dir = temp_dir("champions");
    let generator = r#"impl Generator for Maokai {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::E,
            [
                ("Magic Damage", Min),
                ("Total Magic Damage", Max),
                ("Minion Damage", _1),
            ],
        )
        .end()
    }
}"#;
    write(&dir, "champions/maokai.rs", generator);

    let old = champion_lock(&OLD_E);
    let new = champion_lock(&NEW_E);
    let report = new.check(Some(&old), &dir).unwrap();
    let fixed = read_to_string(dir.join("champions/maokai.rs")).unwrap();

    // Running again after the fix only reports the effect that was removed
    let again = new.check(Some(&old), &dir).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();

    assert_eq!(
        fixed,
        generator.replace(
            "\"Total Magic Damage\"",
            "\"Non-Minion Total Magic Damage\""
        )
    );
    assert_eq!(report.fixes.len(), 1, "{report:?}");
    assert_eq!(report.errors.len(), 1, "{report:?}");
    assert!(report.errors[0].contains("\"Minion Damage\" no longer exists"));
    assert_eq!(
        report.warnings,
        ["Maokai E[0]: new effect \"Root Duration\""]
    );
    assert!(again.fixes.is_empty());
    assert_eq!(again.errors, report.errors);
}

#[test]
fn without_an_old_lock_only_missing_names_are_reported() {
    let dir = temp_dir("no-old");
    write(
        &dir,
        "champions/maokai.rs",
        r#"impl Generator for Maokai {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::E, [("Magic Damage", Min), ("Total Magic Damage", Max)])
            .ability(Key::R, [("Magic Damage", Void)])
            .end()
    }
}"#,
    );

    let report = champion_lock(&NEW_E).check(None, &dir).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();

    assert!(report.fixes.is_empty());
    assert!(report.warnings.is_empty());
    assert_eq!(report.errors.len(), 2, "{report:?}");
    assert!(report.errors[0].contains("\"Total Magic Damage\" no longer exists"));
    assert!(report.errors[1].contains("Maokai R[0]: the ability no longer exists"));
}

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

    assert!(report.errors.is_empty() && report.fixes.is_empty());
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

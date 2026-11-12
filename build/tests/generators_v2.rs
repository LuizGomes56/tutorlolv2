use {
    tutorlolv2_build_dep::formula_v2 as f,
    tutorlolv2_types::{CtxVar::*, MapMode},
    tutorlolv2_wiki::wikitext::{Formula, Sample},
};

#[test]
fn f_builds_formulas_with_precedence() {
    let s = Sample::new([(Level, 3.0), (BonusAd, 100.0), (Stacks, 2.0)]);

    let darius = f![13 + (Level - 1) + 0.3 * BonusAd];
    assert_eq!(darius.eval(&s), Some(45.0));

    let p = Formula::Const(10.0);
    assert_eq!(f![5 * p].eval(&s), Some(50.0));
    assert_eq!(f![p * 5].eval(&s), Some(50.0));
    assert_eq!(f![-p + 1].eval(&s), Some(-9.0));
    assert_eq!(
        f![p * Stacks * (1.03 + 0.03 * BonusAd / 100)].eval(&s),
        Some(10.0 * 2.0 * 1.06)
    );

    // The closure parameter of `edit` is a `Formula`, used by reference
    let q = Formula::Const(1.0).with_map(MapMode::Aram, Formula::Const(2.0));
    assert_eq!(f![q * 3].eval(&s.on(MapMode::Aram)), Some(6.0));
}

use {
    tutorlolv2_build_dep::generators::parser::champions::Champion,
    tutorlolv2_build_dep::generators_v2::ChampionV2,
    tutorlolv2_types::AbilityId::*,
    tutorlolv2_types::AbilityName::*,
    tutorlolv2_wiki::{
        champions::WikiChampion,
        wikitext::{ChampionModel, Lookup},
    },
};

/// Source of an ability data page of `skill` named `name`, with `fields`
fn data_named(skill: &str, name: &str, fields: &str) -> String {
    format!(
        "{{{{{{{{{{1<noinclude>|Ability data</noinclude>}}}}}}|{name}\n|skill = {skill}\n|damagetype = Magic\n{fields}\n}}}}"
    )
}

/// A `ChampionV2` of a champion named `id` with the ability pages `pages`, built in
/// its own folder
fn champion_v2(id: &str, pages: &[(&str, String)]) -> ChampionV2 {
    let dir = std::env::temp_dir().join(format!("tutorlolv2_v2_{id}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    for (page, src) in pages {
        std::fs::write(dir.join(format!("{page}.lua")), src).unwrap();
    }

    let lookup = Lookup::new("return {}", "return {}").unwrap();
    let model = ChampionModel::from_dir(id, &dir, &lookup).0;

    let data = WikiChampion {
        name: id.into(),
        champion_id: id.into(),
        adaptive_type: Default::default(),
        attack_type: Default::default(),
        positions: Vec::new(),
        stats: Default::default(),
        modifiers: Default::default(),
        wiki_abilities: Default::default(),
        exceptions: Default::default(),
        map_stats: Vec::new(),
    };

    ChampionV2::new(Champion::try_from(data).unwrap(), model)
}

#[test]
fn takes_select_by_label_and_fail_with_the_candidates() {
    let mut c = champion_v2(
        "Takes",
        &[
            (
                "Q0",
                data_named(
                    "Q",
                    "Boomerang Throw",
                    "|leveling = {{st|Physical Damage|{{ap|5 to 165}}}}",
                ),
            ),
            (
                "Q1",
                data_named(
                    "Q",
                    "Boulder Toss",
                    "|leveling = {{st|Physical Damage|{{ap|45 to 225}}}}",
                ),
            ),
            (
                "R0",
                data_named(
                    "R",
                    "Shrapnel",
                    "|leveling = {{st|Magic Damage|{{ap|75 to 175}}}}{{st|Magic Damage|{{ap|150 to 350}}}}",
                ),
            ),
            // Hwei's E1 declares `skill = Q`, so its effects are Q's
            (
                "E1",
                data_named(
                    "Q",
                    "Severing Bolt",
                    "|leveling = {{st|Bolt Damage|{{ap|10 to 50}}}}",
                ),
            ),
        ],
    );

    let ambiguous = c.take("Physical Damage", Q(Max)).err().unwrap().to_string();
    assert!(
        ambiguous.contains("Boulder Toss") && ambiguous.contains("Boomerang Throw"),
        "{ambiguous}"
    );
    assert!(c.take("Missing", Q(Min)).is_err());

    c.take_in("Boulder Toss", "Physical Damage", Q(Mega))
        .unwrap()
        .take_nth("Magic Damage", 1, R(Max))
        .unwrap()
        .take_nth("Magic Damage", 0, R(Min))
        .unwrap()
        .take("Bolt Damage", Q(Void))
        .unwrap();
    assert!(
        c.take_nth("Magic Damage", 0, R(Min)).is_err(),
        "taken twice"
    );

    let s = Sample::new([(QLevel, 1.0), (RLevel, 1.0)]);
    assert_eq!(c.damage(Q(Mega)).unwrap().eval(&s), Some(45.0));
    assert_eq!(c.damage(R(Max)).unwrap().eval(&s), Some(150.0));
    assert_eq!(c.damage(Q(Void)).unwrap().eval(&s), Some(10.0));
}

#[test]
fn edits_apply_to_every_map_arm() {
    let mut c = champion_v2(
        "Edits",
        &[(
            "Q0",
            data_named(
                "Q",
                "Rampage",
                "|leveling = {{st|Physical Damage|{{ap|60 to 140}}}}",
            ),
        )],
    );
    c.take("Physical Damage", Q(Void)).unwrap();

    // A branch as map changes add it, then an edit
    let with_map = c
        .damage(Q(Void))
        .unwrap()
        .with_map(MapMode::Aram, Formula::Const(100.0));
    c.set(Q(Void), with_map).edit(Q(Void), |q| f![q * 2]);
    c.derive(Q(Max), Q(Void), |q| f![q + 1]).unwrap();

    let s = Sample::new([(QLevel, 1.0)]);
    assert_eq!(c.damage(Q(Void)).unwrap().eval(&s), Some(120.0));
    assert_eq!(
        c.damage(Q(Void))
            .unwrap()
            .eval(&s.clone().on(MapMode::Aram)),
        Some(200.0)
    );
    assert_eq!(
        c.damage(Q(Max)).unwrap().eval(&s.on(MapMode::Aram)),
        Some(201.0)
    );
}

use tutorlolv2_build_dep::generators_v2::MapStyle;

#[test]
fn finish_writes_rust_and_doc_from_one_tree() {
    let mut c = champion_v2(
        "Finish",
        &[(
            "Q0",
            data_named(
                "Q",
                "Spell",
                "|leveling = {{st|Magic Damage|{{ap|50 to 90}} {{as|(+ 60% AP)}}}}",
            ),
        )],
    );
    c.take("Magic Damage", Q(Void)).unwrap();
    let branch = c
        .damage(Q(Void))
        .unwrap()
        .with_map(MapMode::Aram, Formula::Const(1.0));
    c.set(Q(Void), branch);

    let champion = c.finish().unwrap();
    let q = &champion.abilities[&Q(Void)];
    assert!(
        q.damage.contains("tutorlolv2_types::MapMode::Aram"),
        "{}",
        q.damage
    );
    assert!(
        champion.docs[&Q(Void)].contains("#[cold] Aram"),
        "{}",
        champion.docs[&Q(Void)]
    );
    assert!(champion.map_damages.is_empty());
    assert_eq!(q.name, "Spell");
    assert_eq!(q.comment, "Magic Damage");
}

#[test]
fn whole_style_writes_one_formula_per_mode() {
    let mut c = champion_v2(
        "Whole",
        &[(
            "Q0",
            data_named(
                "Q",
                "Spell",
                "|leveling = {{st|Magic Damage|{{ap|50 to 90}}}}",
            ),
        )],
    );
    c.take("Magic Damage", Q(Void)).unwrap();
    let branch = c
        .damage(Q(Void))
        .unwrap()
        .with_map(MapMode::Arena, Formula::Const(1.0));
    c.set(Q(Void), branch).map_style(MapStyle::Whole);

    let champion = c.finish().unwrap();
    assert!(!champion.abilities[&Q(Void)].damage.contains("match"));
    assert_eq!(
        champion.map_damages[&Q(Void)],
        vec![(MapMode::Arena, "1.0".to_string())]
    );
}

#[test]
fn zero_damages_are_removed_and_value_paths_survive_cast_f32() {
    let mut c = champion_v2(
        "Zero",
        &[(
            "Q0",
            data_named(
                "Q",
                "Spell",
                "|leveling = {{st|Magic Damage|{{ap|0}}}}{{st|Bonus Damage|{{ap|10}}}}",
            ),
        )],
    );
    c.take("Magic Damage", Q(Min))
        .unwrap()
        .take("Bonus Damage", Q(Max))
        .unwrap();
    let champion = c.finish().unwrap();
    assert!(!champion.abilities.contains_key(&Q(Min)));
    assert!(champion.abilities.contains_key(&Q(Max)));

    let rust = (Formula::Const(0.6).with_map(MapMode::Aram, Formula::Const(0.65))
        * Formula::Var(AbilityPower))
    .rust();
    let cast = tutorlolv2_build_dep::scripts::utils::cast_f32(&rust).unwrap();
    let tokens = cast.replace(' ', "");
    assert!(
        tokens.contains("tutorlolv2_types::MapMode::Aram=>"),
        "{cast}"
    );
    assert!(tokens.contains("0.65f32"), "{cast}");
}

#[test]
fn a_champion_whose_damages_were_never_written_fails() {
    let mut c = champion_v2(
        "NoEnd",
        &[(
            "Q0",
            data_named(
                "Q",
                "Spell",
                "|leveling = {{st|Magic Damage|{{ap|50 to 90}}}}",
            ),
        )],
    );
    c.take("Magic Damage", Q(Void)).unwrap();

    let error = c.into_champion().unwrap_err().to_string();
    assert!(error.contains("end()"), "{error}");
}

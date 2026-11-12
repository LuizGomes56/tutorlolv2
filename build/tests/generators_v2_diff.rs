//! Compares the output of the v2 generator of each ported champion with its v1
//! generator's, and writes `cache/v2_diff.json`. Combos, merges, constants and
//! attributes must be the same. Formula and damage type differences are printed for
//! review, not asserted: a difference is a model improvement, a port mistake or a case
//! `f!` can't write

use {
    serde::Serialize,
    std::collections::BTreeSet,
    tutorlolv2_build_dep::{
        MayFail,
        generators::{
            GeneratorExt,
            impls::champions::champion_gen_fn,
            parser::{
                Parser,
                champions::{Champion, ChampionParser},
            },
        },
        generators_v2::impls::{PORTED, champion_gen_fn_v2},
        project_root,
    },
    tutorlolv2_types::{AbilityId, Attrs, DamageType, MapMode},
    tutorlolv2_wiki::{
        champions::WikiChampion,
        wikitext::{LegacyFormula, Report, Sample},
    },
};

type GeneratorFn = fn(WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>>;

/// Champions whose v2 combos fix v1's, as Vex's cast then recast of R
const COMBOS_CHANGED: &[&str] = &["Vex"];

/// Modes the formulas are compared on
const MODES: [MapMode; 3] = [MapMode::Default, MapMode::Aram, MapMode::Arena];

#[derive(Serialize)]
struct Difference {
    id: AbilityId,
    mode: MapMode,
    v1: String,
    v2: String,
}

#[derive(Serialize)]
struct ChampionDiff {
    champion_id: String,
    differences: Vec<Difference>,
    only_v1: Vec<AbilityId>,
    only_v2: Vec<AbilityId>,
    /// v1's then v2's, for the ids in both
    damage_types: Vec<(AbilityId, DamageType, DamageType)>,
    attributes: Vec<(AbilityId, Attrs, Attrs)>,
    combos_equal: bool,
    merges_equal: bool,
    constants_equal: bool,
}

impl ChampionDiff {
    fn new(v1: &Champion, v2: &Champion) -> Self {
        let ids = |c: &Champion| c.abilities.keys().copied().collect::<BTreeSet<_>>();
        let (ids_v1, ids_v2) = (ids(v1), ids(v2));
        let samples = Sample::series();

        let differences = ids_v1
            .intersection(&ids_v2)
            .flat_map(|&id| MODES.map(|mode| (id, mode)))
            .filter_map(|(id, mode)| {
                // v1 writes a whole formula per mode, v2 branches inside its formula
                let v1_src = v1
                    .map_damages
                    .get(&id)
                    .and_then(|arms| arms.iter().find(|(arm, _)| *arm == mode))
                    .map_or(&v1.abilities[&id].damage, |(_, formula)| formula);
                let v2_src = &v2.abilities[&id].damage;

                let values = |src: &str| {
                    let formula = LegacyFormula::parse(src)?;
                    let values = samples.iter().map(|s| formula.eval(&s.clone().on(mode)));
                    Some(values.collect::<Vec<_>>())
                };

                let equal = match (values(v1_src), values(v2_src)) {
                    (Some(a), Some(b)) => Report::equal(&a, &b),
                    _ => v1_src == v2_src,
                };

                (!equal).then(|| Difference {
                    id,
                    mode,
                    v1: v1_src.clone(),
                    v2: v2_src.clone(),
                })
            })
            .collect();

        let both = ids_v1.intersection(&ids_v2);
        let pair = |id: &AbilityId| (&v1.abilities[id], &v2.abilities[id]);

        let damage_types = both
            .clone()
            .map(|id| (*id, pair(id).0.damage_type, pair(id).1.damage_type))
            .filter(|(_, a, b)| a != b)
            .collect();
        let attributes = both
            .map(|id| (*id, pair(id).0.attributes, pair(id).1.attributes))
            .filter(|(_, a, b)| a != b)
            .collect();

        Self {
            champion_id: v1.champion_id.clone(),
            differences,
            damage_types,
            attributes,
            only_v1: ids_v1.difference(&ids_v2).copied().collect(),
            only_v2: ids_v2.difference(&ids_v1).copied().collect(),
            combos_equal: v1.combo == v2.combo || COMBOS_CHANGED.contains(&v1.champion_id.as_str()),
            merges_equal: v1.merge == v2.merge,
            constants_equal: v1.constants == v2.constants,
        }
    }
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn v2_generators_match_v1() {
    let parser = ChampionParser::new().unwrap();
    let mut diffs = Vec::new();

    // Every champion has a v2 generator
    let missing = parser
        .map()
        .keys()
        .filter(|id| !PORTED.contains(&id.as_str()))
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "champions without a v2 generator: {missing:?}"
    );

    for &champion_id in PORTED {
        let data = parser.map()[champion_id].clone();
        let run = |make: GeneratorFn| make(data.clone()).unwrap().call().unwrap();

        let v1 = run(champion_gen_fn(champion_id).unwrap());
        let v2 = run(champion_gen_fn_v2(champion_id).unwrap());
        diffs.push(ChampionDiff::new(&v1, &v2));
    }

    let path = project_root().join("cache/v2_diff.json");
    std::fs::write(&path, serde_json::to_string_pretty(&diffs).unwrap()).unwrap();

    for diff in &diffs {
        for Difference { id, mode, v1, v2 } in &diff.differences {
            println!(
                "[{}] {id:?} on {mode:?}\n  v1: {v1}\n  v2: {v2}",
                diff.champion_id
            );
        }

        for (id, v1, v2) in &diff.damage_types {
            println!(
                "[{}] {id:?} damage type: v1 {v1:?}, v2 {v2:?}",
                diff.champion_id
            );
        }

        for (id, v1, v2) in &diff.attributes {
            println!(
                "[{}] {id:?} attributes: v1 {v1:?}, v2 {v2:?}",
                diff.champion_id
            );
        }

        if !diff.only_v1.is_empty() || !diff.only_v2.is_empty() {
            println!(
                "[{}] only v1: {:?}, only v2: {:?}",
                diff.champion_id, diff.only_v1, diff.only_v2
            );
        }
    }

    println!("{} champions compared, written to {path:?}", diffs.len());

    let mismatched = diffs
        .iter()
        .filter(|d| {
            !(d.combos_equal
                && d.merges_equal
                && d.constants_equal
                && d.attributes.is_empty()
                && d.only_v1.is_empty()
                && d.only_v2.is_empty())
        })
        .map(|d| d.champion_id.as_str())
        .collect::<Vec<_>>();

    assert!(
        mismatched.is_empty(),
        "combos, merges, constants, attributes or ids differ: {mismatched:?}"
    );
}

/// v2's output of `champion_id`
fn v2_champion(parser: &ChampionParser, champion_id: &str) -> Champion {
    let make: GeneratorFn = champion_gen_fn_v2(champion_id).unwrap();
    make(parser.map()[champion_id].clone())
        .unwrap()
        .call()
        .unwrap()
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn maximum_damages_are_never_below_their_minimum() {
    use tutorlolv2_types::DevMergeData;
    let parser = ChampionParser::new().unwrap();
    let samples = Sample::series();
    let mut below = Vec::new();

    for &champion_id in PORTED {
        let v2 = v2_champion(&parser, champion_id);
        let make: GeneratorFn = champion_gen_fn(champion_id).unwrap();
        let v1 = make(parser.map()[champion_id].clone())
            .unwrap()
            .call()
            .unwrap();

        // Whether some sample has the maximum below the minimum
        let inverted = |c: &Champion, min: &AbilityId, max: &AbilityId| {
            let values = |id: &AbilityId| {
                let formula = LegacyFormula::parse(&c.abilities.get(id)?.damage)?;
                Some(samples.iter().map(|s| formula.eval(s)).collect::<Vec<_>>())
            };

            match (values(min), values(max)) {
                (Some(a), Some(b)) => a.iter().zip(&b).any(|(a, b)| b + 1e-6 < *a),
                _ => false,
            }
        };

        // Only where v1 had them in order: a maximum below its minimum in v1 too is
        // the wiki's naming, as a capped minimum
        for DevMergeData { min, max, .. } in &v2.merge {
            if inverted(&v2, min, max) && !inverted(&v1, min, max) {
                below.push(format!("{champion_id} {max:?} < {min:?}"));
            }
        }
    }

    assert!(below.is_empty(), "{below:#?}");
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn aurelion_sol_breath_burst_is_a_percent_of_stardust() {
    use tutorlolv2_types::{AbilityName::_2, CtxVar::*};
    // "(+ (3.1% Stardust)% of target's maximum health)": 100 Stardust is 3.1%
    let parser = ChampionParser::new().unwrap();
    let v2 = v2_champion(&parser, "AurelionSol");
    let burst = LegacyFormula::parse(&v2.abilities[&AbilityId::Q(_2)].damage).unwrap();
    let s = Sample::new([(QLevel, 1.0), (Stacks, 100.0), (EnemyMaxHealth, 1000.0)]);
    assert!(
        (burst.eval(&s) - (60.0 + 31.0)).abs() < 1e-6,
        "{}",
        burst.eval(&s)
    );
}

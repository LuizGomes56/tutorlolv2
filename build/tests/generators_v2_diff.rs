//! Compares the output of the v2 generator of each ported champion with its v1
//! generator's, and writes `cache/v2_diff.json`. Combos, merges and constants must be
//! the same. Formula differences are printed for review, not asserted: a difference
//! is a model improvement, a port mistake or a case `f!` can't write

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
    tutorlolv2_types::{AbilityId, MapMode},
    tutorlolv2_wiki::{
        champions::WikiChampion,
        wikitext::{LegacyFormula, Report, Sample},
    },
};

type GeneratorFn = fn(WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>>;

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

        Self {
            champion_id: v1.champion_id.clone(),
            differences,
            only_v1: ids_v1.difference(&ids_v2).copied().collect(),
            only_v2: ids_v2.difference(&ids_v1).copied().collect(),
            combos_equal: v1.combo == v2.combo,
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
        .filter(|d| !(d.combos_equal && d.merges_equal && d.constants_equal))
        .map(|d| d.champion_id.as_str())
        .collect::<Vec<_>>();

    assert!(
        mismatched.is_empty(),
        "combos, merges or constants differ: {mismatched:?}"
    );
}

use build::{
    MayFail,
    generators::parser::{Parser, champions::ChampionParser},
    scripts::utils::probe_ratio,
};

#[test]
fn check_simplify() -> MayFail {
    let mut champion = ChampionParser::new()?.run_fn("Samira")?;
    let cext = champion.finish()?;
    let identifiers = &cext.identifiers;

    for merge in &champion.merge {
        let min = champion.get(merge.min)?;
        let max = champion.get(merge.max)?;

        let min_i = champion.indexof(merge.min)?;
        let max_i = champion.indexof(merge.max)?;

        let all_vars = &identifiers[min_i]
            .iter()
            .chain(&identifiers[max_i])
            .copied()
            .collect::<Vec<_>>();

        let Some(k) = probe_ratio(&min.damage, &max.damage, all_vars) else {
            continue;
        };

        println!(
            "{amin:?} & {amax:?} const K ≈ {k:.6}",
            amin = merge.min,
            amax = merge.max,
        );
    }

    Ok(())
}

use tutorlolv2_build_dep::{CPARSER, IPARSER, MayFail, RPARSER, generators::parser::Parser};

fn main() -> MayFail {
    std::env::set_current_dir("..")?;

    println!("Champions");
    CPARSER.progress();
    println!("\nItems");
    IPARSER.progress();
    println!("\nRunes");
    RPARSER.progress();

    CPARSER.run_all();
    IPARSER.run_all();
    RPARSER.run_all();

    unsafe { std::env::set_var("OUT_DIR", "./build_output") };

    tutorlolv2_build_dep::run()
}

#[cfg(test)]
mod tests {
    use tutorlolv2_build_dep::{
        CPARSER, MayFail, generators::parser::Parser, scripts::utils::probe_ratio,
    };

    #[test]
    fn check_simplify() -> MayFail {
        std::env::set_current_dir("..")?;

        let mut champion = CPARSER.run_fn("Samira")?;
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

            let Some(k) = probe_ratio(&min.damage, &max.damage, &all_vars) else {
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
}

use tutorlolv2_build_dep::{BuildContext, MayFail, project_root};

fn main() -> MayFail {
    let mut context = BuildContext::new(project_root(), "build_output", true)?;

    context.export_json = true;

    let mut args = std::env::args_os().skip(1);

    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--out-dir") => {
                let out = args.next().ok_or("--out-dir requires a path")?;
                context.out = context.root.join(out);
            }
            Some("--no-docs") => context.docs = false,
            Some("--no-export") => context.export_json = false,
            Some("--help" | "-h") => {
                println!("Usage: tutorlolv2_build_dep [--out-dir PATH] [--no-docs] [--no-export]");
                return Ok(());
            }
            _ => return Err(format!("Unknown argument: {arg:?}").into()),
        }
    }

    context.run()
}

#[cfg(test)]
mod tests {
    use tutorlolv2_build_dep::{
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
}

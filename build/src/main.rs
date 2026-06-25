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

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use tutorlolv2_build_dep::{
        CPARSER, MayFail, generators::parser::Parser, scripts::utils::simplify,
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

            let min_d_s = &identifiers[champion.indexof(merge.min)?];
            let max_d_s = &identifiers[champion.indexof(merge.max)?];

            // ✅ União de TODAS as variáveis únicas das duas fórmulas
            // Uma variável compartilhada (ex: "J") deve ter o MESMO valor em ambas as fórmulas,
            // mas valores DIFERENTES das outras variáveis (ex: "K").
            let all_vars: Vec<_> = min_d_s
                .iter()
                .chain(max_d_s.iter())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();

            const N_PROBES: usize = 20;
            const MIN_VALID_PROBES: usize = 8;
            const TOLERANCE: f64 = 1e-9;

            // Primes distintos por variável garantem que var_i ≠ var_j em cada probe.
            // Para N_PROBES=20, nunca haverá colisão (colisão ocorreria apenas em probe 997+).
            const PRIMES: [i64; 12] = [
                883, 947, 977, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049,
            ];

            let mut k_candidate: Option<f64> = None;
            let mut is_constant = true;
            let mut valid_probes = 0;

            'probes: for probe_idx in 0..N_PROBES {
                let mut min_d = min.damage.clone();
                let mut max_d = max.damage.clone();

                // ✅ Cada variável recebe um valor INDEPENDENTE neste probe
                for (var_idx, var) in all_vars.iter().enumerate() {
                    let p = PRIMES[var_idx % PRIMES.len()];
                    // val ∈ [3, 999], determinístico, diferente por (probe, variável)
                    let val = ((probe_idx as i64 + 1) * p) % 997 + 3;

                    let val_str = val.to_string();
                    min_d = min_d.replace(var.as_var(), &val_str);
                    max_d = max_d.replace(var.as_var(), &val_str);
                }

                // ✅ Avalia cada fórmula separadamente como f64
                // Após substituição, não há mais variáveis — simplify reduz a um número
                let min_val: f64 = match simplify(&min_d).trim().parse() {
                    Ok(v) => v,
                    Err(_) => continue, // simplify não resolveu — pula este probe
                };
                let max_val: f64 = match simplify(&max_d).trim().parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                // Ponto degenerado: denominador ≈ 0 (domínio inválido — Schwartz-Zippel
                // só se aplica sobre o domínio válido, então simplesmente pulamos)
                if min_val.abs() < 1e-10 {
                    continue;
                }

                let ratio = max_val / min_val;
                valid_probes += 1;

                match k_candidate {
                    None => k_candidate = Some(ratio),
                    Some(k) => {
                        // ✅ Comparação como f64 com tolerância relativa
                        let rel_err = (ratio - k).abs() / (1.0 + k.abs());
                        if rel_err > TOLERANCE {
                            is_constant = false;
                            break 'probes;
                        }
                    }
                }
            }

            // Rejeita se não houve probes suficientes (ex: denominador sempre 0)
            if valid_probes < MIN_VALID_PROBES {
                is_constant = false;
            }

            if is_constant {
                if let Some(k) = k_candidate {
                    println!(
                        "{amin:?} and {amax:?} are in form K * min  (K ≈ {k:.6})",
                        amin = merge.min,
                        amax = merge.max,
                    );
                }
            }
        }

        Ok(())
    }
}

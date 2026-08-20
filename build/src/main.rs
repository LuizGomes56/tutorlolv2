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

    use tutorlolv2_build_dep::libfmt::{Builder, Class, KnownToken, Op};

    #[test]
    fn __test() -> MayFail {
        // let _test_src = tutorlolv2_wiki::read_to_string("../build_output/docs.txt")?;
        // let _test_ir = tutorlolv2_wiki::read("../build_output/ir.bin")?;

        // let mut _test_builder = Builder::new();
        // _test_builder.ops = ir;
        // _test_builder.source = _test_src;
        // render(_test_builder);

        Ok(())
    }

    fn render(builder: Builder) {
        let Builder { source, ops, .. } = builder;
        let mut output = String::new();

        count(&ops);

        macro_rules! push_str {
            ($class:expr, $text:expr) => {{
                let r = format!(r#"<span class="{:?}">{}</span>"#, $class, {
                    let s: &str = $text;
                    s
                });
                output.push_str(&r);
            }};
        }

        let mut i = 0;

        for op in ops {
            match op {
                Op::Raw(len) => {
                    output.push_str(&source[i..i + len as usize]);
                    i += len as usize;
                }
                Op::Span { class, len } => {
                    push_str!(class, &source[i..i + len as usize]);
                    i += len as usize;
                }
                Op::Bracket { class, this } => {
                    push_str!(class, this.into());
                }
                Op::Known(token) => match token {
                    KnownToken::Keyword(v) => push_str!(Class::Keyword, v.into()),
                    KnownToken::Primitive(v) => push_str!(Class::Primitive, v.into()),
                    KnownToken::Control(v) => push_str!(Class::Control, v.into()),
                    KnownToken::Constant(v) => push_str!(Class::Constant, v.into()),
                    KnownToken::Variable(v) => push_str!(Class::Variable, v.into()),
                    KnownToken::Type(v) => push_str!(Class::Type, v.into()),
                    KnownToken::Function(v) => push_str!(Class::Function, v.into()),
                    KnownToken::Macro(v) => push_str!(Class::Macro, v.into()),
                },
                Op::Space(n) => {
                    for _ in 0..n {
                        output.push(' ');
                    }
                }
            }
        }
    }

    fn count(ops: &[Op]) {
        let mut i = 0;

        for op in ops.iter().copied() {
            match op {
                Op::Raw(len) => {
                    i += len as usize;
                }
                Op::Span { len, .. } => {
                    i += len as usize;
                }
                Op::Bracket { .. } => {
                    i += 1;
                }
                Op::Known(token) => match token {
                    KnownToken::Keyword(v) => i += <&str>::from(v).len(),
                    KnownToken::Primitive(v) => i += <&str>::from(v).len(),
                    KnownToken::Control(v) => i += <&str>::from(v).len(),
                    KnownToken::Constant(v) => i += <&str>::from(v).len(),
                    KnownToken::Variable(v) => i += <&str>::from(v).len(),
                    KnownToken::Type(v) => i += <&str>::from(v).len(),
                    KnownToken::Function(v) => i += <&str>::from(v).len(),
                    KnownToken::Macro(v) => i += <&str>::from(v).len(),
                },
                Op::Space(n) => {
                    for _ in 0..n {
                        i += 1;
                    }
                }
            }
        }

        println!("Size: {i}")
    }
}

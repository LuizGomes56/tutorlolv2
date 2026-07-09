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

mod __tests {
    use tutorlolv2_build_dep::libfmt::{
        Bracket, Builder, Class, Keyword, KnownToken, Op, rust_html,
    };
    use tutorlolv2_wiki::MayFail;

    #[test]
    fn __test() -> MayFail {
        assert_eq!("(".parse(), Ok(Bracket::RParen));
        assert_eq!(")".parse(), Ok(Bracket::LParen));
        assert_eq!("[".parse(), Ok(Bracket::RBracket));
        assert_eq!("]".parse(), Ok(Bracket::LBracket));
        assert_eq!("{".parse(), Ok(Bracket::RCurly));
        assert_eq!("}".parse(), Ok(Bracket::LCurly));
        assert_eq!("Self".parse(), Ok(Keyword::SelfUpper));
        assert_eq!("self".parse(), Ok(Keyword::SelfLower));
        assert_eq!(
            KnownToken::check("Self"),
            Some(Op::Known(KnownToken::Keyword(Keyword::SelfUpper)))
        );
        assert!(KnownToken::check("impl").is_some());

        let input = r#"impl Generator for Neeko {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Min), /* Initial Magic Damage */
                (2, _1),  /* Subsequent Magic Damage */
                (3, Max), /* Total Maximum Magic Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Bonus Magic Damage */])
        .ability(Key::E, [(1, Void) /* Magic Damage */])
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .combo([
            Ability(E(Void)),
            Ability(Q(Max)),
            Attack,
            Ability(W(Void)),
            Ability(R(Void)),
        ])?
        .combo([Ability(Q(_1)), Attack, Ability(W(Void))])?
        .end()
    }
}"#;

        // let builder = rust_html(input);
        // tutorlolv2_wiki::write("__source.txt", &builder.source)?;
        // tutorlolv2_wiki::write("__ops.txt", format!("{:#?}", builder.ops))?;
        // render(builder);

        let _test_src = tutorlolv2_wiki::read_to_string("../build_output/docs.txt")?;
        let _test_ir = tutorlolv2_wiki::read("../build_output/ir.bin")?;
        let (ir, _) = bincode::decode_from_slice::<Vec<Op>, bincode::config::Configuration>(
            &_test_ir,
            bincode::config::Configuration::default(),
        )?;
        let mut _test_builder = Builder::new();
        _test_builder.ops = ir;
        _test_builder.source = _test_src;
        render(_test_builder);

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

        let out = format!(
            "<!DOCTYPE html>\n<html>\n<head>\n<title>tutorlolv2</title>\n<style>\n{CSS}\n</style>\n</head>\n<body>\n<pre>\n{output}\n</pre>\n</body>\n</html>"
        );

        tutorlolv2_wiki::write("__render.txt", out).unwrap();
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

    const CSS: &str = r#"
html {
    line-height: 1.5;
    text-size-adjust: 100%;
    tab-size: 4;
    font-family:
        ui-sans-serif,
        system-ui,
        -apple-system,
        BlinkMacSystemFont,
        "Segoe UI",
        Roboto,
        "Helvetica Neue",
        Arial,
        "Noto Sans",
        sans-serif,
        "Apple Color Emoji",
        "Segoe UI Emoji",
        "Segoe UI Symbol",
        "Noto Color Emoji";
    font-feature-settings: normal;
    font-variation-settings: normal;
}

.Control {
    color: #c586c0;
}

.Lifetime,
.Keyword,
.Macro,
.Boolean {
    color: #569cd6;
}

.Primitive,
.Type {
    color: #4ec8b0;
}

.Comment {
    color: #969696;
    background-color: #262626;
    padding: 1px 0;
}

.Function {
    color: #dcdcaa;
}

.Number {
    color: #b3cda8;
}

.Constant,
._i {
    color: #4fc1ff;
}

.Bracket1 {
    color: #ffd700;
}

.Bracket2 {
    color: #da70d6;
}

.Bracket3 {
    color: #189fff;
}

.String {
    white-space: break-spaces;
    color: #ce9178;
}

.Variable {
    color: #9cdcfe;
}

code,
pre {
    color: #d4d4d4;
    line-height: 1.5;
    background: transparent;
    font-family: Consolas, Monaco, "AndaleMono", "UbuntuMono", monospace;
    font-size: 1em;
}

code {
    display: block;
    overflow-x: auto;
}

code pre {
    margin: 0;
    white-space: pre-wrap;
    word-break: break-word;
}

h1,
h2,
h3,
h4,
h5,
h6,
hr,
figure,
p,
pre {
    margin: 0px;
}

body {
    color: #ffffff;
    background-color: #121212;
    margin: 0;
}</style>"#;
}

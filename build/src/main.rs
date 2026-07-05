use serde::de::DeserializeOwned;
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
    use derive_more::{Display, FromStr};
    use std::{ops::Range, sync::LazyLock};
    use strum::{EnumIter, IntoEnumIterator};
    use synoptic::{Highlighter, TokOpt};

    #[derive(Clone, Copy, Debug, Display, FromStr, EnumIter)]
    #[display(rename_all = "lowercase")]
    enum Keyword {
        As,
        Void,
        Pub,
        Use,
        Crate,
        Mut,
        Static,
        Ref,
        Dyn,
        Unsafe,
        Extern,
        Type,
        Super,
        Mod,
        Struct,
        Const,
        Enum,
        Fn,
        Let,
        Impl,
        Trait,
        Where,
        #[display("Self")]
        SelfUpper,
        #[display("self")]
        SelfLower,
    }

    #[derive(Clone, Copy, Debug, Display, FromStr, EnumIter)]
    #[display(rename_all = "lowercase")]
    enum Primitive {
        Bool,
        Usize,
        U8,
        U16,
        U32,
        U64,
        Isize,
        I8,
        I16,
        I32,
        I64,
        F32,
        F64,
        Char,
        Str,
    }

    #[derive(Clone, Copy, Debug, Display, FromStr, EnumIter)]
    #[display(rename_all = "lowercase")]
    enum Control {
        Break,
        Continue,
        Intrinsic,
        Loop,
        Match,
        Return,
        Yield,
        For,
        While,
        If,
        Else,
        In,
        Impossible,
        Unrecognized,
    }

    #[derive(Clone, Copy, Debug, Display, FromStr, EnumIter)]
    enum Constant {
        Some,
        None,
        Melee,
        Ranged,
        Physical,
        Undefined,
        Unknown,
        Unspecified,
        Mixed,
        True,
        Adaptive,
        Onhit,
        OnhitMin,
        OnhitMax,
        Area,
        AreaOnhit,
        AreaOnhitMin,
        AreaOnhitMax,
        Magic,
        Void,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        Min,
        _1Min,
        _2Min,
        _3Min,
        _4Min,
        _5Min,
        _6Min,
        _7Min,
        _8Min,
        Max,
        _1Max,
        _2Max,
        _3Max,
        _4Max,
        _5Max,
        _6Max,
        _7Max,
        _8Max,
        Mega,
        Minion,
        Minion1,
        Minion2,
        Minion3,
        MinionMax,
        Monster,
        Monster1,
        Monster2,
        Monster3,
        Monster4,
        MonsterMax,
    }

    #[derive(Clone, Copy, Debug, Display)]
    enum KnownToken {
        Keyword(Keyword),
        Primitive(Primitive),
        Control(Control),
        Constant(Constant),
    }

    impl KnownToken {
        fn check(text: &str) -> Option<Op> {
            fn parse<T: core::str::FromStr>(
                text: &str,
                constructor: fn(T) -> KnownToken,
                class: Class,
            ) -> Option<(KnownToken, Class)> {
                text.parse::<T>().ok().map(|v| (constructor(v), class))
            }

            None.or_else(|| parse(text, Self::Keyword, Class::Keyword))
                .or_else(|| parse(text, Self::Primitive, Class::Primitive))
                .or_else(|| parse(text, Self::Control, Class::Control))
                .or_else(|| parse(text, Self::Constant, Class::Constant))
                .map(|(token, kind)| Op::Known { kind, token })
        }
    }

    enum Op {
        Span { kind: Class, len: Range<usize> },
        Known { kind: Class, token: KnownToken },
        Raw(Range<usize>),
        Text(Range<usize>),
        Space(u8),
        Indent(u8),
        NewLine,
    }

    #[derive(Clone, Copy, Debug, Display, FromStr, EnumIter)]
    pub enum Class {
        Comment,
        String,
        Lifetime,
        Keyword,
        Control,
        Constant,
        Type,
        Primitive,
        Number,
        Boolean,
        Macro,
        Function,
        Variable,
        Bracket1,
        Bracket2,
        Bracket3,
    }

    impl Class {
        pub const fn bracket(len: usize) -> Self {
            match len % 3 + 1 {
                1 => Class::Bracket1,
                2 => Class::Bracket2,
                3 => Class::Bracket3,
                _ => unreachable!(),
            }
        }
    }

    struct Builder {
        source: String,
        ops: Vec<Op>,
    }

    impl Builder {
        fn new() -> Self {
            Self {
                source: String::new(),
                ops: Vec::new(),
            }
        }

        fn raw(&mut self, text: &str) {
            let start = self.source.len();

            self.source.push_str(text);
            self.ops.push(Op::Raw(start..self.source.len()));
        }

        fn span(&mut self, class: Class, text: &str) {
            if let Some(op) = KnownToken::check(text) {
                return self.ops.push(op);
            }

            let start = self.source.len();

            self.source.push_str(text);
            self.ops.push(Op::Span {
                kind: class,
                len: start..self.source.len(),
            });
        }

        fn space(&mut self, n: u8) {
            self.ops.push(Op::Space(n));
        }

        fn newline(&mut self) {
            self.ops.push(Op::NewLine);
        }
    }

    static RUST_HIGHLIGHTER: LazyLock<Highlighter> = LazyLock::new(|| {
        let mut h = Highlighter::new(4);

        h.bounded("Comment", r"/\*", r"\*/", false);
        h.keyword("Comment", r"//.*$");
        h.bounded_interp("String", "\"", "\"", "\\{", "\\}", true);
        h.keyword("Lifetime", r"'\w+");

        trait RegexAdd: IntoEnumIterator + ToString {
            fn regadd() -> String {
                format!(
                    r"\b({})\b",
                    Self::iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>()
                        .join("|")
                )
            }
        }

        impl<T: IntoEnumIterator + ToString> RegexAdd for T {}

        h.keyword("Keyword", &Keyword::regadd());
        h.keyword("Control", &Control::regadd());
        h.keyword("Constant", r"::[A-Z_][A-Za-z0-9_]*\b");
        h.keyword("Constant", r"\b[A-Z][A-Z0-9_]*\b");
        h.keyword("Constant", &Constant::regadd());
        h.keyword("Type", r"\b[A-Z][a-zA-Z0-9]*\b");
        h.keyword("Primitive", &Primitive::regadd());
        h.keyword("Number", r"\b(?:0x[0-9A-Fa-f_]+|0o[0-7_]+|0b[01_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d[\d_]*)?)(?:[iu](?:8|16|32|64|128|size)|f(?:32|64))?\b");
        h.keyword("Boolean", r"\b(true|false)\b");
        h.keyword("Macro", r"[a-zA-Z_][a-zA-Z0-9_]*!");
        h.keyword("Function", r"\b[a-z][a-zA-Z0-9_]*\(");
        h.keyword("Function", r"\b(zero)\b");
        h.keyword("Function", r"\b([a-z][a-zA-Z0-9_]*)\s*\(");
        h.keyword("Variable", r"\b[a-z][a-zA-Z0-9_]*\b");
        h
    });

    fn highlight(input: &str) -> Builder {
        let code = input.lines().map(str::to_string).collect::<Vec<_>>();

        // let mut h = RUST_HIGHLIGHTER.clone();
        let mut h: Highlighter = todo!();
        h.run(&code);

        let mut b = Builder::new();
        let mut bracket_stack = Vec::new();

        for (i, line) in code.iter().enumerate() {
            for token in h.line(i, line) {
                match token {
                    TokOpt::Some(text, k) if let Ok(kind) = k.parse() => match kind {
                        Class::Variable if text.ends_with("__fn__") => {
                            b.span(Class::Function, &text[..text.len() - "__fn__".len()]);
                        }
                        Class::Function if text.ends_with('(') => {
                            let name = &text[..text.len() - 1];

                            b.span(kind, name);

                            let bracket = Class::bracket(bracket_stack.len());
                            bracket_stack.push(bracket);

                            b.span(bracket, "(");
                        }
                        Class::String => {
                            let mut start = 0;

                            for (i, ch) in text.char_indices() {
                                if ch == '{' || ch == '}' {
                                    if start != i {
                                        b.span(kind, &text[start..i]);
                                    }

                                    b.span(Class::Keyword, &text[i..i + ch.len_utf8()]);
                                    start = i + ch.len_utf8();
                                }
                            }

                            if start != text.len() {
                                b.span(kind, &text[start..]);
                            }
                        }
                        Class::Constant if text.starts_with("::") => {
                            b.raw("::");
                            b.span(Class::Constant, &text[2..]);
                        }
                        Class::Comment => {
                            let txt = text.trim_matches(|c| c == '*' || c == '/').trim();
                            b.span(kind, txt);
                        }
                        _ => b.span(kind, &text),
                    },
                    TokOpt::Some(text, _) | TokOpt::None(text) => {
                        let mut buf = String::new();

                        for ch in text.chars() {
                            match ch {
                                '(' | '[' | '{' => {
                                    if !buf.is_empty() {
                                        b.raw(&buf);
                                        buf.clear();
                                    }

                                    let class = Class::bracket(bracket_stack.len());
                                    bracket_stack.push(class);

                                    b.span(class, ch.encode_utf8(&mut [0; 4]));
                                }
                                ')' | ']' | '}' => {
                                    if !buf.is_empty() {
                                        b.raw(&buf);
                                        buf.clear();
                                    }

                                    let class = bracket_stack.pop().unwrap_or(Class::Bracket1);
                                    b.span(class, ch.encode_utf8(&mut [0; 4]));
                                }
                                _ => buf.push(ch),
                            }
                        }

                        if !buf.is_empty() {
                            b.raw(&buf);
                        }
                    }
                }
            }

            b.newline();
        }

        b
    }
}

use serde::Serialize;
use serde_json::{Serializer, Value, ser::PrettyFormatter};
use std::{io::Cursor, sync::LazyLock};
use synoptic::{Highlighter, TokOpt};

/// Encodes some data using `brotli` at the maximum level, which is 11.
/// Panics if the input is invalid, or if the compression fails
pub fn encode_brotli_11(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut input = Cursor::new(bytes);
    let mut params = brotli::enc::BrotliEncoderParams::default();
    params.quality = 11;
    params.size_hint = bytes.len();
    let _ = brotli::BrotliCompress(&mut input, &mut output, &params);
    output
}

static RUST_HIGHLIGHTER: LazyLock<Highlighter> = LazyLock::new(|| {
    let mut h = Highlighter::new(4);
    // Comment
    h.bounded("_x", r"/\*", r"\*/", false);
    // Comment
    h.keyword("_x", r"//.*$");
    // String
    h.bounded_interp("_s", "\"", "\"", "\\{", "\\}", true);
    // Lifetime
    h.keyword("_l", r"'\w+");

    fn regkw<const N: usize>(values: [&str; N]) -> String {
        format!(r"\b({})\b", values.join("|"))
    }

    h.keyword(
        // Keyword
        "_k",
        &regkw([
            "void", "pub", "use", "crate", "mut", "static", "ref", "dyn", "unsafe", "extern",
            "type", "super", "mod", "struct", "const", "enum", "fn", "let", "impl", "trait",
            "where", "Self", "self",
        ]),
    );
    h.keyword(
        // Control
        "_r",
        &regkw([
            "break",
            "continue",
            "intrinsic",
            "loop",
            "match",
            "return",
            "yield",
            "for",
            "while",
            "match",
            "if",
            "else",
            "as",
            "in",
            "unknown",
            "impossible",
            "unrecognized",
        ]),
    );
    // Constant
    h.keyword("_c", r"::[A-Z_][A-Za-z0-9_]*\b");
    // Constant
    h.keyword("_c", r"\b[A-Z][A-Z0-9_]*\b");
    // Constant
    h.keyword(
        "_c",
        &regkw([
            "Some",
            "None",
            "Melee",
            "Ranged",
            "Physical",
            "Undefined",
            "Unknown",
            "Unspecified",
            "Mixed",
            "True",
            "Adaptive",
            "Onhit",
            "OnhitMin",
            "OnhitMax",
            "Area",
            "AreaOnhit",
            "AreaOnhitMin",
            "AreaOnhitMax",
            "Magic",
            "Void",
            "_1",
            "_2",
            "_3",
            "_4",
            "_5",
            "_6",
            "_7",
            "_8",
            "Min",
            "_1Min",
            "_2Min",
            "_3Min",
            "_4Min",
            "_5Min",
            "_6Min",
            "_7Min",
            "_8Min",
            "Max",
            "_1Max",
            "_2Max",
            "_3Max",
            "_4Max",
            "_5Max",
            "_6Max",
            "_7Max",
            "_8Max",
            "Mega",
            "Minion",
            "Minion1",
            "Minion2",
            "Minion3",
            "MinionMax",
            "Monster",
            "Monster1",
            "Monster2",
            "Monster3",
            "Monster4",
            "MonsterMax",
        ]),
    );
    // Type
    h.keyword("_t", r"\b[A-Z][a-zA-Z0-9]*\b");
    h.keyword(
        // Primitive
        "_p",
        &regkw([
            "bool", "usize", "u8", "u16", "u32", "u64", "isize", "i8", "i16", "i32", "i64", "f32",
            "f64", "char", "str",
        ]),
    );
    h.keyword(
        // Number
        "_n",
        r"\b(?:0x[0-9A-Fa-f_]+|0o[0-7_]+|0b[01_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d[\d_]*)?)(?:[iu](?:8|16|32|64|128|size)|f(?:32|64))?\b",
    );
    // Boolean
    h.keyword("_b", r"\b(true|false)\b");
    // Macro
    h.keyword("_m", r"[a-zA-Z_][a-zA-Z0-9_]*!");
    // Function
    h.keyword("_f", r"\b[a-z][a-zA-Z0-9_]*\(");
    // Function
    h.keyword("_f", r"\b(zero)\b");
    // Function
    h.keyword("_f", r"\b([a-z][a-zA-Z0-9_]*)\s*\(");
    // Variable
    h.keyword("_v", r"\b[a-z][a-zA-Z0-9_]*\b");
    h
});

/// Converts Rust code contained in the input [`str`] to an HTML [`String`]
pub fn rust_html(rust_code: &str) -> String {
    let code = rust_code
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut h = RUST_HIGHLIGHTER.clone();
    h.run(&code);

    let mut bracket_stack: Vec<u8> = Vec::new();
    let mut out = String::new();
    for (i, line) in code.iter().enumerate() {
        let mut line_html = String::new();

        for token in h.line(i, line) {
            match token {
                TokOpt::Some(text, kind) => match kind.as_str() {
                    "_v" if text.ends_with("__fn__") => {
                        let name = &text[..text.len() - "__fn__".len()];
                        line_html.push_str(&format!(r#"<span class="_f">{name}</span>"#));
                    }
                    "_f" if text.ends_with('(') => {
                        let name = &text[..text.len() - 1];
                        line_html.push_str(&format!("<span class=\"{kind}\">{name}</span>"));
                        let c = ((bracket_stack.len() % 3) + 1) as u8;
                        bracket_stack.push(c);
                        line_html.push_str(&format!(r#"<span class="_b{c}">(</span>"#));
                    }
                    "_s" => {
                        let mut buf = String::new();
                        let flush = |buf: &mut String, line_html: &mut String| {
                            if !buf.is_empty() {
                                line_html.push_str(&format!(r#"<span class="_s">{buf}</span>"#));
                                buf.clear();
                            }
                        };
                        for ch in text.chars() {
                            match ch == '{' || ch == '}' {
                                true => {
                                    flush(&mut buf, &mut line_html);
                                    line_html.push_str(&format!(r#"<span class="_k">{ch}</span>"#));
                                }
                                false => {
                                    buf.push(ch);
                                }
                            }
                        }
                        flush(&mut buf, &mut line_html);
                    }
                    "_c" if text.starts_with("::") => {
                        let name = &text[2..];
                        line_html.push_str("::");
                        line_html.push_str(&format!("<span class=\"{kind}\">{name}</span>"));
                    }
                    "_x" => {
                        let text = text.trim_matches(|c| c == '*' || c == '/').trim();
                        line_html.push_str(&format!("<span class=\"{kind}\">{text}</span>"));
                    }
                    kind => {
                        line_html.push_str(&format!("<span class=\"{kind}\">{text}</span>"));
                    }
                },
                TokOpt::None(text) => {
                    for ch in text.chars() {
                        match ch {
                            '(' | '[' | '{' => {
                                let c = ((bracket_stack.len() % 3) + 1) as u8;
                                bracket_stack.push(c);
                                line_html.push_str(&format!(r#"<span class="_b{c}">{ch}</span>"#));
                            }
                            ')' | ']' | '}' => match bracket_stack.pop() {
                                Some(c) => {
                                    line_html
                                        .push_str(&format!(r#"<span class="_b{c}">{ch}</span>"#));
                                }
                                None => {
                                    line_html.push(ch);
                                }
                            },
                            _ => line_html.push(ch),
                        }
                    }
                }
            }
        }

        out.push_str(&line_html);
        out.push('\n');
    }

    format!("<pre>{out}</pre>")
}

static JSON_HIGHLIGHTER: LazyLock<Highlighter> = LazyLock::new(|| {
    let mut h = Highlighter::new(4);

    // String
    h.keyword("_s", r#""(?:[^"\\]|\\.)*""#);
    // Number
    h.keyword("_n", r"-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?");
    // Boolean
    h.keyword("_b", r"\b(?:null|true|false)\b");
    h
});

/// Converts JSON code contained in the input [`str`] to an HTML [`String`]
pub fn json_html(data: &str) -> String {
    let input = json_pretty(data);
    let lines = input.lines().map(str::to_string).collect::<Vec<String>>();

    let mut h = JSON_HIGHLIGHTER.clone();
    h.run(&lines);

    let mut bracket_stack: Vec<u8> = Vec::new();
    let mut out = String::new();

    for (i, line) in lines.iter().enumerate() {
        let mut toks = h.line(i, line).into_iter().collect::<Vec<_>>();
        for k in 0..toks.len() {
            let is_string = matches!(&toks[k], TokOpt::Some(_, kind) if kind == "_s");
            if !is_string {
                continue;
            }

            let mut next_non_ws = None::<char>;
            let mut idx = k + 1;
            while idx < toks.len() {
                match &toks[idx] {
                    TokOpt::None(txt) => {
                        if let Some(ch) = txt.chars().find(|c| !matches!(c, ' ' | '\t')) {
                            next_non_ws = Some(ch);
                            break;
                        }
                    }
                    TokOpt::Some(_, _) => break,
                }
                idx += 1;
            }

            if next_non_ws == Some(':') {
                if let TokOpt::Some(text, _) = &toks[k] {
                    toks[k] = TokOpt::Some(text.clone(), "_v".to_string());
                }
            }
        }

        let mut line_html = String::new();
        for t in toks {
            match t {
                TokOpt::Some(text, kind) => {
                    line_html.push_str(&format!("<span class=\"{kind}\">{text}</span>"));
                }
                TokOpt::None(text) => {
                    for ch in text.chars() {
                        match ch {
                            '{' | '[' | '(' => {
                                let c = ((bracket_stack.len() % 3) + 1) as u8;
                                bracket_stack.push(c);
                                line_html.push_str(&format!(r#"<span class="_b{c}">{ch}</span>"#));
                            }
                            '}' | ']' | ')' => match bracket_stack.pop() {
                                Some(c) => {
                                    line_html
                                        .push_str(&format!(r#"<span class="_b{c}">{ch}</span>"#));
                                }
                                None => {
                                    line_html.push(ch);
                                }
                            },
                            _ => line_html.push(ch),
                        }
                    }
                }
            }
        }

        out.push_str(&line_html);
        out.push('\n');
    }

    format!("<pre>{out}</pre>")
}

/// Converts JSON code to a pretty-printed [`String`]. It does not turn it to HTML
pub fn json_pretty(input: &str) -> String {
    let v: Value = serde_json::from_str(input).unwrap();
    let mut buf = Vec::new();
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    v.serialize(&mut ser).unwrap();
    String::from_utf8(buf).unwrap()
}

use html5minify::minify;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::{Serializer, Value, ser::PrettyFormatter};
use std::{
    io::{Cursor, Write},
    process::{Command, Stdio},
};
use synoptic::{Highlighter, TokOpt};

/// Takes an HTML string as input and minifies it, returning a sequence
/// of bytes. Text defined inside tags `<pre>` and `<code>` are ignored
pub fn minify_html(html: &mut str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut html_cursor = Cursor::new(html.as_bytes());
    minify(&mut html_cursor, &mut result).unwrap();
    result
}

/// Encodes some data using `zstd` at the maximum level, which is 9
/// Panics if the input is invalid, or if the compression fails
pub fn encode_zstd_9(bytes: &[u8]) -> Vec<u8> {
    zstd::encode_all(bytes, 9).unwrap()
}

/// Encodes some data using `brotli` at the maximum level, which is 11.
/// Panics if the input is invalid, or if the compression fails
pub fn encode_brotli_11(bytes: &[u8]) -> Vec<u8> {
    let mut encoder = brotli2::write::BrotliEncoder::new(Vec::new(), 11);
    encoder.write_all(bytes).unwrap();
    encoder.finish().unwrap()
}

/// Converts the input [`str`] to pascal case
/// ```rs
/// assert_eq!(pascal_case("hello world"), "HelloWorld");
/// assert_eq!(pascal_case("Blade of the Ruined King", "BladeOfTheRuinedKing");
/// ```
pub fn pascal_case(input: &str) -> String {
    let mut words = Vec::new();
    let mut cur = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_alphanumeric() {
            cur.push(ch);
        } else if ch == '\'' {
            if let Some(&next) = chars.peek() {
                if (next == 's' || next == 'S') && !cur.is_empty() {
                    chars.next();
                    cur.push('s');
                    continue;
                }
            }

            if !cur.is_empty() {
                words.push(std::mem::take(&mut cur));
            }
        } else {
            if !cur.is_empty() {
                words.push(std::mem::take(&mut cur));
            }
        }
    }

    if !cur.is_empty() {
        words.push(cur);
    }

    let mut out = String::new();
    for w in words {
        let mut it = w.chars();
        if let Some(first) = it.next() {
            for uc in first.to_uppercase() {
                out.push(uc);
            }
            for lc in it.flat_map(|c| c.to_lowercase()) {
                out.push(lc);
            }
        }
    }

    out
}

pub fn to_ssnake(value: &str) -> String {
    fn is_boundary(c: char) -> bool {
        c.is_ascii_whitespace() || (c.is_ascii_punctuation() && c != '\'')
    }
    fn is_ignorable(c: char) -> bool {
        c == '\''
    }
    let mut out = String::with_capacity(value.len() * 2);
    let mut chars = value.chars().peekable();
    let mut prev_was_alpha = false;
    let mut prev_was_upper = false;
    let mut prev_was_digit = false;
    while let Some(c) = chars.next() {
        if is_ignorable(c) {
            continue;
        }
        if is_boundary(c) {
            if !out.is_empty() && !out.ends_with('_') {
                out.push('_');
            }
            prev_was_alpha = false;
            prev_was_upper = false;
            prev_was_digit = false;
            continue;
        }
        let is_upper = c.is_ascii_uppercase();
        let is_alpha = c.is_ascii_alphabetic();
        let is_digit = c.is_ascii_digit();
        let next_is_lower = chars
            .peek()
            .map(|n| n.is_ascii_lowercase())
            .unwrap_or(false);
        let need_us = if out.is_empty() || out.ends_with('_') {
            false
        } else if is_upper {
            (prev_was_alpha && !prev_was_upper)
                || prev_was_digit
                || (prev_was_upper && next_is_lower)
        } else if is_alpha {
            prev_was_digit
        } else if is_digit {
            prev_was_alpha
        } else {
            false
        };
        if need_us && !out.ends_with('_') {
            out.push('_');
        }
        out.push(c.to_ascii_uppercase());
        prev_was_alpha = is_alpha;
        prev_was_upper = is_upper;
        prev_was_digit = is_digit;
    }
    if out.ends_with('_') {
        out.pop();
    }
    out
}

/// Invokes `rustfmt` program to the input [`str`], with some defined `width`,
/// often set to be `80`. Returns the formatted code or an empty [`String`] if
/// it emits an error or warning
pub fn rustfmt(src: &str) -> String {
    let try_run = || -> Result<String, Box<dyn std::error::Error>> {
        let mut child = Command::new("rustfmt")
            .args(&["--emit", "stdout", "--config", "max_width=48"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        child
            .stdin
            .as_mut()
            .ok_or("Failed to open stdin")?
            .write_all(src.as_bytes())?;
        let output = child.wait_with_output()?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    };
    try_run().unwrap_or(src.to_string())
}

static RUST_HIGHLIGHTER: Lazy<Highlighter> = Lazy::new(|| {
    let mut h = Highlighter::new(4);
    h.bounded("comment", r"/\*", r"\*/", false);
    h.keyword("comment", r"//.*$");
    h.bounded_interp("string", "\"", "\"", "\\{", "\\}", true);
    h.keyword("lifetime", r"'\w+");

    fn regkw<const N: usize>(values: [&str; N]) -> String {
        format!(r"\b({})\b", values.join("|"))
    }

    h.keyword(
        "keyword",
        &regkw([
            "void", "pub", "use", "crate", "mut", "static", "ref", "dyn", "unsafe", "extern",
            "type", "super", "mod", "struct", "const", "enum", "fn", "let", "impl", "trait",
            "where", "Self", "self",
        ]),
    );
    h.keyword(
        "control",
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
    h.keyword("constant", r"::[A-Z_][A-Za-z0-9_]*\b");
    h.keyword("constant", r"\b[A-Z]+\b");
    h.keyword(
        "constant",
        &regkw([
            "Some",
            "None",
            "Melee",
            "Ranged",
            "Physical",
            "Undefined",
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
    h.keyword("type", r"\b[A-Z][a-zA-Z0-9_]*\b");
    h.keyword(
        "primitive",
        &regkw([
            "bool", "usize", "u8", "u16", "u32", "u64", "isize", "i8", "i16", "i32", "i64", "f32",
            "f64", "char", "str",
        ]),
    );
    h.keyword(
        "number",
        r"\b(?:0x[0-9A-Fa-f_]+|0o[0-7_]+|0b[01_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d[\d_]*)?)(?:[iu](?:8|16|32|64|128|size)|f(?:32|64))?\b",
    );
    h.keyword("boolean", r"\b(true|false)\b");
    h.keyword("macro", r"[a-zA-Z_][a-zA-Z0-9_]*!");
    h.keyword("function", r"\b[a-z][a-zA-Z0-9_]*\(");
    h.keyword("function", r"\b(zero)\b");
    h.keyword("variable", r"\b[a-z][a-zA-Z0-9_]*\b");
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
                    "function" if text.ends_with('(') => {
                        let name = &text[..text.len() - 1];
                        line_html.push_str(&format!("<span class=\"{kind}\">{name}</span>"));
                        let c = ((bracket_stack.len() % 3) + 1) as u8;
                        bracket_stack.push(c);
                        line_html.push_str(&format!(r#"<span class="bracket_{c}">(</span>"#));
                    }
                    "string" => {
                        let mut buf = String::new();
                        let flush = |buf: &mut String, line_html: &mut String| {
                            if !buf.is_empty() {
                                line_html
                                    .push_str(&format!(r#"<span class="string">{buf}</span>"#));
                                buf.clear();
                            }
                        };
                        for ch in text.chars() {
                            match ch == '{' || ch == '}' {
                                true => {
                                    flush(&mut buf, &mut line_html);
                                    line_html
                                        .push_str(&format!(r#"<span class="keyword">{ch}</span>"#));
                                }
                                false => {
                                    buf.push(ch);
                                }
                            }
                        }
                        flush(&mut buf, &mut line_html);
                    }
                    "constant" if text.starts_with("::") => {
                        let name = &text[2..];
                        line_html.push_str("::");
                        line_html.push_str(&format!("<span class=\"{kind}\">{name}</span>"));
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
                                line_html
                                    .push_str(&format!(r#"<span class="bracket_{c}">{ch}</span>"#));
                            }
                            ')' | ']' | '}' => match bracket_stack.pop() {
                                Some(c) => {
                                    line_html.push_str(&format!(
                                        r#"<span class="bracket_{c}">{ch}</span>"#
                                    ));
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

static JSON_HIGHLIGHTER: Lazy<Highlighter> = Lazy::new(|| {
    let mut h = Highlighter::new(4);

    h.keyword("string", r#""(?:[^"\\]|\\.)*""#);
    h.keyword("number", r"-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?");
    h.keyword("boolean", r"\b(?:null|true|false)\b");
    h
});

/// Converts JSON code contained in the input [`str`] to an HTML [`String`]
pub fn json_html(input: &str) -> String {
    let lines = input.lines().map(str::to_string).collect::<Vec<String>>();

    let mut h = JSON_HIGHLIGHTER.clone();
    h.run(&lines);

    let mut bracket_stack: Vec<u8> = Vec::new();
    let mut out = String::new();

    for (i, line) in lines.iter().enumerate() {
        let mut toks = h.line(i, line).into_iter().collect::<Vec<_>>();
        for k in 0..toks.len() {
            let is_string = matches!(&toks[k], TokOpt::Some(_, kind) if kind == "string");
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
                    toks[k] = TokOpt::Some(text.clone(), "variable".to_string());
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
                                line_html
                                    .push_str(&format!(r#"<span class="bracket_{c}">{ch}</span>"#));
                            }
                            '}' | ']' | ')' => match bracket_stack.pop() {
                                Some(c) => {
                                    line_html.push_str(&format!(
                                        r#"<span class="bracket_{c}">{ch}</span>"#
                                    ));
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

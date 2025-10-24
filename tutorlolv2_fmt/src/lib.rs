use html5minify::minify;
use serde::Serialize;
use serde_json::{Serializer, Value, ser::PrettyFormatter};
use std::{
    io::{Cursor, Write},
    process::{Command, Stdio},
};
use synoptic::{Highlighter, TokOpt};

pub fn minify_html(html: &mut str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut html_cursor = Cursor::new(html.as_bytes());
    minify(&mut html_cursor, &mut result).unwrap();
    result
}

pub fn encode_zstd_9(bytes: &[u8]) -> Vec<u8> {
    zstd::encode_all(bytes, 9).unwrap()
}

pub fn encode_brotli_11(bytes: &[u8]) -> Vec<u8> {
    let mut encoder = brotli2::write::BrotliEncoder::new(Vec::new(), 11);
    encoder.write_all(bytes).unwrap();
    encoder.finish().unwrap()
}

pub fn to_pascal_case(input: &str) -> String {
    let mut words = Vec::new();
    let mut cur = String::new();

    for ch in input.chars() {
        if ch.is_alphanumeric() {
            cur.push(ch);
        } else if !cur.is_empty() {
            words.push(cur);
            cur = String::new();
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

pub fn remove_special_chars(s: &str) -> String {
    s.replace(" ", "")
        .replace("-", "")
        .replace(")", "")
        .replace("(", "")
        .replace("'", "")
        .replace(".", "")
        .replace(",", "")
        .replace(":", "")
}

pub fn invoke_rustfmt(src: &str, width: usize) -> String {
    let try_run = || -> Result<String, Box<dyn std::error::Error>> {
        let mut child = Command::new("rustfmt")
            .args(&[
                "--emit",
                "stdout",
                "--config",
                &format!("max_width={width}"),
            ])
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

pub fn highlight_rust(rust_code: &str) -> String {
    let mut h = Highlighter::new(4);
    h.bounded("comment", r"/\*", r"\*/", false);
    h.keyword("comment", r"//.*$");
    h.bounded_interp("string", "\"", "\"", "\\{", "\\}", true);
    h.keyword("lifetime", r"'\w+");
    h.keyword(
        "keyword",
        r"\b(void|pub|use|crate|mut|static|ref|dyn|unsafe|extern|type|super|mod|struct|const|enum|fn|let|impl|trait|where|loop|Self|self)\b",
    );
    h.keyword(
        "control",
        r"\b(break|continue|intrinsic|match|return|yield|for|while|match|if|else|as|in)\b",
    );

    h.keyword("constant", r"\b[A-Z]+\b");
    h.keyword("constant", r"\b(AbilityHaste|AbilityPower|Armor|ArmorPenetration|MagicPenetration|AttackDamage|AttackSpeed|GoldPer10Seconds|AdaptiveForce|CriticalStrikeChance|CriticalStrikeDamage|Health|LifeSteal|MagicResist|Mana|MoveSpeed|Omnivamp|BaseHealthRegen|BaseManaRegen|Tenacity|HealAndShieldPower|_1|_2|_3|_4|_5|_6|_7|_8|Mega|Max|Min|Minion|Minion1|Minion2|Minion3|MinionMax|Monster|Monster1|Monster2|Monster3|Monster4|MonsterMax|Void|_1Max|_2Max|_3Max|_4Max|_5Max|_6Max|_7Max|_8Max|_1Min|_2Min|_3Min|_4Min|_5Min|_6Min|_7Min|_8Min|Some|None|Top|Jungle|Middle|Bottom|Support|Melee|Ranged|BASIC_ATTACK|CRITICAL_STRIKE|Physical|Magic|Mixed|True|Adaptative|Unknown|Onhit|OnhitMin|OnhitMax)\b");
    h.keyword("type", r"\b[A-Z][a-zA-Z0-9_]*\b");
    h.keyword(
        "primitive",
        r"\b(bool|usize|u8|u16|u32|u64|isize|i8|i16|i32|i64|f32|f64|char|str|tutorlolv2_macros)\b",
    );
    h.keyword("float", r"\b\d+\.?\d*f32\b");
    h.keyword("number", r"\b\d+\.?\d*\b");
    h.keyword("boolean", r"\b(true|false)\b");
    h.keyword("macro", r"[a-zA-Z_][a-zA-Z0-9_]*!");
    h.keyword("function", r"\b[a-z][a-zA-Z0-9_]*\(");
    h.keyword("function", r"\b(zero|generator)\b");
    h.keyword("variable", r"\b[a-z][a-zA-Z0-9_]*\b");

    let code = rust_code
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>();

    h.run(&code);

    let mut out = String::new();
    for (i, line) in code.iter().enumerate() {
        let mut line_html = String::new();
        for token in h.line(i, line) {
            match token {
                TokOpt::Some(text, kind) => {
                    line_html.push_str(&format!("<span class=\"{kind}\">{text}</span>"));
                }
                TokOpt::None(text) => {
                    line_html.push_str(&text);
                }
            }
        }
        out.push_str(&line_html);
        out.push_str("\n");
    }
    format!("<pre>{}</pre>", out)
}

pub fn highlight_json(input: &str) -> String {
    let mut h = Highlighter::new(4);

    h.keyword("string", r#""(?:[^"\\]|\\.)*""#);
    h.keyword("number", r"-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?");
    h.keyword("boolean", r"\b(?:null|true|false)\b");

    let lines = input.lines().map(str::to_string).collect::<Vec<String>>();
    h.run(&lines);

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
                TokOpt::None(text) => line_html.push_str(&text),
            }
        }
        out.push_str(&line_html);
        out.push('\n');
    }

    format!("<pre>{}</pre>", out)
}

pub fn prettify_json(input: &str) -> String {
    let v: Value = serde_json::from_str(input).unwrap();
    let mut buf = Vec::new();
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    v.serialize(&mut ser).unwrap();
    String::from_utf8(buf).unwrap()
}

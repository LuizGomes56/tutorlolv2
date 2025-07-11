use regex::{Captures, Regex};
use std::io::Write;
use std::process::{Command, Stdio};
use synoptic::{Highlighter, TokOpt};

pub(super) fn transform_expr(expr: &str) -> (String, bool) {
    let re_num = Regex::new(r"\b(\d+(\.\d+)?)\b").unwrap();
    let with_f64 = re_num.replace_all(expr, |caps: &Captures| format!("{}f64", &caps[1]));
    let re_up = Regex::new(r"\b([A-Z][A-Z0-9_]*)\b").unwrap();
    let count_ctx = re_up.find_iter(with_f64.as_ref()).count();
    let result = re_up.replace_all(&with_f64, |caps: &Captures| format!("ctx.{}", &caps[1]));
    (result.into_owned(), count_ctx > 0)
}

pub(super) fn invoke_rustfmt(src: &str) -> String {
    let mut child = Command::new("rustfmt")
        .args(&["--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(src.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    String::from_utf8_lossy(&output.stdout).into_owned()
}

pub(super) fn highlight(code_string: &str) -> String {
    let mut h = Highlighter::new(4);
    h.bounded("comment", r"/\*", r"\*/", false);
    h.keyword("comment", r"//.*$");
    h.bounded_interp("string", "\"", "\"", "\\{", "\\}", true);
    h.keyword("lifetime", r"'\w+");
    h.keyword(
        "keyword",
        r"\b(pub|use|crate|mut|static|dyn|unsafe|extern|type|super|mod|struct|const|enum|fn|let|impl|trait|where|loop|Self|self)\b",
    );
    h.keyword(
        "control",
        r"\b(break|continue|intrinsic|match|return|yield|for|while|match|if|else|as|in)\b",
    );
    h.keyword("constant", r"\b[A-Z]+\b");
    h.keyword("type", r"\b[A-Z][a-zA-Z0-9_]*\b");
    h.keyword(
        "primitive",
        r"\b(bool|usize|i32|i64|f64|char|str|generator_macros)\b",
    );
    h.keyword("float", r"\b\d+\.?\d*f64\b");
    h.keyword("number", r"\b\d+\.?\d*\b");
    h.keyword("boolean", r"\b(true|false)\b");
    h.keyword("macro", r"[a-zA-Z_][a-zA-Z0-9_]*!");
    h.keyword("function", r"\b[a-z][a-zA-Z0-9_]*\(");
    h.keyword("variable", r"\b[a-z][a-zA-Z0-9_]*\b");
    h.keyword("punctuation", r"[+\-*/=&^|!:;,<>.\[\]{}()] ");

    let code = code_string
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

#[macro_export]
macro_rules! compress_bytes {
    ($map:expr) => {{
        use std::io::Write;
        let bytes = bincode::serde::encode_to_vec(&$map, bincode::config::standard()).unwrap();
        let mut encoder = brotli2::write::BrotliEncoder::new(Vec::new(), 11);
        encoder.write_all(&bytes).unwrap();
        encoder.finish().unwrap()
    }};
}

pub mod generator_runner;
pub mod phf_champions;
pub mod phf_items;
pub mod phf_meta;
pub mod phf_runes;

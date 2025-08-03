use regex::{Captures, Regex};
use std::{
    fmt,
    io::Write,
    process::{Command, Stdio},
};
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
        .args(&["--emit", "stdout", "--config", "max_width=80"])
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
        r"\b(pub|use|crate|mut|static|ref|dyn|unsafe|extern|type|super|mod|struct|const|enum|fn|let|impl|trait|where|loop|Self|self)\b",
    );
    h.keyword(
        "control",
        r"\b(break|continue|intrinsic|match|return|yield|for|while|match|if|else|as|in)\b",
    );
    h.keyword("constant", r"\b[A-Z]+\b");
    h.keyword("constant", r"\b(Some|None)\b");
    h.keyword("type", r"\b[A-Z][a-zA-Z0-9_]*\b");
    h.keyword(
        "primitive",
        r"\b(bool|usize|u8|u16|u32|u64|isize|i8|i16|i32|i64|f64|char|str|generator_macros)\b",
    );
    h.keyword("float", r"\b\d+\.?\d*f64\b");
    h.keyword("number", r"\b\d+\.?\d*\b");
    h.keyword("boolean", r"\b(true|false)\b");
    h.keyword("macro", r"[a-zA-Z_][a-zA-Z0-9_]*!");
    h.keyword("function", r"\b[a-z][a-zA-Z0-9_]*\(");
    h.keyword("variable", r"\b[a-z][a-zA-Z0-9_]*\b");

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
    ($bytes:expr) => {{
        use std::io::Write;
        let compress_lvl = if cfg!(debug_assertions) { 1 } else { 11 };
        let mut encoder = brotli2::write::BrotliEncoder::new(Vec::new(), compress_lvl);
        encoder.write_all(&$bytes).unwrap();
        encoder.finish().unwrap()
    }};
}

#[macro_export]
macro_rules! init_map {
    (file $type_name:ty, $path:literal) => {{
        let content = std::fs::read_to_string($path).unwrap();
        serde_json::from_str::<$type_name>(&content).unwrap()
    }};
    (dir $type_name:ty, $path:literal) => {{
        let mut map = BTreeMap::<String, $type_name>::new();
        if let Some(dir) = std::fs::read_dir($path).ok() {
            for entry in dir {
                let path = entry.unwrap().path();
                let file_stem = path.file_stem().unwrap();
                let file_name = file_stem.to_str().unwrap();
                let content = std::fs::read_to_string(&path).unwrap();
                let parsed = serde_json::from_str::<$type_name>(&content).unwrap();
                map.insert(file_name.to_owned(), parsed);
            }
        }
        map
    }};
}

pub fn is_valid_math_expression(expr: &str) -> bool {
    if expr.chars().any(|c| c.is_ascii_lowercase()) {
        return false;
    }
    let expr = expr.replace(' ', "");
    let valid_token_re = Regex::new(r"^[\d\.\+\-\*/\(\)A-Z_]+$").unwrap();
    if !valid_token_re.is_match(&expr) {
        return false;
    }

    let mut stack = vec![];
    for c in expr.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                if stack.pop().is_none() {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

#[derive(Debug, Clone)]
enum Expr {
    Num(String),
    Var(String),
    Op(Box<Expr>, char, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_prec(f, 0)
    }
}

impl Expr {
    fn fmt_with_prec(&self, f: &mut fmt::Formatter<'_>, parent_prec: u8) -> fmt::Result {
        let (prec, left, op, right) = match self {
            Expr::Op(left, op, right) => {
                let prec = match op {
                    '+' | '-' => 1,
                    '*' | '/' => 2,
                    _ => 0,
                };
                (prec, left, op, right)
            }
            Expr::Num(s) | Expr::Var(s) => {
                return write!(f, "{}", s);
            }
        };

        let needs_parens = prec < parent_prec;

        if needs_parens {
            write!(f, "(")?;
        }

        left.fmt_with_prec(f, prec)?;
        write!(f, " {} ", op)?;
        right.fmt_with_prec(f, prec + 1)?;

        if needs_parens {
            write!(f, ")")?;
        }

        Ok(())
    }
}

fn parse_expression(tokens: &[String]) -> (Expr, usize) {
    parse_expression_prec(tokens, 0, 0)
}

fn parse_expression_prec(tokens: &[String], min_prec: u8, mut pos: usize) -> (Expr, usize) {
    let mut lhs = parse_primary(tokens, &mut pos);

    while pos < tokens.len() {
        let op = tokens[pos].chars().next().unwrap();
        let prec = match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            _ => break,
        };

        if prec < min_prec {
            break;
        }

        pos += 1;
        let (rhs, new_pos) = parse_expression_prec(tokens, prec + 1, pos);
        pos = new_pos;
        lhs = Expr::Op(Box::new(lhs), op, Box::new(rhs));
    }

    (lhs, pos)
}

fn parse_primary(tokens: &[String], pos: &mut usize) -> Expr {
    let token = &tokens[*pos];
    *pos += 1;
    if token == "(" {
        let (expr, new_pos) = parse_expression(&tokens[*pos..]);
        *pos += new_pos + 1;
        expr
    } else if token.chars().all(|c| c.is_ascii_digit() || c == '.') {
        Expr::Num(token.clone())
    } else {
        Expr::Var(token.clone())
    }
}

fn tokenize(expr: &str) -> Vec<String> {
    let re = Regex::new(r"[A-Z_]+|\d+\.\d+|\d+|[\+\-\*/\(\)]").unwrap();
    re.find_iter(expr).map(|m| m.as_str().to_string()).collect()
}

pub fn clean_math_expr(expr: &str) -> String {
    if !is_valid_math_expression(expr) {
        return "0.0".to_string();
    }
    let tokens = tokenize(expr);
    let (parsed, _) = parse_expression(&tokens);
    parsed.to_string()
}

pub fn clear_suffixes(input: &str) -> String {
    let re_f64 = Regex::new(r"(\d+(?:\.\d+)?)(f64)").unwrap();
    let no_suffix = re_f64.replace_all(input, "$1");
    let re_decimal = Regex::new(r"\d+\.\d+").unwrap();
    re_decimal
        .replace_all(&no_suffix, |caps: &regex::Captures| {
            let full = &caps[0];
            if let Some((whole, decimal)) = full.split_once('.') {
                if decimal.chars().all(|c| c == '0') && decimal.len() <= 10 {
                    whole.to_string()
                } else {
                    full.to_string()
                }
            } else {
                full.to_string()
            }
        })
        .to_string()
}

pub trait JoinNumVec {
    fn join(&self, sep: &str) -> String;
}

macro_rules! join_num_vec_trait_impl {
    ($t:ty) => {
        impl<T: ToString + Copy> JoinNumVec for $t {
            fn join(&self, sep: &str) -> String {
                self.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(sep)
            }
        }
    };
}

join_num_vec_trait_impl!(Vec<T>);

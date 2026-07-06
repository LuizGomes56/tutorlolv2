use {
    derive_more::{Display, FromStr},
    serde::Serialize,
    serde_json::{Serializer, Value, ser::PrettyFormatter},
    std::{io::Cursor, ops::Range, sync::LazyLock},
    strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr},
    synoptic::{Highlighter, TokOpt},
};

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

#[derive(Clone, Copy, Debug, Display, EnumString, EnumIter, IntoStaticStr, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
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
    #[strum(serialize = "Self")]
    SelfUpper,
    #[strum(serialize = "self")]
    SelfLower,
}

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, IntoStaticStr, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Primitive {
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

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, IntoStaticStr, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Control {
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

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, IntoStaticStr, PartialEq)]
pub enum Constant {
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

#[derive(Clone, Copy, Debug, Display, IntoStaticStr, PartialEq)]
pub enum KnownToken {
    Keyword(Keyword),
    Primitive(Primitive),
    Control(Control),
    Constant(Constant),
}

impl KnownToken {
    pub fn check(text: &str) -> Option<Op> {
        fn parse<T: core::str::FromStr>(
            text: &str,
            constructor: fn(T) -> KnownToken,
        ) -> Option<KnownToken> {
            text.parse::<T>().ok().map(constructor)
        }

        None.or_else(|| parse(text, Self::Keyword))
            .or_else(|| parse(text, Self::Primitive))
            .or_else(|| parse(text, Self::Control))
            .or_else(|| parse(text, Self::Constant))
            .map(Op::Known)
    }
}

#[derive(Clone, Copy, Debug, EnumString, IntoStaticStr, PartialEq)]
pub enum Bracket {
    #[strum(serialize = "(")]
    RParen,
    #[strum(serialize = ")")]
    LParen,
    #[strum(serialize = "{")]
    RCurly,
    #[strum(serialize = "}")]
    LCurly,
    #[strum(serialize = "[")]
    RBracket,
    #[strum(serialize = "]")]
    LBracket,
}

#[derive(Clone, Debug, IntoStaticStr, PartialEq)]
pub enum Op {
    Span { class: Class, len: Range<usize> },
    Raw(Range<usize>),
    Bracket { class: Class, this: Bracket },
    Known(KnownToken),
    Space(u8),
    NewLine,
}

#[derive(Clone, Copy, Debug, Display, EnumIter, FromStr, IntoStaticStr, PartialEq)]
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

pub struct Builder {
    pub source: String,
    pub ops: Vec<Op>,
    bracket_stack: Vec<Class>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            ops: Vec::new(),
            bracket_stack: Vec::new(),
        }
    }

    pub fn bracket(&mut self, this: Bracket) {
        let class = match this {
            Bracket::RParen | Bracket::RCurly | Bracket::RBracket => {
                let class = Class::bracket(self.bracket_stack.len());
                self.bracket_stack.push(class);
                class
            }
            Bracket::LParen | Bracket::LCurly | Bracket::LBracket => {
                self.bracket_stack.pop().unwrap_or(Class::Bracket1)
            }
        };
        self.ops.push(Op::Bracket { class, this });
    }

    pub fn raw(&mut self, text: &str) {
        let start = self.source.len();

        self.source.push_str(text);
        self.ops.push(Op::Raw(start..self.source.len()));
    }

    pub fn span(&mut self, class: Class, text: &str) {
        if let Some(op) = KnownToken::check(text) {
            return self.ops.push(op);
        }

        eprintln!("Span::{class:?}({text:?})");

        let start = self.source.len();

        self.source.push_str(text);
        self.ops.push(Op::Span {
            class,
            len: start..self.source.len(),
        });
    }

    pub fn space(&mut self, n: u8) {
        self.ops.push(Op::Space(n));
    }

    pub fn newline(&mut self) {
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

pub fn rust_html(input: &str) -> Builder {
    let code = input.lines().map(str::to_string).collect::<Vec<_>>();

    let mut h = RUST_HIGHLIGHTER.clone();
    h.run(&code);

    let mut b = Builder::new();

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
                        b.bracket(Bracket::RParen);
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
                    let mut spaces = 0u8;

                    macro_rules! flush_buf {
                        () => {
                            if !buf.is_empty() {
                                b.raw(&buf);
                                buf.clear();
                            }
                        };
                    }
                    macro_rules! flush_spaces {
                        () => {
                            if spaces > 0 {
                                b.space(spaces);
                                spaces = 0;
                            }
                        };
                    }

                    for ch in text.chars() {
                        if let Some(bracket) = match ch {
                            '(' => Some(Bracket::RParen),
                            ')' => Some(Bracket::LParen),
                            '{' => Some(Bracket::RCurly),
                            '}' => Some(Bracket::LCurly),
                            '[' => Some(Bracket::RBracket),
                            ']' => Some(Bracket::LBracket),
                            _ => None,
                        } {
                            flush_spaces!();
                            flush_buf!();
                            b.bracket(bracket);
                        } else if ch == ' ' {
                            flush_buf!();
                            spaces += 1;
                        } else {
                            flush_spaces!();
                            buf.push(ch);
                        }
                    }
                    flush_spaces!();
                    flush_buf!();
                }
            }
        }

        b.newline();
    }

    b
}

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
    let mut s = input.trim().to_string();

    if s.starts_with('"') && s.ends_with('"') {
        if let Ok(unescaped) = serde_json::from_str::<String>(&s) {
            s = unescaped;
        }
    }

    let v = {
        let start = input.find("__JSON__").unwrap() + 8;
        let end = input.rfind("__JSON__").unwrap();

        let encoded = &input[start..end];

        let decoded = base64::decode(encoded).unwrap();
        let json_str = String::from_utf8(decoded).unwrap();

        serde_json::from_str::<Value>(&json_str).unwrap()
    };

    let mut buf = Vec::new();
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    v.serialize(&mut ser).unwrap();

    String::from_utf8(buf).unwrap()
}

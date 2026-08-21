//! Compact lossless codec for highlighted Generator source blocks.
//!
//! This module intentionally does not parse Rust semantically. At build time it:
//! - receives the exact rustfmt-produced `&str`;
//! - classifies lexical ranges with `synoptic`;
//! - applies caller-provided class overrides;
//! - counts repeated `(class, text)` atoms globally;
//! - rescans the original build-time strings and writes compact bytecode.
//!
//! No `Vec<Op>`, AST, `GeneratorId`, or source text exists at runtime.
//!
//! Runtime lookup is `(EntityKind, owner_index)`, where `owner_index` is the
//! discriminant/index of `ChampionId`, `ItemId`, or `RuneId` in the consuming
//! crate. Missing owners cost one bit in the presence bitmap and no stream bytes.

use crate::{
    common::EntityKind,
    render::{Class, Highlighter as HtmlHighlighter},
};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt, str,
    sync::LazyLock,
};
use synoptic::{Highlighter as SynopticHighlighter, TokOpt};

pub const GENERATOR_MAGIC: [u8; 4] = *b"GBC1";
pub const GENERATOR_VERSION: u8 = 2;

// Header:
//  0..4   magic
//  4      version
//  5      flags
//  6..8   header len
//  8..10  champion owner count
// 10..12  item owner count
// 12..14  rune owner count
// 14..16  atom count
// 16      direct atom count
// 17      reserved
// 18..20  dictionary data len
// 20..24  stream data len
// 24..26  champion present count
// 26..28  item present count
// 28..30  rune present count
// 30..32  reserved
const HEADER_LEN: usize = 32;
const FLAG_WIDE_OWNER_OFFSETS: u8 = 1 << 0;

// 0x00..=0xdf are one-byte direct dictionary atom ids.
const DIRECT_ATOM_LIMIT: usize = 0xe0;

const OP_ATOM_U16: u8 = 0xe0;
const OP_RAW_U8: u8 = 0xe1;
const OP_RAW_U16: u8 = 0xe2;

// 0xe3..=0xf2 encode Class 0..=15 directly in the opcode.
const OP_SPAN_U8_BASE: u8 = 0xe3;
const OP_SPAN_U8_LAST: u8 = OP_SPAN_U8_BASE + 15;
const OP_SPAN_U16: u8 = 0xf3;

// Bracket classes are reconstructed from nesting depth at runtime.
const OP_LPAREN: u8 = 0xf4;
const OP_RPAREN: u8 = 0xf5;
const OP_LCURLY: u8 = 0xf6;
const OP_RCURLY: u8 = 0xf7;
const OP_LBRACKET: u8 = 0xf8;
const OP_RBRACKET: u8 = 0xf9;

const MAX_CLASS: u8 = Class::Bracket3 as u8;

// ============================================================================
// Public build-time API
// ============================================================================

/// Exact-token class correction supplied by the consuming build script.
///
/// Typical usage:
///
/// ```ignore
/// const GENERATOR_CLASS_OVERRIDES: &[ClassOverride] = &[
///     ClassOverride::new("_1Min", Class::Constant),
///     ClassOverride::new("Physical", Class::Constant),
/// ];
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassOverride {
    pub text: &'static str,
    pub class: Class,
}

impl ClassOverride {
    pub const fn new(text: &'static str, class: Class) -> Self {
        Self { text, class }
    }
}

#[derive(Debug)]
pub enum GeneratorError {
    OwnerOutOfRange {
        kind: EntityKind,
        owner: u16,
        count: u16,
    },
    DuplicateOwner {
        kind: EntityKind,
        owner: u16,
    },
    TokenTooLong(usize),
    TooManyAtoms,
    DictionaryTooLarge,
    StreamTooLarge,
    Corrupt(&'static str),
    Utf8(str::Utf8Error),
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OwnerOutOfRange { kind, owner, count } => {
                write!(
                    f,
                    "{kind:?} owner {owner} is outside table of {count} owners"
                )
            }
            Self::DuplicateOwner { kind, owner } => {
                write!(f, "{kind:?} owner {owner} was inserted twice")
            }
            Self::TokenTooLong(len) => write!(f, "generator token is too long ({len} bytes)"),
            Self::TooManyAtoms => write!(f, "generator dictionary exceeds u16::MAX atoms"),
            Self::DictionaryTooLarge => {
                write!(f, "generator dictionary data exceeds u16::MAX bytes")
            }
            Self::StreamTooLarge => write!(f, "generator stream data exceeds u32::MAX bytes"),
            Self::Corrupt(message) => write!(f, "corrupt generator database: {message}"),
            Self::Utf8(error) => write!(f, "generator database contains invalid utf-8: {error}"),
        }
    }
}

impl std::error::Error for GeneratorError {}

impl From<str::Utf8Error> for GeneratorError {
    fn from(value: str::Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GeneratorStats {
    pub champions_present: usize,
    pub items_present: usize,
    pub runes_present: usize,
    pub source_bytes: usize,
    pub direct_atoms: usize,
    pub extended_atoms: usize,
    pub dictionary_bytes: usize,
    pub presence_bytes: usize,
    pub offset_bytes: usize,
    pub index_bytes: usize,
    pub stream_bytes: usize,
    pub total_bytes: usize,
    pub wide_owner_offsets: bool,
}

pub struct GeneratorBuildOutput {
    pub bytes: Vec<u8>,
    pub stats: GeneratorStats,
}

// ============================================================================
// Generic lexical highlighter
// ============================================================================

static RUST_HIGHLIGHTER: LazyLock<SynopticHighlighter> = LazyLock::new(|| {
    let mut h = SynopticHighlighter::new(4);

    h.bounded("Comment", r"/\*", r"\*/", false);
    h.keyword("Comment", r"//.*$");
    h.bounded_interp("String", "\"", "\"", "\\{", "\\}", true);
    h.keyword("Lifetime", r"'\w+");

    h.keyword(
        "Keyword",
        r"\b(?:as|async|await|const|crate|dyn|enum|extern|fn|impl|let|mod|move|mut|pub|ref|static|struct|trait|type|unsafe|use|where|Self|self|super)\b",
    );
    h.keyword(
        "Control",
        r"\b(?:break|continue|else|for|if|in|loop|match|return|while|yield)\b",
    );

    // Keep these two rules before the generic PascalCase type rule. This mirrors
    // the old libfmt behavior; domain-specific mistakes are fixed by overrides.
    h.keyword("Constant", r"::[A-Z_][A-Za-z0-9_]*\b");
    h.keyword("Constant", r"\b[A-Z][A-Z0-9_]*\b");
    h.keyword("Type", r"\b[A-Z][A-Za-z0-9_]*\b");

    h.keyword(
        "Primitive",
        r"\b(?:bool|char|str|usize|isize|u8|u16|u32|u64|u128|i8|i16|i32|i64|i128|f32|f64)\b",
    );
    h.keyword(
        "Number",
        r"\b(?:0x[0-9A-Fa-f_]+|0o[0-7_]+|0b[01_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d[\d_]*)?)(?:[iu](?:8|16|32|64|128|size)|f(?:32|64))?\b",
    );
    h.keyword("Boolean", r"\b(?:true|false)\b");
    h.keyword("Macro", r"\b[A-Za-z_][A-Za-z0-9_]*!");
    h.keyword("Function", r"\b[a-z_][A-Za-z0-9_]*\s*\(");
    h.keyword("Variable", r"\b[a-z_][A-Za-z0-9_]*\b");

    h
});

#[inline]
fn class_from_name(name: &str) -> Option<Class> {
    Some(match name {
        "Comment" => Class::Comment,
        "String" => Class::String,
        "Lifetime" => Class::Lifetime,
        "Keyword" => Class::Keyword,
        "Control" => Class::Control,
        "Constant" => Class::Constant,
        "Type" => Class::Type,
        "Primitive" => Class::Primitive,
        "Number" => Class::Number,
        "Boolean" => Class::Boolean,
        "Macro" => Class::Macro,
        "Function" => Class::Function,
        "Variable" => Class::Variable,
        _ => return None,
    })
}

type OverrideMap = HashMap<&'static str, Class>;

fn build_override_map(values: &[ClassOverride]) -> OverrideMap {
    let mut result = HashMap::with_capacity(values.len());
    for value in values {
        result.insert(value.text, value.class);
    }
    result
}

#[inline]
fn override_class(overrides: &OverrideMap, text: &str, original: Class) -> Class {
    // Never let an identifier override recolor string/comment contents.
    if matches!(original, Class::String | Class::Comment) {
        original
    } else {
        overrides.get(text).copied().unwrap_or(original)
    }
}

// ============================================================================
// Streaming scanner -- no token/Op IR is created
// ============================================================================

trait ScanSink {
    fn text(&mut self, class: Option<Class>, text: &str) -> Result<(), GeneratorError>;
    fn bracket(&mut self, bracket: u8) -> Result<(), GeneratorError>;
}

fn scan_source(
    source: &str,
    overrides: &OverrideMap,
    sink: &mut impl ScanSink,
) -> Result<(), GeneratorError> {
    // `split('\n')`, unlike `lines()`, preserves whether the source ended with a
    // final newline. Newlines are emitted only between split segments.
    let lines = source.split('\n').map(str::to_owned).collect::<Vec<_>>();

    let mut highlighter = RUST_HIGHLIGHTER.clone();
    highlighter.run(&lines);

    for (line_index, line) in lines.iter().enumerate() {
        for token in highlighter.line(line_index, line) {
            match token {
                TokOpt::Some(text, class_name) => {
                    if let Some(class) = class_from_name(&class_name) {
                        scan_styled(&text, class, overrides, sink)?;
                    } else {
                        scan_raw(&text, overrides, sink)?;
                    }
                }
                TokOpt::None(text) => scan_raw(&text, overrides, sink)?,
            }
        }

        if line_index + 1 != lines.len() {
            sink.text(None, "\n")?;
        }
    }

    Ok(())
}

fn scan_styled(
    text: &str,
    class: Class,
    overrides: &OverrideMap,
    sink: &mut impl ScanSink,
) -> Result<(), GeneratorError> {
    if text.is_empty() {
        return Ok(());
    }

    match class {
        // Synoptic function rule includes the opening parenthesis and may include
        // whitespace immediately before it. Preserve every byte while making only
        // the function name part of the Function span.
        Class::Function if text.ends_with('(') => {
            let before_paren = &text[..text.len() - 1];
            let name = before_paren.trim_end_matches(|c| c == ' ' || c == '\t');
            let spacing = &before_paren[name.len()..];

            if !name.is_empty() {
                sink.text(Some(override_class(overrides, name, Class::Function)), name)?;
            }
            if !spacing.is_empty() {
                sink.text(None, spacing)?;
            }
            sink.bracket(b'(')?;
        }

        // `::Foo` is one synoptic Constant token. Keep the path separator raw.
        Class::Constant if text.starts_with("::") => {
            sink.text(None, "::")?;
            let value = &text[2..];
            if !value.is_empty() {
                sink.text(
                    Some(override_class(overrides, value, Class::Constant)),
                    value,
                )?;
            }
        }

        // Preserve the old interpolation coloring behavior without treating braces
        // inside strings as structural bracket opcodes.
        Class::String => {
            let mut start = 0;
            for (index, ch) in text.char_indices() {
                if ch != '{' && ch != '}' {
                    continue;
                }

                if start != index {
                    sink.text(Some(Class::String), &text[start..index])?;
                }

                let end = index + ch.len_utf8();
                sink.text(Some(Class::Keyword), &text[index..end])?;
                start = end;
            }

            if start != text.len() {
                sink.text(Some(Class::String), &text[start..])?;
            }
        }

        Class::Comment => sink.text(Some(Class::Comment), text)?,

        _ => sink.text(Some(override_class(overrides, text, class)), text)?,
    }

    Ok(())
}

// Longest-first so `..=` is not split as `..` + `=`.
const MULTI_PUNCT: &[&str] = &[
    "<<=", ">>=", "..=", "...", "::", "->", "=>", "&&", "||", "==", "!=", "<=", ">=", "<<", ">>",
    "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "..",
];

fn scan_raw(
    text: &str,
    overrides: &OverrideMap,
    sink: &mut impl ScanSink,
) -> Result<(), GeneratorError> {
    let mut pos = 0;

    while pos < text.len() {
        let rest = &text[pos..];
        let byte = rest.as_bytes()[0];

        if matches!(byte, b'(' | b')' | b'{' | b'}' | b'[' | b']') {
            sink.bracket(byte)?;
            pos += 1;
            continue;
        }

        if byte.is_ascii_whitespace() {
            let start = pos;
            pos += 1;
            while pos < text.len() && text.as_bytes()[pos].is_ascii_whitespace() {
                pos += 1;
            }
            sink.text(None, &text[start..pos])?;
            continue;
        }

        // Split identifiers even if synoptic left them raw. This makes overrides
        // reliable and lets the dictionary discover repeated identifiers itself.
        if byte.is_ascii_alphabetic() || byte == b'_' {
            let start = pos;
            pos += 1;
            while pos < text.len() {
                let b = text.as_bytes()[pos];
                if b.is_ascii_alphanumeric() || b == b'_' {
                    pos += 1;
                } else {
                    break;
                }
            }

            let value = &text[start..pos];
            sink.text(overrides.get(value).copied(), value)?;
            continue;
        }

        if let Some(value) = MULTI_PUNCT
            .iter()
            .copied()
            .find(|value| rest.starts_with(value))
        {
            sink.text(None, value)?;
            pos += value.len();
            continue;
        }

        let ch = rest.chars().next().expect("non-empty remainder");
        let len = ch.len_utf8();
        sink.text(None, &rest[..len])?;
        pos += len;
    }

    Ok(())
}

// ============================================================================
// First pass: corpus frequency table
// ============================================================================

// style 0 = raw; style 1..=16 = Class + 1.
type FrequencyTable = HashMap<u8, HashMap<Box<str>, u32>>;

#[inline]
fn style_id(class: Option<Class>) -> u8 {
    class.map_or(0, |class| class as u8 + 1)
}

struct FrequencySink<'a> {
    values: &'a mut FrequencyTable,
}

impl ScanSink for FrequencySink<'_> {
    fn text(&mut self, class: Option<Class>, text: &str) -> Result<(), GeneratorError> {
        if text.is_empty() {
            return Ok(());
        }

        let style = style_id(class);
        literal_encoded_len(style, text.len())?;

        let values = self.values.entry(style).or_default();
        if let Some(count) = values.get_mut(text) {
            *count = count.saturating_add(1);
        } else {
            values.insert(text.into(), 1);
        }

        Ok(())
    }

    fn bracket(&mut self, _: u8) -> Result<(), GeneratorError> {
        // Brackets already cost one byte and therefore cannot benefit from the atom pool.
        Ok(())
    }
}

// ============================================================================
// Build-time owner database
// ============================================================================

pub struct GeneratorDbBuilder {
    overrides: OverrideMap,
    champions: Vec<Option<String>>,
    items: Vec<Option<String>>,
    runes: Vec<Option<String>>,
    frequency: FrequencyTable,
    source_bytes: usize,
}

impl GeneratorDbBuilder {
    pub fn new(
        champion_count: u16,
        item_count: u16,
        rune_count: u16,
        overrides: &[ClassOverride],
    ) -> Self {
        Self {
            overrides: build_override_map(overrides),
            champions: (0..champion_count).map(|_| None).collect(),
            items: (0..item_count).map(|_| None).collect(),
            runes: (0..rune_count).map(|_| None).collect(),
            frequency: HashMap::new(),
            source_bytes: 0,
        }
    }

    pub fn without_overrides(champion_count: u16, item_count: u16, rune_count: u16) -> Self {
        Self::new(champion_count, item_count, rune_count, &[])
    }

    pub fn push_champion(
        &mut self,
        owner_index: u16,
        source: &str,
    ) -> Result<&mut Self, GeneratorError> {
        self.push(EntityKind::Champion, owner_index, source)
    }

    pub fn push_item(
        &mut self,
        owner_index: u16,
        source: &str,
    ) -> Result<&mut Self, GeneratorError> {
        self.push(EntityKind::Item, owner_index, source)
    }

    pub fn push_rune(
        &mut self,
        owner_index: u16,
        source: &str,
    ) -> Result<&mut Self, GeneratorError> {
        self.push(EntityKind::Rune, owner_index, source)
    }

    /// Inserts the exact build-time source for `(kind, owner_index)`.
    ///
    /// `owner_index` should be the consuming enum's `.index() as u16`.
    /// No generated id is returned or stored.
    pub fn push(
        &mut self,
        kind: EntityKind,
        owner_index: u16,
        source: &str,
    ) -> Result<&mut Self, GeneratorError> {
        let (count, already_present) = match kind {
            EntityKind::Champion => (
                self.champions.len(),
                self.champions
                    .get(owner_index as usize)
                    .is_some_and(Option::is_some),
            ),
            EntityKind::Item => (
                self.items.len(),
                self.items
                    .get(owner_index as usize)
                    .is_some_and(Option::is_some),
            ),
            EntityKind::Rune => (
                self.runes.len(),
                self.runes
                    .get(owner_index as usize)
                    .is_some_and(Option::is_some),
            ),
        };

        if owner_index as usize >= count {
            return Err(GeneratorError::OwnerOutOfRange {
                kind,
                owner: owner_index,
                count: count as u16,
            });
        }

        if already_present {
            return Err(GeneratorError::DuplicateOwner {
                kind,
                owner: owner_index,
            });
        }

        // Pass 1 happens immediately. Only frequency counts and the original String
        // survive; no token stream/IR is retained.
        {
            let mut sink = FrequencySink {
                values: &mut self.frequency,
            };
            scan_source(source, &self.overrides, &mut sink)?;
        }

        self.source_bytes += source.len();

        match kind {
            EntityKind::Champion => self.champions[owner_index as usize] = Some(source.to_owned()),
            EntityKind::Item => self.items[owner_index as usize] = Some(source.to_owned()),
            EntityKind::Rune => self.runes[owner_index as usize] = Some(source.to_owned()),
        }

        Ok(self)
    }

    pub fn finish(self) -> Result<Vec<u8>, GeneratorError> {
        Ok(self.finish_with_stats()?.bytes)
    }

    pub fn finish_with_stats(self) -> Result<GeneratorBuildOutput, GeneratorError> {
        let Self {
            overrides,
            champions,
            items,
            runes,
            frequency,
            source_bytes,
        } = self;

        let champion_count = u16::try_from(champions.len())
            .map_err(|_| GeneratorError::Corrupt("champion owner count overflow"))?;
        let item_count = u16::try_from(items.len())
            .map_err(|_| GeneratorError::Corrupt("item owner count overflow"))?;
        let rune_count = u16::try_from(runes.len())
            .map_err(|_| GeneratorError::Corrupt("rune owner count overflow"))?;

        let champions_present = champions.iter().filter(|v| v.is_some()).count();
        let items_present = items.iter().filter(|v| v.is_some()).count();
        let runes_present = runes.iter().filter(|v| v.is_some()).count();

        let pool = AtomPool::new(frequency)?;

        // Pass 2: rescan exact source and emit final bytecode directly.
        let mut stream_data = Vec::new();
        let champion_index = encode_owner_group(&champions, &overrides, &pool, &mut stream_data)?;
        let item_index = encode_owner_group(&items, &overrides, &pool, &mut stream_data)?;
        let rune_index = encode_owner_group(&runes, &overrides, &pool, &mut stream_data)?;

        if stream_data.len() > u32::MAX as usize {
            return Err(GeneratorError::StreamTooLarge);
        }

        let wide_owner_offsets = stream_data.len() > u16::MAX as usize;
        let owner_offset_width = if wide_owner_offsets { 4 } else { 2 };

        let mut dictionary_data = Vec::new();
        let mut atom_offsets = Vec::with_capacity(pool.values.len() + 1);

        for atom in &pool.values {
            if dictionary_data.len() > u16::MAX as usize {
                return Err(GeneratorError::DictionaryTooLarge);
            }

            atom_offsets.push(dictionary_data.len() as u16);
            dictionary_data.push(atom.style);
            dictionary_data.extend_from_slice(atom.text.as_bytes());
        }

        if dictionary_data.len() > u16::MAX as usize {
            return Err(GeneratorError::DictionaryTooLarge);
        }
        atom_offsets.push(dictionary_data.len() as u16);

        let atom_count =
            u16::try_from(pool.values.len()).map_err(|_| GeneratorError::TooManyAtoms)?;
        let champion_present_u16 = u16::try_from(champions_present)
            .map_err(|_| GeneratorError::Corrupt("champion present count overflow"))?;
        let item_present_u16 = u16::try_from(items_present)
            .map_err(|_| GeneratorError::Corrupt("item present count overflow"))?;
        let rune_present_u16 = u16::try_from(runes_present)
            .map_err(|_| GeneratorError::Corrupt("rune present count overflow"))?;

        let presence_bytes =
            champion_index.presence.len() + item_index.presence.len() + rune_index.presence.len();
        let offset_count =
            champion_index.offsets.len() + item_index.offsets.len() + rune_index.offsets.len();
        let offset_bytes = offset_count * owner_offset_width;
        let index_bytes = presence_bytes + offset_bytes;
        let dictionary_bytes = atom_offsets.len() * 2 + dictionary_data.len();

        let mut writer = BinWriter::with_capacity(
            HEADER_LEN + dictionary_bytes + index_bytes + stream_data.len(),
        );

        writer
            .bytes(&GENERATOR_MAGIC)
            .u8(GENERATOR_VERSION)
            .u8(if wide_owner_offsets {
                FLAG_WIDE_OWNER_OFFSETS
            } else {
                0
            })
            .u16(HEADER_LEN as u16)
            .u16(champion_count)
            .u16(item_count)
            .u16(rune_count)
            .u16(atom_count)
            .u8(pool.direct_count as u8)
            .u8(0)
            .u16(dictionary_data.len() as u16)
            .u32(stream_data.len() as u32)
            .u16(champion_present_u16)
            .u16(item_present_u16)
            .u16(rune_present_u16)
            .u16(0);

        debug_assert_eq!(writer.len(), HEADER_LEN);

        for offset in atom_offsets {
            writer.u16(offset);
        }
        writer.bytes(&dictionary_data);

        writer
            .bytes(&champion_index.presence)
            .bytes(&item_index.presence)
            .bytes(&rune_index.presence);

        write_owner_offsets(&mut writer, &champion_index.offsets, wide_owner_offsets);
        write_owner_offsets(&mut writer, &item_index.offsets, wide_owner_offsets);
        write_owner_offsets(&mut writer, &rune_index.offsets, wide_owner_offsets);

        writer.bytes(&stream_data);

        let bytes = writer.finish();
        let stats = GeneratorStats {
            champions_present,
            items_present,
            runes_present,
            source_bytes,
            direct_atoms: pool.direct_count,
            extended_atoms: pool.values.len() - pool.direct_count,
            dictionary_bytes,
            presence_bytes,
            offset_bytes,
            index_bytes,
            stream_bytes: stream_data.len(),
            total_bytes: bytes.len(),
            wide_owner_offsets,
        };

        Ok(GeneratorBuildOutput { bytes, stats })
    }
}

// ============================================================================
// Atom dictionary
// ============================================================================

#[derive(Debug, Clone)]
struct Atom {
    style: u8,
    text: Box<str>,
}

#[derive(Debug)]
struct AtomCandidate {
    atom: Atom,
    count: u32,
    direct_saving: i128,
    extended_saving: i128,
}

impl AtomCandidate {
    fn new(atom: Atom, count: u32) -> Result<Self, GeneratorError> {
        let literal = literal_encoded_len(atom.style, atom.text.len())? as i128;
        // One u16 offset entry + one style byte + text bytes.
        let dictionary = (2 + 1 + atom.text.len()) as i128;
        let count_i = count as i128;

        Ok(Self {
            direct_saving: count_i * (literal - 1) - dictionary,
            extended_saving: count_i * (literal - 3) - dictionary,
            atom,
            count,
        })
    }
}

struct AtomPool {
    values: Vec<Atom>,
    ids: HashMap<u8, HashMap<Box<str>, u16>>,
    direct_count: usize,
}

impl AtomPool {
    fn new(frequency: FrequencyTable) -> Result<Self, GeneratorError> {
        let mut candidates = Vec::new();

        for (style, values) in frequency {
            for (text, count) in values {
                if count < 2 {
                    continue;
                }

                // The dictionary uses u16 byte offsets. Large literals remain inline.
                if 1 + text.len() > u16::MAX as usize {
                    continue;
                }

                let candidate = AtomCandidate::new(Atom { style, text }, count)?;
                if candidate.direct_saving > 0 || candidate.extended_saving > 0 {
                    candidates.push(candidate);
                }
            }
        }

        candidates.sort_by(|a, b| {
            b.direct_saving
                .cmp(&a.direct_saving)
                .then_with(|| b.count.cmp(&a.count))
                .then_with(|| a.atom.style.cmp(&b.atom.style))
                .then_with(|| a.atom.text.cmp(&b.atom.text))
        });

        let direct_take = candidates
            .iter()
            .take(DIRECT_ATOM_LIMIT)
            .take_while(|candidate| candidate.direct_saving > 0)
            .count();

        let direct_candidates = candidates.drain(..direct_take).collect::<Vec<_>>();

        candidates.retain(|candidate| candidate.extended_saving > 0);
        candidates.sort_by(|a, b| {
            b.extended_saving
                .cmp(&a.extended_saving)
                .then_with(|| b.count.cmp(&a.count))
                .then_with(|| a.atom.style.cmp(&b.atom.style))
                .then_with(|| a.atom.text.cmp(&b.atom.text))
        });

        let mut values = Vec::new();
        let mut dictionary_len = 0usize;

        for candidate in direct_candidates {
            let len = 1 + candidate.atom.text.len();
            if dictionary_len + len <= u16::MAX as usize {
                dictionary_len += len;
                values.push(candidate.atom);
            }
        }

        let direct_count = values.len();

        for candidate in candidates {
            if values.len() >= u16::MAX as usize {
                break;
            }

            let len = 1 + candidate.atom.text.len();
            if dictionary_len + len <= u16::MAX as usize {
                dictionary_len += len;
                values.push(candidate.atom);
            }
        }

        let mut ids = HashMap::<u8, HashMap<Box<str>, u16>>::new();
        for (index, atom) in values.iter().enumerate() {
            ids.entry(atom.style)
                .or_default()
                .insert(atom.text.clone(), index as u16);
        }

        Ok(Self {
            values,
            ids,
            direct_count,
        })
    }

    #[inline]
    fn id(&self, style: u8, text: &str) -> Option<u16> {
        self.ids.get(&style)?.get(text).copied()
    }

    fn encode(
        &self,
        class: Option<Class>,
        text: &str,
        out: &mut Vec<u8>,
    ) -> Result<(), GeneratorError> {
        let style = style_id(class);

        if let Some(id) = self.id(style, text) {
            if (id as usize) < self.direct_count {
                out.push(id as u8);
            } else {
                out.push(OP_ATOM_U16);
                out.extend_from_slice(&id.to_le_bytes());
            }
            return Ok(());
        }

        encode_literal(class, text, out)
    }
}

struct EncoderSink<'a> {
    pool: &'a AtomPool,
    out: &'a mut Vec<u8>,
}

impl ScanSink for EncoderSink<'_> {
    fn text(&mut self, class: Option<Class>, text: &str) -> Result<(), GeneratorError> {
        if text.is_empty() {
            return Ok(());
        }
        self.pool.encode(class, text, self.out)
    }

    fn bracket(&mut self, bracket: u8) -> Result<(), GeneratorError> {
        let opcode = match bracket {
            b'(' => OP_LPAREN,
            b')' => OP_RPAREN,
            b'{' => OP_LCURLY,
            b'}' => OP_RCURLY,
            b'[' => OP_LBRACKET,
            b']' => OP_RBRACKET,
            _ => {
                return Err(GeneratorError::Corrupt(
                    "invalid bracket emitted by scanner",
                ));
            }
        };
        self.out.push(opcode);
        Ok(())
    }
}

fn literal_encoded_len(style: u8, len: usize) -> Result<usize, GeneratorError> {
    if len > u16::MAX as usize {
        return Err(GeneratorError::TokenTooLong(len));
    }

    Ok(match (style == 0, len <= u8::MAX as usize) {
        (_, true) => 2 + len,
        (true, false) => 3 + len,
        (false, false) => 4 + len,
    })
}

fn encode_literal(
    class: Option<Class>,
    text: &str,
    out: &mut Vec<u8>,
) -> Result<(), GeneratorError> {
    let len = text.len();
    literal_encoded_len(style_id(class), len)?;

    match class {
        None if len <= u8::MAX as usize => {
            out.push(OP_RAW_U8);
            out.push(len as u8);
        }
        None => {
            out.push(OP_RAW_U16);
            out.extend_from_slice(&(len as u16).to_le_bytes());
        }
        Some(class) if len <= u8::MAX as usize => {
            out.push(OP_SPAN_U8_BASE + class as u8);
            out.push(len as u8);
        }
        Some(class) => {
            out.push(OP_SPAN_U16);
            out.push(class as u8);
            out.extend_from_slice(&(len as u16).to_le_bytes());
        }
    }

    out.extend_from_slice(text.as_bytes());
    Ok(())
}

// ============================================================================
// Sparse owner index: bitmap + offsets only for present owners
// ============================================================================

struct PackedOwners {
    presence: Vec<u8>,
    offsets: Vec<u32>,
}

fn encode_owner_group(
    values: &[Option<String>],
    overrides: &OverrideMap,
    pool: &AtomPool,
    stream: &mut Vec<u8>,
) -> Result<PackedOwners, GeneratorError> {
    let mut presence = vec![0u8; bitmap_len(values.len())];
    let present = values.iter().filter(|value| value.is_some()).count();
    let mut offsets = Vec::with_capacity(present + 1);

    for (owner, source) in values.iter().enumerate() {
        let Some(source) = source else {
            continue;
        };

        bitmap_set(&mut presence, owner);
        offsets.push(stream_offset(stream.len())?);

        let mut sink = EncoderSink { pool, out: stream };
        scan_source(source, overrides, &mut sink)?;
    }

    offsets.push(stream_offset(stream.len())?);

    Ok(PackedOwners { presence, offsets })
}

#[inline]
fn stream_offset(len: usize) -> Result<u32, GeneratorError> {
    u32::try_from(len).map_err(|_| GeneratorError::StreamTooLarge)
}

#[inline]
fn bitmap_len(count: usize) -> usize {
    (count + 7) / 8
}

#[inline]
fn bitmap_set(bitmap: &mut [u8], index: usize) {
    bitmap[index >> 3] |= 1u8 << (index & 7);
}

#[inline]
fn bitmap_get(bitmap: &[u8], index: usize) -> bool {
    bitmap[index >> 3] & (1u8 << (index & 7)) != 0
}

/// Number of present owners with logical index strictly less than `index`.
fn bitmap_rank(bitmap: &[u8], index: usize) -> usize {
    let full_bytes = index >> 3;
    let remaining = index & 7;

    let mut rank = bitmap[..full_bytes]
        .iter()
        .map(|value| value.count_ones() as usize)
        .sum::<usize>();

    if remaining != 0 {
        let mask = ((1u16 << remaining) - 1) as u8;
        rank += (bitmap[full_bytes] & mask).count_ones() as usize;
    }

    rank
}

fn write_owner_offsets(writer: &mut BinWriter, offsets: &[u32], wide: bool) {
    if wide {
        for &offset in offsets {
            writer.u32(offset);
        }
    } else {
        for &offset in offsets {
            debug_assert!(offset <= u16::MAX as u32);
            writer.u16(offset as u16);
        }
    }
}

// ============================================================================
// Zero-copy runtime database
// ============================================================================

#[derive(Debug, Clone, Copy)]
struct OwnerTableLayout {
    count: u16,
    present: u16,
    presence: usize,
    offsets: usize,
}

#[derive(Debug, Clone, Copy)]
struct GeneratorLayout {
    atom_offsets: usize,
    atom_data: usize,
    champion: OwnerTableLayout,
    item: OwnerTableLayout,
    rune: OwnerTableLayout,
    stream_data: usize,
    owner_offset_width: usize,
}

/// Zero-copy runtime view over `generator.bin`.
#[derive(Clone, Copy)]
pub struct GeneratorDb<'a> {
    bytes: &'a [u8],
    atoms: u16,
    direct_atoms: u8,
    dictionary_data_len: u16,
    stream_data_len: u32,
    layout: GeneratorLayout,
}

impl<'a> GeneratorDb<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self, GeneratorError> {
        Self::parse(bytes)
    }

    pub fn parse(bytes: &'a [u8]) -> Result<Self, GeneratorError> {
        if bytes.len() < HEADER_LEN {
            return Err(GeneratorError::Corrupt("buffer is shorter than header"));
        }
        if bytes[0..4] != GENERATOR_MAGIC {
            return Err(GeneratorError::Corrupt("bad generator magic"));
        }
        if bytes[4] != GENERATOR_VERSION {
            return Err(GeneratorError::Corrupt("unsupported generator version"));
        }

        let flags = bytes[5];
        if flags & !FLAG_WIDE_OWNER_OFFSETS != 0 {
            return Err(GeneratorError::Corrupt("unknown generator flags"));
        }

        let header_len = read_u16_at(bytes, 6)? as usize;
        if header_len != HEADER_LEN {
            return Err(GeneratorError::Corrupt(
                "unexpected generator header length",
            ));
        }

        let champion_count = read_u16_at(bytes, 8)?;
        let item_count = read_u16_at(bytes, 10)?;
        let rune_count = read_u16_at(bytes, 12)?;
        let atoms = read_u16_at(bytes, 14)?;
        let direct_atoms = bytes[16];
        let dictionary_data_len = read_u16_at(bytes, 18)?;
        let stream_data_len = read_u32_at(bytes, 20)?;
        let champion_present = read_u16_at(bytes, 24)?;
        let item_present = read_u16_at(bytes, 26)?;
        let rune_present = read_u16_at(bytes, 28)?;

        if direct_atoms as usize > DIRECT_ATOM_LIMIT || direct_atoms as u16 > atoms {
            return Err(GeneratorError::Corrupt("invalid direct atom count"));
        }
        if champion_present > champion_count
            || item_present > item_count
            || rune_present > rune_count
        {
            return Err(GeneratorError::Corrupt(
                "present owner count exceeds owner count",
            ));
        }

        let owner_offset_width = if flags & FLAG_WIDE_OWNER_OFFSETS != 0 {
            4
        } else {
            2
        };

        let atom_offsets = HEADER_LEN;
        let atom_data = checked_add(atom_offsets, checked_mul(atoms as usize + 1, 2)?)?;

        let champion_presence = checked_add(atom_data, dictionary_data_len as usize)?;
        let item_presence = checked_add(champion_presence, bitmap_len(champion_count as usize))?;
        let rune_presence = checked_add(item_presence, bitmap_len(item_count as usize))?;

        let champion_offsets = checked_add(rune_presence, bitmap_len(rune_count as usize))?;
        let item_offsets = checked_add(
            champion_offsets,
            checked_mul(champion_present as usize + 1, owner_offset_width)?,
        )?;
        let rune_offsets = checked_add(
            item_offsets,
            checked_mul(item_present as usize + 1, owner_offset_width)?,
        )?;
        let stream_data = checked_add(
            rune_offsets,
            checked_mul(rune_present as usize + 1, owner_offset_width)?,
        )?;
        let end = checked_add(stream_data, stream_data_len as usize)?;

        if end != bytes.len() {
            return Err(GeneratorError::Corrupt(
                "generator section layout does not match buffer length",
            ));
        }

        let db = Self {
            bytes,
            atoms,
            direct_atoms,
            dictionary_data_len,
            stream_data_len,
            layout: GeneratorLayout {
                atom_offsets,
                atom_data,
                champion: OwnerTableLayout {
                    count: champion_count,
                    present: champion_present,
                    presence: champion_presence,
                    offsets: champion_offsets,
                },
                item: OwnerTableLayout {
                    count: item_count,
                    present: item_present,
                    presence: item_presence,
                    offsets: item_offsets,
                },
                rune: OwnerTableLayout {
                    count: rune_count,
                    present: rune_present,
                    presence: rune_presence,
                    offsets: rune_offsets,
                },
                stream_data,
                owner_offset_width,
            },
        };

        db.validate()?;
        Ok(db)
    }

    #[inline]
    pub fn champion_count(&self) -> u16 {
        self.layout.champion.count
    }

    #[inline]
    pub fn item_count(&self) -> u16 {
        self.layout.item.count
    }

    #[inline]
    pub fn rune_count(&self) -> u16 {
        self.layout.rune.count
    }

    #[inline]
    pub fn atom_count(&self) -> u16 {
        self.atoms
    }

    #[inline]
    pub fn direct_atom_count(&self) -> u8 {
        self.direct_atoms
    }

    pub fn has_generator(
        &self,
        kind: EntityKind,
        owner_index: u16,
    ) -> Result<bool, GeneratorError> {
        let table = self.owner_table(kind);
        self.validate_owner_index(kind, table, owner_index)?;
        let bitmap = self.owner_bitmap(table)?;
        Ok(bitmap_get(bitmap, owner_index as usize))
    }

    pub fn entity_bytes(
        &self,
        kind: EntityKind,
        owner_index: u16,
    ) -> Result<Option<&'a [u8]>, GeneratorError> {
        let table = self.owner_table(kind);
        self.validate_owner_index(kind, table, owner_index)?;

        let bitmap = self.owner_bitmap(table)?;
        if !bitmap_get(bitmap, owner_index as usize) {
            return Ok(None);
        }

        let ordinal = bitmap_rank(bitmap, owner_index as usize);
        if ordinal >= table.present as usize {
            return Err(GeneratorError::Corrupt(
                "owner bitmap rank exceeds present count",
            ));
        }

        let start = self.owner_offset(table, ordinal)? as usize;
        let end = self.owner_offset(table, ordinal + 1)? as usize;

        if end < start || end > self.stream_data_len as usize {
            return Err(GeneratorError::Corrupt("bad generator owner byte range"));
        }

        self.bytes
            .get(self.layout.stream_data + start..self.layout.stream_data + end)
            .map(Some)
            .ok_or(GeneratorError::Corrupt(
                "generator owner slice out of bounds",
            ))
    }

    pub fn render_plain(
        &self,
        kind: EntityKind,
        owner_index: u16,
    ) -> Result<Option<String>, GeneratorError> {
        let Some(bytes) = self.entity_bytes(kind, owner_index)? else {
            return Ok(None);
        };

        let mut emit = |out: &mut String, _class: Option<Class>, text: &str| out.push_str(text);

        self.render_with(bytes, &mut emit).map(Some)
    }

    pub fn render_html(
        &self,
        kind: EntityKind,
        owner_index: u16,
    ) -> Result<Option<String>, GeneratorError> {
        let Some(bytes) = self.entity_bytes(kind, owner_index)? else {
            return Ok(None);
        };

        let mut emit = |out: &mut String, class: Option<Class>, text: &str| match class {
            Some(class) => HtmlHighlighter::push_span(out, class, text),
            None => HtmlHighlighter::push_escaped(out, text),
        };

        self.render_with(bytes, &mut emit).map(Some)
    }

    fn render_with(
        &self,
        bytes: &[u8],
        emit: &mut dyn FnMut(&mut String, Option<Class>, &str),
    ) -> Result<String, GeneratorError> {
        let mut renderer = GeneratorRenderer {
            db: self,
            emit,
            bracket_stack: Vec::new(),
        };
        let mut cursor = GeneratorCursor::new(bytes);
        let mut out = String::new();

        renderer.render(&mut cursor, &mut out)?;

        if !cursor.is_eof() {
            return Err(GeneratorError::Corrupt(
                "generator owner has trailing bytes",
            ));
        }

        // Do not require balanced brackets. The codec is lexical/display-only and
        // intentionally accepts arbitrary text, just like the old libfmt path.
        Ok(out)
    }

    #[inline]
    fn owner_table(&self, kind: EntityKind) -> OwnerTableLayout {
        match kind {
            EntityKind::Champion => self.layout.champion,
            EntityKind::Item => self.layout.item,
            EntityKind::Rune => self.layout.rune,
        }
    }

    fn validate_owner_index(
        &self,
        kind: EntityKind,
        table: OwnerTableLayout,
        owner_index: u16,
    ) -> Result<(), GeneratorError> {
        if owner_index >= table.count {
            return Err(GeneratorError::OwnerOutOfRange {
                kind,
                owner: owner_index,
                count: table.count,
            });
        }
        Ok(())
    }

    fn owner_bitmap(&self, table: OwnerTableLayout) -> Result<&'a [u8], GeneratorError> {
        let len = bitmap_len(table.count as usize);
        self.bytes
            .get(table.presence..table.presence + len)
            .ok_or(GeneratorError::Corrupt(
                "owner presence bitmap out of bounds",
            ))
    }

    fn owner_offset(&self, table: OwnerTableLayout, ordinal: usize) -> Result<u32, GeneratorError> {
        if ordinal > table.present as usize {
            return Err(GeneratorError::Corrupt("owner offset ordinal out of range"));
        }

        let pos = table
            .offsets
            .checked_add(ordinal.checked_mul(self.layout.owner_offset_width).ok_or(
                GeneratorError::Corrupt("owner offset multiplication overflow"),
            )?)
            .ok_or(GeneratorError::Corrupt("owner offset overflow"))?;

        if self.layout.owner_offset_width == 2 {
            Ok(read_u16_at(self.bytes, pos)? as u32)
        } else {
            read_u32_at(self.bytes, pos)
        }
    }

    fn atom(&self, id: u16) -> Result<AtomView<'a>, GeneratorError> {
        if id >= self.atoms {
            return Err(GeneratorError::Corrupt("atom id out of range"));
        }

        let start = self.atom_offset(id)? as usize;
        let end = self.atom_offset(id + 1)? as usize;
        if end <= start || end > self.dictionary_data_len as usize {
            return Err(GeneratorError::Corrupt("bad atom range"));
        }

        let entry = self
            .bytes
            .get(self.layout.atom_data + start..self.layout.atom_data + end)
            .ok_or(GeneratorError::Corrupt("atom slice out of bounds"))?;

        let style = entry[0];
        let class = match style {
            0 => None,
            1..=16 => Some(class_from_u8(style - 1)?),
            _ => return Err(GeneratorError::Corrupt("invalid atom style")),
        };
        let text = str::from_utf8(&entry[1..])?;

        Ok(AtomView { class, text })
    }

    fn atom_offset(&self, index: u16) -> Result<u16, GeneratorError> {
        read_u16_at(self.bytes, self.layout.atom_offsets + index as usize * 2)
    }

    fn validate(&self) -> Result<(), GeneratorError> {
        self.validate_atom_offsets()?;
        self.validate_owner_table(self.layout.champion)?;
        self.validate_owner_table(self.layout.item)?;
        self.validate_owner_table(self.layout.rune)?;

        let champion_first = self.owner_offset(self.layout.champion, 0)?;
        let champion_last =
            self.owner_offset(self.layout.champion, self.layout.champion.present as usize)?;
        let item_first = self.owner_offset(self.layout.item, 0)?;
        let item_last = self.owner_offset(self.layout.item, self.layout.item.present as usize)?;
        let rune_first = self.owner_offset(self.layout.rune, 0)?;
        let rune_last = self.owner_offset(self.layout.rune, self.layout.rune.present as usize)?;

        if champion_first != 0
            || champion_last != item_first
            || item_last != rune_first
            || rune_last != self.stream_data_len
        {
            return Err(GeneratorError::Corrupt(
                "owner offset groups do not cover stream contiguously",
            ));
        }

        Ok(())
    }

    fn validate_atom_offsets(&self) -> Result<(), GeneratorError> {
        let mut last = 0u16;
        for index in 0..=self.atoms {
            let offset = self.atom_offset(index)?;
            if offset < last || offset > self.dictionary_data_len {
                return Err(GeneratorError::Corrupt("non-monotonic atom offsets"));
            }
            last = offset;
        }

        if last != self.dictionary_data_len {
            return Err(GeneratorError::Corrupt(
                "atom offsets do not end at dictionary end",
            ));
        }
        Ok(())
    }

    fn validate_owner_table(&self, table: OwnerTableLayout) -> Result<(), GeneratorError> {
        let bitmap = self.owner_bitmap(table)?;

        let actual_present = (0..table.count as usize)
            .filter(|&index| bitmap_get(bitmap, index))
            .count();
        if actual_present != table.present as usize {
            return Err(GeneratorError::Corrupt(
                "presence bitmap count does not match header",
            ));
        }

        // Unused high bits in the final byte must be zero.
        let remaining = table.count as usize & 7;
        if remaining != 0 && !bitmap.is_empty() {
            let valid_mask = ((1u16 << remaining) - 1) as u8;
            if bitmap[bitmap.len() - 1] & !valid_mask != 0 {
                return Err(GeneratorError::Corrupt(
                    "presence bitmap contains bits outside owner range",
                ));
            }
        }

        let mut last = None;
        for ordinal in 0..=table.present as usize {
            let offset = self.owner_offset(table, ordinal)?;
            if offset > self.stream_data_len {
                return Err(GeneratorError::Corrupt(
                    "owner offset exceeds stream length",
                ));
            }
            if let Some(previous) = last {
                if offset < previous {
                    return Err(GeneratorError::Corrupt("non-monotonic owner offsets"));
                }
            }
            last = Some(offset);
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct AtomView<'a> {
    class: Option<Class>,
    text: &'a str,
}

// ============================================================================
// Runtime decoder / renderer
// ============================================================================

struct GeneratorRenderer<'db, 'bytes, 'emit> {
    db: &'db GeneratorDb<'bytes>,
    emit: &'emit mut dyn FnMut(&mut String, Option<Class>, &str),
    bracket_stack: Vec<Class>,
}

impl GeneratorRenderer<'_, '_, '_> {
    fn render(
        &mut self,
        cursor: &mut GeneratorCursor<'_>,
        out: &mut String,
    ) -> Result<(), GeneratorError> {
        while !cursor.is_eof() {
            let opcode = cursor.u8()?;

            if opcode < OP_ATOM_U16 {
                if opcode >= self.db.direct_atoms {
                    return Err(GeneratorError::Corrupt("direct atom id out of range"));
                }
                self.atom(opcode as u16, out)?;
                continue;
            }

            match opcode {
                OP_ATOM_U16 => self.atom(cursor.u16()?, out)?,
                OP_RAW_U8 => {
                    let len = cursor.u8()? as usize;
                    self.literal(cursor, out, None, len)?;
                }
                OP_RAW_U16 => {
                    let len = cursor.u16()? as usize;
                    self.literal(cursor, out, None, len)?;
                }
                OP_SPAN_U8_BASE..=OP_SPAN_U8_LAST => {
                    let class = class_from_u8(opcode - OP_SPAN_U8_BASE)?;
                    let len = cursor.u8()? as usize;
                    self.literal(cursor, out, Some(class), len)?;
                }
                OP_SPAN_U16 => {
                    let class = class_from_u8(cursor.u8()?)?;
                    let len = cursor.u16()? as usize;
                    self.literal(cursor, out, Some(class), len)?;
                }
                OP_LPAREN => self.open_bracket(out, "("),
                OP_RPAREN => self.close_bracket(out, ")"),
                OP_LCURLY => self.open_bracket(out, "{"),
                OP_RCURLY => self.close_bracket(out, "}"),
                OP_LBRACKET => self.open_bracket(out, "["),
                OP_RBRACKET => self.close_bracket(out, "]"),
                _ => return Err(GeneratorError::Corrupt("unknown generator opcode")),
            }
        }

        Ok(())
    }

    fn atom(&mut self, id: u16, out: &mut String) -> Result<(), GeneratorError> {
        let atom = self.db.atom(id)?;
        (self.emit)(out, atom.class, atom.text);
        Ok(())
    }

    fn literal(
        &mut self,
        cursor: &mut GeneratorCursor<'_>,
        out: &mut String,
        class: Option<Class>,
        len: usize,
    ) -> Result<(), GeneratorError> {
        let text = str::from_utf8(cursor.bytes(len)?)?;
        (self.emit)(out, class, text);
        Ok(())
    }

    fn open_bracket(&mut self, out: &mut String, text: &str) {
        let class = Class::bracket(self.bracket_stack.len());
        self.bracket_stack.push(class);
        (self.emit)(out, Some(class), text);
    }

    fn close_bracket(&mut self, out: &mut String, text: &str) {
        // Display-only: unmatched closing brackets are legal input and receive
        // Bracket1, matching the permissive behavior of the old Builder.
        let class = self.bracket_stack.pop().unwrap_or(Class::Bracket1);
        (self.emit)(out, Some(class), text);
    }
}

struct GeneratorCursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> GeneratorCursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos == self.bytes.len()
    }

    fn u8(&mut self) -> Result<u8, GeneratorError> {
        let value = *self.bytes.get(self.pos).ok_or(GeneratorError::Corrupt(
            "unexpected end of generator stream",
        ))?;
        self.pos += 1;
        Ok(value)
    }

    fn u16(&mut self) -> Result<u16, GeneratorError> {
        let bytes = self.bytes(2)?;
        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn bytes(&mut self, len: usize) -> Result<&'a [u8], GeneratorError> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or(GeneratorError::Corrupt("generator cursor overflow"))?;
        let result = self
            .bytes
            .get(self.pos..end)
            .ok_or(GeneratorError::Corrupt(
                "unexpected end of generator stream",
            ))?;
        self.pos = end;
        Ok(result)
    }
}

// ============================================================================
// Binary helpers
// ============================================================================

struct BinWriter {
    bytes: Vec<u8>,
}

impl BinWriter {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    fn len(&self) -> usize {
        self.bytes.len()
    }

    fn u8(&mut self, value: u8) -> &mut Self {
        self.bytes.push(value);
        self
    }

    fn u16(&mut self, value: u16) -> &mut Self {
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self
    }

    fn u32(&mut self, value: u32) -> &mut Self {
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self
    }

    fn bytes(&mut self, value: &[u8]) -> &mut Self {
        self.bytes.extend_from_slice(value);
        self
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

fn read_u16_at(bytes: &[u8], pos: usize) -> Result<u16, GeneratorError> {
    let end = pos
        .checked_add(2)
        .ok_or(GeneratorError::Corrupt("u16 offset overflow"))?;
    let slice = bytes
        .get(pos..end)
        .ok_or(GeneratorError::Corrupt("u16 out of bounds"))?;
    Ok(u16::from_le_bytes(slice.try_into().unwrap()))
}

fn read_u32_at(bytes: &[u8], pos: usize) -> Result<u32, GeneratorError> {
    let end = pos
        .checked_add(4)
        .ok_or(GeneratorError::Corrupt("u32 offset overflow"))?;
    let slice = bytes
        .get(pos..end)
        .ok_or(GeneratorError::Corrupt("u32 out of bounds"))?;
    Ok(u32::from_le_bytes(slice.try_into().unwrap()))
}

#[inline]
fn checked_add(a: usize, b: usize) -> Result<usize, GeneratorError> {
    a.checked_add(b)
        .ok_or(GeneratorError::Corrupt("section offset overflow"))
}

#[inline]
fn checked_mul(a: usize, b: usize) -> Result<usize, GeneratorError> {
    a.checked_mul(b)
        .ok_or(GeneratorError::Corrupt("section size overflow"))
}

#[inline]
fn class_from_u8(value: u8) -> Result<Class, GeneratorError> {
    if value > MAX_CLASS {
        return Err(GeneratorError::Corrupt("invalid class id"));
    }
    Ok(class_from_u8_unchecked(value))
}

#[inline]
fn class_from_u8_unchecked(value: u8) -> Class {
    debug_assert!(value <= MAX_CLASS);
    // Class is #[repr(u8)] with contiguous discriminants 0..=15 in render.rs.
    unsafe { std::mem::transmute::<u8, Class>(value) }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const OVERRIDES: &[ClassOverride] = &[
        ClassOverride::new("_1Min", Class::Constant),
        ClassOverride::new("Physical", Class::Constant),
    ];

    #[test]
    fn exact_round_trip_and_override() {
        let source = r#"impl Foo {
    fn generate(&mut self) {
        self.value = _1Min + Physical;
    }
}"#;

        let mut builder = GeneratorDbBuilder::new(2, 4, 3, OVERRIDES);
        builder.push(EntityKind::Champion, 1, source).unwrap();

        let packed = builder.finish_with_stats().unwrap();
        let db = GeneratorDb::parse(&packed.bytes).unwrap();

        assert_eq!(
            db.render_plain(EntityKind::Champion, 1).unwrap().unwrap(),
            source
        );

        let html = db.render_html(EntityKind::Champion, 1).unwrap().unwrap();
        assert!(html.contains(&format!(
            "<span class=\"C{}\">_1Min</span>",
            Class::Constant as u8
        )));
    }

    #[test]
    fn sparse_owners_return_none_without_generator_id() {
        let mut builder = GeneratorDbBuilder::new(1, 16, 16, &[]);
        builder.push_item(3, "item_three()").unwrap();
        builder.push_item(15, "item_fifteen()").unwrap();
        builder.push_rune(9, "rune_nine()").unwrap();

        let bytes = builder.finish().unwrap();
        let db = GeneratorDb::parse(&bytes).unwrap();

        assert_eq!(
            db.render_plain(EntityKind::Item, 3).unwrap().as_deref(),
            Some("item_three()")
        );
        assert_eq!(
            db.render_plain(EntityKind::Item, 15).unwrap().as_deref(),
            Some("item_fifteen()")
        );
        assert!(db.render_plain(EntityKind::Item, 4).unwrap().is_none());
        assert_eq!(
            db.render_plain(EntityKind::Rune, 9).unwrap().as_deref(),
            Some("rune_nine()")
        );
    }

    #[test]
    fn preserves_final_newline_comments_and_invalid_brackets() {
        let source = "/* keep me exactly */\nfoo(]\n";

        let mut builder = GeneratorDbBuilder::new(1, 0, 0, &[]);
        builder.push_champion(0, source).unwrap();

        let bytes = builder.finish().unwrap();
        let db = GeneratorDb::parse(&bytes).unwrap();

        assert_eq!(
            db.render_plain(EntityKind::Champion, 0).unwrap().unwrap(),
            source
        );
    }

    #[test]
    fn present_empty_source_is_different_from_missing_owner() {
        let mut builder = GeneratorDbBuilder::new(0, 2, 0, &[]);
        builder.push_item(1, "").unwrap();

        let bytes = builder.finish().unwrap();
        let db = GeneratorDb::parse(&bytes).unwrap();

        assert!(db.render_plain(EntityKind::Item, 0).unwrap().is_none());
        assert_eq!(
            db.render_plain(EntityKind::Item, 1).unwrap().as_deref(),
            Some("")
        );
    }
}

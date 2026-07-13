//! Scans a source string into `Segment`s. At every position: try every
//! registered template and keep the longest match whose holes all
//! resolve; otherwise try the per-class grammar; otherwise fall back to an
//! uncolored literal character.

use crate::class::Class;
use crate::dictionary::Dictionary;
use crate::hole::Hole;
use crate::io::Writer;
use crate::segment::Segment;
use crate::template::TemplateRegistry;
use regex::Regex;
use std::sync::LazyLock;

static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z_][a-zA-Z0-9_]*").unwrap());
static SCREAMING_CONSTANT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:::[A-Z_][A-Za-z0-9_]*|[A-Z][A-Z0-9_]*)").unwrap());
static TYPE_NAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[A-Z][a-zA-Z0-9]*").unwrap());
static NUMBER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:0x[0-9A-Fa-f_]+|0o[0-7_]+|0b[01_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d[\d_]*)?)(?:[iu](?:8|16|32|64|128|size)|f(?:32|64))?",
    )
    .unwrap()
});
static LIFETIME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^'[a-z_][a-zA-Z0-9_]*").unwrap());
static LINE_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^//[^\n]*").unwrap());
static STRING_LITERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^"(?:[^"\\]|\\.)*""#).unwrap());

const BRACKET_CHARS: &str = "()[]{}";

pub struct Encoder<'a> {
    source: &'a str,
    dictionary: &'a Dictionary,
    templates: &'a TemplateRegistry,
    pos: usize,
    bracket_depth: usize,
    segments: Vec<Segment>,
}

impl<'a> Encoder<'a> {
    pub fn new(
        source: &'a str,
        dictionary: &'a Dictionary,
        templates: &'a TemplateRegistry,
    ) -> Self {
        Encoder {
            source,
            dictionary,
            templates,
            pos: 0,
            bracket_depth: 0,
            segments: Vec::new(),
        }
    }

    /// Runs the whole scan and serializes the result to `.ir` + `.txt` bytes.
    pub fn encode(mut self) -> (Vec<u8>, Vec<u8>) {
        while self.pos < self.source.len() {
            self.step();
        }
        let mut writer = Writer::new();
        for segment in &self.segments {
            segment.write(&mut writer);
        }
        writer.finish()
    }

    /// Consumes one segment's worth of source starting at `self.pos`.
    fn step(&mut self) {
        if self.try_template() {
            return;
        }
        if self.try_known_class() {
            return;
        }
        self.push_literal_char();
    }

    /// Tries every template at the current position, keeping the longest
    /// match whose holes all resolve. A template whose pattern matches but
    /// whose holes don't resolve is skipped, not treated as an error.
    fn try_template(&mut self) -> bool {
        let remaining = &self.source[self.pos..];
        let mut best: Option<(u8, usize, Vec<Hole>)> = None;

        for (id, template) in self.templates.all() {
            let Some(captures) = template.pattern().captures(remaining) else {
                continue;
            };
            let Some(holes) = template.encode_holes(&captures, self.dictionary) else {
                continue;
            };
            let matched_len = captures.get(0).unwrap().end();
            if best.as_ref().map_or(true, |(_, len, _)| matched_len > *len) {
                best = Some((id, matched_len, holes));
            }
        }

        match best {
            Some((id, len, holes)) => {
                self.absorb_bracket_depth(&remaining[..len]);
                self.segments.push(Segment::template(id, holes));
                self.pos += len;
                true
            }
            None => false,
        }
    }

    /// A template's fixed text can contain literal brace characters (e.g.
    /// the two `{` in the generator-impl pattern). They're rendered as
    /// plain, uncolored text by the template itself, but the nesting
    /// *count* still has to see them — otherwise brackets later in the
    /// source would drift out of sync with what's actually open at that
    /// point.
    fn absorb_bracket_depth(&mut self, consumed: &str) {
        for c in consumed.chars() {
            if "([{".contains(c) {
                self.bracket_depth += 1;
            } else if ")]}".contains(c) {
                self.bracket_depth = self.bracket_depth.saturating_sub(1);
            }
        }
    }

    /// Tries the fixed per-class grammar, in priority order, at the
    /// current position.
    fn try_known_class(&mut self) -> bool {
        let remaining = &self.source[self.pos..];

        if let Some(m) = LINE_COMMENT.find(remaining) {
            return self.push_unknown(Class::Comment, m.as_str());
        }
        if let Some(m) = STRING_LITERAL.find(remaining) {
            return self.push_unknown(Class::String, m.as_str());
        }
        if let Some(m) = LIFETIME.find(remaining) {
            return self.push_unknown(Class::Lifetime, m.as_str());
        }
        if let Some(m) = IDENTIFIER.find(remaining) {
            return self.push_word(m.as_str());
        }
        if let Some(m) = SCREAMING_CONSTANT.find(remaining) {
            return self.push_dictionary_word(Class::Constant, m.as_str());
        }
        if let Some(m) = TYPE_NAME.find(remaining) {
            return self.push_dictionary_word(Class::Type, m.as_str());
        }
        if let Some(m) = NUMBER.find(remaining) {
            return self.push_unknown(Class::Number, m.as_str());
        }
        if let Some(c) = remaining.chars().next() {
            if BRACKET_CHARS.contains(c) {
                return self.push_bracket(c);
            }
        }
        false
    }

    /// Classifies a lowercase identifier already known to start at the
    /// cursor: boolean literal, then keyword/control/primitive
    /// dictionaries, then macro (`!` suffix) or function (`(` suffix)
    /// dictionaries, falling back to the variable dictionary.
    fn push_word(&mut self, word: &str) -> bool {
        if word == "true" || word == "false" {
            return self.push_unknown(Class::Boolean, word);
        }
        for class in [Class::Keyword, Class::Control, Class::Primitive] {
            if self.dictionary.index_of(class, word).is_some() {
                return self.push_dictionary_word(class, word);
            }
        }
        let after = &self.source[self.pos + word.len()..];
        if after.starts_with('!') {
            let with_bang = &self.source[self.pos..self.pos + word.len() + 1];
            return self.push_dictionary_word(Class::Macro, with_bang);
        }
        if after.starts_with('(') {
            return self.push_dictionary_word(Class::Function, word);
        }
        self.push_dictionary_word(Class::Variable, word)
    }

    fn push_dictionary_word(&mut self, class: Class, word: &str) -> bool {
        match self.dictionary.index_of(class, word) {
            Some(index) => self.segments.push(Segment::colored_known(class, index)),
            None => self.segments.push(Segment::colored_unknown(class, word)),
        }
        self.pos += word.len();
        true
    }

    fn push_unknown(&mut self, class: Class, text: &str) -> bool {
        self.segments.push(Segment::colored_unknown(class, text));
        self.pos += text.len();
        true
    }

    /// Rainbow brackets: depth is tracked mod 3, and a closing bracket
    /// decrements before computing its class so it matches its opener.
    fn push_bracket(&mut self, c: char) -> bool {
        let is_open = "([{".contains(c);
        if !is_open {
            self.bracket_depth = self.bracket_depth.saturating_sub(1);
        }
        let class = match self.bracket_depth % 3 {
            0 => Class::Bracket1,
            1 => Class::Bracket2,
            _ => Class::Bracket3,
        };
        if is_open {
            self.bracket_depth += 1;
        }
        self.segments
            .push(Segment::colored_unknown(class, c.to_string()));
        self.pos += c.len_utf8();
        true
    }

    /// Uncategorized characters (operators, whitespace, punctuation) get
    /// no span at all. Consecutive ones are merged into a single literal
    /// segment instead of one segment per character.
    fn push_literal_char(&mut self) {
        let c = self.source[self.pos..].chars().next().unwrap();
        match self.segments.last_mut() {
            Some(Segment::Literal(text)) => text.push(c),
            _ => self.segments.push(Segment::literal(c.to_string())),
        }
        self.pos += c.len_utf8();
    }
}

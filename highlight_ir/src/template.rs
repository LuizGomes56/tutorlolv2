//! Templates collapse a recurring multi-token source pattern into a single
//! invocation. Each template owns a regex for finding itself in the
//! source, a fixed hole schema (encoded implicitly by its own
//! `encode_holes`/`read_holes` pair), and a render routine that rebuilds
//! the original HTML from the holes.

use std::str::FromStr;

use crate::class::Class;
use crate::dictionary::Dictionary;
use crate::hole::Hole;
use crate::io::Cursor;
use regex::{Captures, Regex};
use tutorlolv2::ItemId;

pub trait Template: Sync {
    /// Regex used by the encoder to find this pattern in the source text.
    /// Anchored with `^` so it only ever matches at the cursor's current
    /// position, never further ahead.
    fn pattern(&self) -> &Regex;

    /// Turns a regex match into holes, or `None` if the captured text
    /// doesn't resolve to known values (dictionary miss, enum miss, or an
    /// inconsistency between two occurrences of what should be the same
    /// value). `None` means the encoder falls back to normal tokenization
    /// for this span — templates never store an "unknown" fallback of
    /// their own.
    fn encode_holes(&self, captures: &Captures, dictionary: &Dictionary) -> Option<Vec<Hole>>;

    /// Reads exactly the holes this template's schema defines, in order.
    fn read_holes(&self, cursor: &mut Cursor) -> Vec<Hole>;

    fn render(&self, holes: &[Hole], dictionary: &Dictionary) -> String;
}

/// `impl Generator for {Name} { fn generate(&mut self) -> MayFail {`
///
/// One hole: the implementing type's name, stored as arbitrary text since
/// it isn't guaranteed to be one of the small set of known `Type` names.
pub struct GeneratorImplTemplate {
    pattern: Regex,
}

impl GeneratorImplTemplate {
    pub fn new() -> Self {
        GeneratorImplTemplate {
            pattern: Regex::new(
                r"^impl Generator for ([A-Z][a-zA-Z0-9_]*) \{\s*fn generate\(&mut self\) -> MayFail \{",
            )
            .unwrap(),
        }
    }
}

impl Template for GeneratorImplTemplate {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }

    fn encode_holes(&self, captures: &Captures, _dictionary: &Dictionary) -> Option<Vec<Hole>> {
        Some(vec![Hole::Text(captures[1].to_string())])
    }

    fn read_holes(&self, cursor: &mut Cursor) -> Vec<Hole> {
        vec![Hole::Text(cursor.read_aux_text().to_string())]
    }

    fn render(&self, holes: &[Hole], _dictionary: &Dictionary) -> String {
        let Hole::Text(name) = &holes[0] else {
            unreachable!("schema guarantees a Text hole")
        };
        format!(
            "<span class=\"Keyword\">impl</span> <span class=\"Type\">Generator</span> \
<span class=\"Keyword\">for</span> <span class=\"Type\">{name}</span> {{\n    \
<span class=\"Keyword\">fn</span> <span class=\"Function\">generate</span>(&mut self) -> \
<span class=\"Type\">MayFail</span> {{"
        )
    }
}

/// `static ITEM_<NAME>: Item = Item { name: "<name>",`
///
/// Two holes: the item id (as a two-byte discriminant, reused for both the
/// static's screaming-case name and the display name) and the struct type
/// name (as a one-byte index into the existing `Type` dictionary, reused
/// for both the type annotation and the struct-literal head).
pub struct StaticItemTemplate {
    pattern: Regex,
}

impl StaticItemTemplate {
    pub fn new() -> Self {
        StaticItemTemplate {
            pattern: Regex::new(r#"^static (ITEM_[A-Z_]+): (\w+) = (\w+) \{\s*name: "(\w+)","#)
                .unwrap(),
        }
    }
}

impl Template for StaticItemTemplate {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }

    fn encode_holes(&self, captures: &Captures, dictionary: &Dictionary) -> Option<Vec<Hole>> {
        let item = ItemId::from_str(&captures[1]).ok()?;
        if captures[2] != captures[3] || captures[4] != *item.name() {
            return None; // structurally similar but inconsistent — let the fallback handle it
        }
        let type_index = dictionary.index_of(Class::Type, &captures[2])?;
        Some(vec![Hole::Index16(item as _), Hole::Index8(type_index)])
    }

    fn read_holes(&self, cursor: &mut Cursor) -> Vec<Hole> {
        vec![
            Hole::Index16(cursor.read_u16_le()),
            Hole::Index8(cursor.read_u8()),
        ]
    }

    fn render(&self, holes: &[Hole], dictionary: &Dictionary) -> String {
        let (Hole::Index16(item_bits), Hole::Index8(type_index)) = (&holes[0], &holes[1]) else {
            unreachable!("schema guarantees Index16 then Index8")
        };
        // Safe: the encoder only ever emits a discriminant it already
        // resolved from a real `ItemId` variant (see `encode_holes`).
        let item = unsafe { ItemId::from_u16_unchecked(*item_bits) };
        let type_name = dictionary.word_at(Class::Type, *type_index);
        format!(
            "<span class=\"Keyword\">static</span> <span class=\"Constant\">{item:?}</span>: \
<span class=\"Type\">{type_name}</span> = <span class=\"Type\">{type_name}</span> {{\n    \
name: <span class=\"String\">\"{}\"</span>,",
            item.name()
        )
    }
}

/// Owns every registered template, indexed by `templateId`.
pub struct TemplateRegistry {
    templates: Vec<Box<dyn Template>>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        TemplateRegistry {
            templates: vec![
                Box::new(GeneratorImplTemplate::new()),
                Box::new(StaticItemTemplate::new()),
            ],
        }
    }

    pub fn get(&self, id: u8) -> &dyn Template {
        self.templates[id as usize].as_ref()
    }

    /// Every registered template paired with its id, for the encoder to
    /// try in turn at each source position.
    pub fn all(&self) -> impl Iterator<Item = (u8, &dyn Template)> {
        self.templates
            .iter()
            .enumerate()
            .map(|(i, t)| (i as u8, t.as_ref()))
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        TemplateRegistry::new()
    }
}

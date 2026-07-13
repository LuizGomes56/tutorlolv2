//! Static per-`Class` dictionaries of known tokens. In production these
//! word lists would be generated (e.g. by a `build.rs`) from the project's
//! JSON file of known tokens; the lists below are a small placeholder
//! fixture that exercises the same lookup mechanism end to end. Swapping in
//! real data only means changing the `const` arrays — `Dictionary` itself
//! doesn't change.

use crate::class::Class;

const KEYWORD_WORDS: &[&str] = &["fn", "let", "impl", "struct", "enum", "for", "static", "mut"];
const CONTROL_WORDS: &[&str] = &["if", "else", "match", "while", "loop", "return", "break"];
const CONSTANT_WORDS: &[&str] = &["ITEM_SWORD", "ITEM_SHIELD", "MAX_HEALTH"];
const TYPE_WORDS: &[&str] = &["Item", "Player", "World"];
const PRIMITIVE_WORDS: &[&str] = &["u8", "u16", "u32", "bool", "str"];
const MACRO_WORDS: &[&str] = &["format!", "vec!"];
const FUNCTION_WORDS: &[&str] = &["generate", "zero"];
const VARIABLE_WORDS: &[&str] = &["self", "value", "result"];

pub struct Dictionary {
    /// One slice of known token strings per `Class`, indexed by `class as
    /// usize`. Classes without a dictionary keep the default empty slice.
    tables: [&'static [&'static str]; 16],
}

impl Dictionary {
    pub fn new() -> Self {
        let mut tables: [&'static [&'static str]; 16] = [&[]; 16];
        tables[Class::Keyword as usize] = KEYWORD_WORDS;
        tables[Class::Control as usize] = CONTROL_WORDS;
        tables[Class::Constant as usize] = CONSTANT_WORDS;
        tables[Class::Type as usize] = TYPE_WORDS;
        tables[Class::Primitive as usize] = PRIMITIVE_WORDS;
        tables[Class::Macro as usize] = MACRO_WORDS;
        tables[Class::Function as usize] = FUNCTION_WORDS;
        tables[Class::Variable as usize] = VARIABLE_WORDS;
        Dictionary { tables }
    }

    /// Looks up `word` inside `class`'s table, returning its index if known.
    pub fn index_of(&self, class: Class, word: &str) -> Option<u8> {
        self.tables[class as usize]
            .iter()
            .position(|&known| known == word)
            .map(|i| i as u8)
    }

    /// Resolves a known index back to its literal text.
    pub fn word_at(&self, class: Class, index: u8) -> &'static str {
        self.tables[class as usize][index as usize]
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Dictionary::new()
    }
}

use crate::MayFail;
use heck::ToSnakeCase;
use lazy_regex::regex;
use regex::Regex;
use std::fmt::Display;

pub use tutorlolv2_types::Tag;

pub const fn generator_directory(tag: Tag) -> &'static str {
    match tag {
        Tag::Champions => "build/src/generators/champions",
        Tag::Items => "build/src/generators/impls/items",
        Tag::Runes => "build/src/generators/impls/runes",
    }
}

pub trait RegExtractor: Display {
    /// Returns a vector of all the numbers that could be extracted
    /// from some string, preserving the order that they were found
    fn capture_numbers(&self) -> Vec<f64> {
        let text = self.to_string();

        regex!(r"\d+(?:\.\d+)?")
            .find_iter(&text)
            .filter_map(|m| m.as_str().parse().ok())
            .collect()
    }

    /// Numbers matched by the `N` capture groups of `pattern`, in order. Fails
    /// when the text no longer matches, so a wiki change can't go unnoticed
    /// `pattern` is usually built with [`regex!`](lazy_regex::regex), which checks
    /// it at compile time
    fn capture<const N: usize>(&self, pattern: &Regex) -> MayFail<[f64; N]> {
        let text = self.to_string();
        let source = pattern.as_str();
        let captures = pattern
            .captures(&text)
            .ok_or_else(|| format!("Pattern {source:?} did not match {text:?}"))?;

        let values = captures
            .iter()
            .skip(1)
            .map(|group| -> MayFail<f64> {
                let group =
                    group.ok_or_else(|| format!("Pattern {source:?} has an empty group"))?;
                Ok(group.as_str().parse()?)
            })
            .collect::<MayFail<Vec<_>>>()?;

        let len = values.len();

        values
            .try_into()
            .map_err(|_| format!("Pattern {source:?} has {len} groups, expected {N}").into())
    }
}

impl<T: Display + ?Sized> RegExtractor for T {}

#[derive(Copy, Clone)]
pub enum SaveTo<'a> {
    GeneratorDir(Tag),
    GeneratorRaw(Tag, &'a str),
    RiotItems,
    RiotRunes,
    InternalRaw(Tag, &'a str),
    InternalDir(Tag),
}

impl<'a> SaveTo<'a> {
    pub fn path(&self) -> String {
        let relative: String = match self {
            SaveTo::GeneratorDir(tag) => generator_directory(*tag).to_owned(),
            SaveTo::GeneratorRaw(tag, s) => {
                let path = Self::GeneratorDir(*tag).path();
                let file = s.to_snake_case();
                format!("{path}/{file}.rs")
            }
            SaveTo::RiotItems => "cache/riot/items.json".into(),
            SaveTo::RiotRunes => "cache/riot/runes.json".into(),
            SaveTo::InternalRaw(tag, s) => format!("cache/app/{}/{s}.json", tag.as_ref()),
            SaveTo::InternalDir(tag) => format!("cache/app/{}", tag.as_ref()),
        };

        crate::project_root()
            .join(relative)
            .to_string_lossy()
            .into_owned()
    }
}

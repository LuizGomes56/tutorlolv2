use crate::MayFail;
use heck::ToSnakeCase;
use lazy_regex::regex;
use regex::Regex;
use std::fmt::Display;

pub use types::Tag;

pub const fn generator_directory(tag: Tag) -> &'static str {
    match tag {
        Tag::Champions => "crates/build/src/generators/champions",
        Tag::Items => "crates/build/src/generators/impls/items",
        Tag::Runes => "crates/build/src/generators/impls/runes",
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

                number(group.as_str())
            })
            .collect::<MayFail<Vec<_>>>()?;

        let len = values.len();

        values
            .try_into()
            .map_err(|_| format!("Pattern {source:?} has {len} groups, expected {N}").into())
    }
}

impl<T: Display + ?Sized> RegExtractor for T {}

/// A number written in digits, or as a word the wiki uses for counts, as the
/// `two` of Sett's "next two basic attacks" or the `twice` of Ahri's recasts
pub fn number(text: &str) -> MayFail<f64> {
    if let Ok(value) = text.parse() {
        return Ok(value);
    }

    let value = match text.to_ascii_lowercase().as_str() {
        "half" => 0.5,
        "one" | "once" | "single" => 1.0,
        "two" | "twice" | "double" | "second" => 2.0,
        "three" | "thrice" | "triple" | "third" => 3.0,
        "four" | "quadruple" | "fourth" => 4.0,
        "five" | "fifth" => 5.0,
        "six" | "sixth" => 6.0,
        "seven" => 7.0,
        "eight" => 8.0,
        "nine" => 9.0,
        "ten" => 10.0,
        _ => return Err(format!("{text:?} is not a number").into()),
    };

    Ok(value)
}

/// Number of times an effect that happens `every` seconds does over `duration`
/// seconds, as the 8 ticks of Udyr's blizzard every 0.5 seconds for 4 seconds.
/// A tick that would land after the duration doesn't happen
pub fn ticks(every: f64, duration: f64) -> MayFail<f64> {
    if every <= 0.0 || duration <= 0.0 {
        return Err(format!("Expected positive ticks, got every {every} for {duration}").into());
    }

    Ok((duration / every + 1e-9).floor())
}

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

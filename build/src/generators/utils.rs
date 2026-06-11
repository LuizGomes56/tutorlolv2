use regex::Regex;
use std::{fmt::Display, sync::LazyLock};
use tutorlolv2_fmt::to_ssnake;
use tutorlolv2_types::{Key, Position};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    Items,
    Champions,
    Runes,
}

impl Display for Tag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Tag::Items => "items",
            Tag::Champions => "champions",
            Tag::Runes => "runes",
        };

        write!(f, "{name}")
    }
}

pub trait RegExtractor: Display {
    /// Returns a vector of all the numbers that could be extracted
    /// from some string, preserving the order that they were found
    fn capture_numbers(&self) -> Vec<f64> {
        static CAPTURE_NUMBERS_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"\d+(?:\.\d+)?").unwrap());

        let text = self.to_string();

        CAPTURE_NUMBERS_RE
            .find_iter(&text)
            .filter_map(|m| m.as_str().parse().ok())
            .collect()
    }

    fn parenthesize(&self) -> String {
        format!("({self})")
    }

    fn sep<T: Display>(&self, sep: char, value: T) -> String {
        format!("{self} {sep} {value}")
    }

    fn times<T: Display>(&self, value: T) -> String {
        self.sep('*', value)
    }

    fn plus<T: Display>(&self, value: T) -> String {
        self.sep('+', value)
    }

    fn minus<T: Display>(&self, value: T) -> String {
        self.sep('-', value)
    }

    fn div<T: Display>(&self, value: T) -> String {
        self.sep('/', value)
    }
}

#[macro_export]
macro_rules! formula {
    (($($inner:tt)*)) => {
        formula!($($inner)*).parenthesize()
    };
    ($first:tt $($rest:tt)*) => {
        $crate::formula_impl!($crate::formula_atom!($first); $($rest)*)
    };
}

#[macro_export]
macro_rules! formula_atom {
    (($($inner:tt)*)) => {
        $crate::formula!($($inner)*).parenthesize()
    };
    ($value:tt) => {
        $value
    };
}

#[macro_export]
macro_rules! formula_impl {
    ($acc:expr;) => {
        $acc
    };
    ($acc:expr; + $rhs:tt $($rest:tt)*) => {
        $crate::formula_impl!(
            $acc.plus($crate::formula_atom!($rhs));
            $($rest)*
        )
    };
    ($acc:expr; - $rhs:tt $($rest:tt)*) => {
        $crate::formula_impl!(
            $acc.minus($crate::formula_atom!($rhs));
            $($rest)*
        )
    };
    ($acc:expr; * $rhs:tt $($rest:tt)*) => {
        $crate::formula_impl!(
            $acc.times($crate::formula_atom!($rhs));
            $($rest)*
        )
    };
    ($acc:expr; / $rhs:tt $($rest:tt)*) => {
        $crate::formula_impl!(
            $acc.div(formula_atom!($rhs));
            $($rest)*
        )
    };
}

impl<T: Display + ?Sized> RegExtractor for T {}

#[derive(Copy, Clone)]
pub enum SaveTo<'a> {
    GeneratorDir(Tag),
    GeneratorRaw(Tag, &'a str),
    RiotChampions,
    RiotItems,
    RiotItemsDir,
    RiotChampionsDir,
    RiotRunes,
    RiotLangDir(&'a str),
    RiotRawChampions(&'a str),
    RiotCache(Tag, &'a (dyn Display + Send + Sync)),
    InternalRaw(Tag, &'a str),
    InternalDir(Tag),
    InternalScraperData,
    InternalChampionLanguages,
    InternalDamagingItems,
    InternalLanguages,
    InternalMaps,
    InternalRuneNames,
    InternalRunes,
    ImgChampion(&'a str),
    ImgAbility(&'a str, Key),
    ImgItem(&'a str),
    ImgCentered(&'a str, usize),
    ImgSplash(&'a str, usize),
    ImgRunes(usize),
    ScraperBuilds(Position, &'a str),
    ScraperCombos(&'a str),
    InternalScraperBuilds(Position, &'a str),
    InternalScraperCombos(&'a str),
}

impl<'a> SaveTo<'a> {
    pub fn path(&self) -> String {
        let img = "raw_img";

        match self {
            SaveTo::GeneratorDir(tag) => format!("tutorlolv2_dev/src/generators/gen_{tag}"),
            SaveTo::GeneratorRaw(tag, s) => {
                let path = Self::GeneratorDir(*tag).path();
                let file = match tag {
                    Tag::Items | Tag::Runes => to_ssnake(s),
                    Tag::Champions => s.to_string(),
                }
                .to_lowercase();
                format!("{path}/{file}.rs")
            }
            SaveTo::ImgChampion(s) => format!("{img}/champions/{s}.png"),
            SaveTo::ImgAbility(s, c) => format!("{img}/abilities/{s}{c:?}.png"),
            SaveTo::ImgItem(s) => format!("{img}/items/{s}.png"),
            SaveTo::ImgCentered(s, n) => format!("{img}/centered/{s}_{n}.jpg"),
            SaveTo::ImgSplash(s, n) => format!("{img}/splash/{s}_{n}.jpg"),
            SaveTo::ImgRunes(n) => format!("{img}/runes/{n}.png"),
            SaveTo::RiotCache(s, f) => format!("cache/riot/{s}/{f}.json"),
            SaveTo::RiotItems => "cache/riot/items.json".into(),
            SaveTo::RiotChampions => "cache/riot/champions.json".into(),
            SaveTo::RiotItemsDir => "cache/riot/items".into(),
            SaveTo::RiotChampionsDir => "cache/riot/champions".into(),
            SaveTo::RiotRunes => "cache/riot/runes.json".into(),
            SaveTo::RiotLangDir(s) => format!("cache/riot/champions_lang/{s}.json"),
            SaveTo::RiotRawChampions(s) => format!("cache/riot/raw_champions/{s}.json"),
            SaveTo::ScraperBuilds(position, s) => {
                format!("cache/scraper/builds/{position:?}/{s}.html")
            }
            SaveTo::ScraperCombos(s) => format!("cache/scraper/combos/{s}.html"),
            SaveTo::InternalRaw(tag, s) => format!("internal/{tag}/{s}.json"),
            SaveTo::InternalDir(tag) => format!("internal/{tag}"),
            SaveTo::InternalScraperBuilds(position, s) => {
                format!("internal/scraper/builds/{position:?}/{s}.json")
            }
            SaveTo::InternalScraperCombos(champion_id) => {
                format!("internal/scraper/combos/{champion_id}.json")
            }
            SaveTo::InternalScraperData => "internal/scraper/data.json".into(),
            SaveTo::InternalChampionLanguages => "internal/champion_languages.json".into(),
            SaveTo::InternalDamagingItems => "internal/damaging_items.json".into(),
            SaveTo::InternalLanguages => "internal/languages.json".into(),
            SaveTo::InternalMaps => "internal/maps.json".into(),
            SaveTo::InternalRuneNames => "internal/rune_names.json".into(),
            SaveTo::InternalRunes => "internal/runes.json".into(),
        }
    }
}

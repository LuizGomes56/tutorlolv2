use heck::ToSnakeCase;
use regex::Regex;
use std::{fmt::Display, sync::LazyLock};

pub use tutorlolv2_types::Tag;

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
        $crate::formula!($($inner)*).parenthesize()
    };
    ($first:tt $($rest:tt)*) => {
        $crate::formula_impl!($crate::formula_atom!($first); $($rest)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! formula_atom {
    (($($inner:tt)*)) => {
        $crate::formula!($($inner)*).parenthesize()
    };
    ($value:tt) => {
        &$value
    };
}

#[doc(hidden)]
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
            $acc.div($crate::formula_atom!($rhs));
            $($rest)*
        )
    };
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
            SaveTo::GeneratorDir(tag) => format!("build/src/generators/impls/{}", tag.as_ref()),
            SaveTo::GeneratorRaw(tag, s) => {
                let path = Self::GeneratorDir(*tag).path();
                let file = s.to_snake_case();
                format!("{path}/{file}.rs")
            }
            SaveTo::RiotItems => "cache/riot/items.json".into(),
            SaveTo::RiotRunes => "cache/riot/runes.json".into(),
            SaveTo::InternalRaw(tag, s) => format!("internal/{}/{s}.json", tag.as_ref()),
            SaveTo::InternalDir(tag) => format!("internal/{}", tag.as_ref()),
        };

        crate::project_root()
            .join(relative)
            .to_string_lossy()
            .into_owned()
    }
}

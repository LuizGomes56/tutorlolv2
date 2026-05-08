use regex::Regex;
use std::{fmt::Display, sync::LazyLock};

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

    fn parens(&self) -> String {
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

impl<T: Display + ?Sized> RegExtractor for T {}

use regex::Regex;
use std::{fmt::Display, str::FromStr, sync::LazyLock};

pub trait RegExtractor {
    /// Returns a vector of all the numbers that could be extracted
    /// from some string, preserving the order that they were found
    fn capture_numbers<T: FromStr>(&self) -> Vec<T>;
    fn parens(&self) -> String;
    fn times<T: Display>(&self, value: T) -> String;
    fn plus<U>(&self, value: U) -> String
    where
        U: Display;
    fn minus<T>(&self, value: T) -> String
    where
        T: Display;
    fn div<T>(&self, value: T) -> String
    where
        T: Display;
}

impl RegExtractor for str {
    fn capture_numbers<T: FromStr>(&self) -> Vec<T> {
        static CAPTURE_NUMBERS_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"\d+(?:\.\d+)?").unwrap());

        CAPTURE_NUMBERS_RE
            .find_iter(self)
            .filter_map(|m| m.as_str().parse().ok())
            .collect()
    }

    fn parens(&self) -> String {
        format!("({self})")
    }

    fn plus<T: Display>(&self, value: T) -> String {
        format!("{self} + {value}")
    }

    fn minus<T: Display>(&self, value: T) -> String {
        format!("{self} - {value}")
    }

    fn times<T: Display>(&self, value: T) -> String {
        format!("{self} * {value}")
    }

    fn div<T: Display>(&self, value: T) -> String {
        format!("{self} / {value}")
    }
}

impl<T: Display> RegExtractor for T {
    fn capture_numbers<U: FromStr>(&self) -> Vec<U> {
        self.to_string().as_str().capture_numbers::<U>()
    }

    fn parens(&self) -> String {
        self.to_string().as_str().parens()
    }
    fn times<U: Display>(&self, value: U) -> String {
        self.to_string().as_str().times(value)
    }
    fn minus<U: Display>(&self, value: U) -> String {
        self.to_string().as_str().minus(value)
    }
    fn div<U: Display>(&self, value: U) -> String {
        self.to_string().as_str().div(value)
    }
    fn plus<U: Display>(&self, value: U) -> String {
        self.to_string().as_str().plus(value)
    }
}

//! Output formats for highlighted code.
//!
//! [`Html`] is the format the web frontend has always used. [`Spans`] keeps the text and
//! its highlight classes apart, for renderers that do not draw HTML (such as a native UI).

use crate::render::Class;
use std::ops::Range;

/// A sink for highlighted text. Every renderer in this crate can write into any `Markup`.
pub trait Markup: Default {
    /// Appends `text` highlighted with `class`, or as plain text when `class` is `None`.
    fn push(&mut self, class: Option<Class>, text: &str);

    /// Appends literal source text written by this crate's callers, such as `" => "`.
    fn push_raw(&mut self, text: &str);

    /// Appends a fragment of the same format.
    fn append(&mut self, other: &Self);

    /// Width used to decide whether a list fits on one line.
    fn width(&self) -> usize;

    /// Removes `suffix` from the end, if the fragment ends with it. Returns whether it did.
    fn strip_suffix(&mut self, suffix: &str) -> bool;

    /// Appends a value given as text. HTML may receive values that are already highlighted.
    fn push_value(&mut self, class: Class, value: &str) {
        self.push(Some(class), value);
    }
}

/// Highlighted HTML: `<span class="C{n}">` per highlighted token, text escaped.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Html(pub String);

impl Html {
    pub fn into_string(self) -> String {
        self.0
    }

    pub(crate) fn push_span(out: &mut String, class: Class, text: &str) {
        out.push_str("<span class=\"C");
        out.push_str(&(class as u8).to_string());
        out.push_str("\">");
        Self::push_escaped(out, text);
        out.push_str("</span>");
    }

    pub(crate) fn push_escaped(out: &mut String, text: &str) {
        for ch in text.chars() {
            match ch {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '"' => out.push_str("&quot;"),
                '\'' => out.push_str("&#39;"),
                _ => out.push(ch),
            }
        }
    }
}

impl Markup for Html {
    fn push(&mut self, class: Option<Class>, text: &str) {
        match class {
            Some(class) => Self::push_span(&mut self.0, class, text),
            None => Self::push_escaped(&mut self.0, text),
        }
    }

    /// Inserted unescaped, as the HTML highlighter always did.
    fn push_raw(&mut self, text: &str) {
        self.0.push_str(text);
    }

    fn append(&mut self, other: &Self) {
        self.0.push_str(&other.0);
    }

    /// Counts markup characters too. This keeps the existing HTML line breaking exactly.
    fn width(&self) -> usize {
        self.0.chars().count()
    }

    fn strip_suffix(&mut self, suffix: &str) -> bool {
        let found = self.0.ends_with(suffix);

        if found {
            self.0.truncate(self.0.len() - suffix.len());
        }

        found
    }

    fn push_value(&mut self, class: Class, value: &str) {
        // Values that are already highlighted HTML are inserted as they are.
        if value.contains("</span>") {
            self.0.push_str(value);
        } else {
            Self::push_span(&mut self.0, class, value);
        }
    }
}

/// A run of [`Spans::text`] with one highlight class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// Length in bytes.
    pub len: u32,
    pub class: Option<Class>,
}

/// Plain text plus the highlight class of each run. Adjacent runs never share a class.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Spans {
    pub text: String,
    pub spans: Vec<Span>,
}

impl Spans {
    /// Byte range and class of every run, in order.
    pub fn runs(&self) -> impl Iterator<Item = (Range<usize>, Option<Class>)> + '_ {
        let mut start = 0;

        self.spans.iter().map(move |span| {
            let end = start + span.len as usize;
            let run = (start..end, span.class);
            start = end;
            run
        })
    }
}

impl Markup for Spans {
    fn push(&mut self, class: Option<Class>, text: &str) {
        if text.is_empty() {
            return;
        }

        self.text.push_str(text);

        match self.spans.last_mut() {
            Some(last) if last.class == class => last.len += text.len() as u32,
            _ => self.spans.push(Span {
                len: text.len() as u32,
                class,
            }),
        }
    }

    fn push_raw(&mut self, text: &str) {
        self.push(None, text);
    }

    fn append(&mut self, other: &Self) {
        for (range, class) in other.runs() {
            self.push(class, &other.text[range]);
        }
    }

    fn width(&self) -> usize {
        self.text.chars().count()
    }

    fn strip_suffix(&mut self, suffix: &str) -> bool {
        let Some(last) = self.spans.last_mut() else {
            return suffix.is_empty();
        };

        // Only plain text that lies entirely in the last run can be stripped.
        if last.class.is_some()
            || (last.len as usize) < suffix.len()
            || !self.text.ends_with(suffix)
        {
            return false;
        }

        self.text.truncate(self.text.len() - suffix.len());
        last.len -= suffix.len() as u32;

        if last.len == 0 {
            self.spans.pop();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spans_merge_runs_and_strip_plain_suffixes() {
        let mut spans = Spans::default();

        spans.push(Some(Class::Keyword), "fn");
        spans.push(None, " ");
        spans.push(None, "x");
        spans.push(Some(Class::Number), "1");
        spans.push(None, ",\n\t");

        assert_eq!(spans.text, "fn x1,\n\t");
        assert_eq!(
            spans.runs().collect::<Vec<_>>(),
            [
                (0..2, Some(Class::Keyword)),
                (2..4, None),
                (4..5, Some(Class::Number)),
                (5..8, None),
            ]
        );

        assert!(spans.strip_suffix(",\n\t"));
        assert_eq!(spans.text, "fn x1");
        assert_eq!(spans.spans.len(), 3);
        // The suffix must be plain text; a highlighted `1` is never stripped.
        assert!(!spans.strip_suffix("1"));
    }

    #[test]
    fn html_escapes_text_but_not_raw_source() {
        let mut html = Html::default();

        html.push(Some(Class::Number), "<1>");
        html.push(None, "&");
        html.push_raw(" => ");

        assert_eq!(html.0, "<span class=\"C8\">&lt;1&gt;</span>&amp; => ");
    }
}

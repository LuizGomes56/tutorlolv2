//! Highlighted source code: the web app's `formulas/components/code.rs` and the
//! `code, pre` rules of `public/highliter.css`.
use crate::theme;
use gpui::{
    App, HighlightStyle, IntoElement, ParentElement, RenderOnce, SharedString, Styled, StyledText,
    Window, div, relative, rgb,
};
use std::{ops::Range, sync::Arc};
use tutorlolv2_codec::Spans;

/// `tab-size: 4`.
const TAB_SIZE: usize = 4;

/// Text and highlight ranges ready for [`StyledText`]. Cheap to clone.
#[derive(Clone)]
pub struct Highlighted {
    text: SharedString,
    highlights: Arc<[(Range<usize>, HighlightStyle)]>,
}

impl Highlighted {
    /// Converts codec spans, expanding tabs to the next multiple of [`TAB_SIZE`] columns.
    pub fn new(spans: &Spans) -> Self {
        let mut text = String::with_capacity(spans.text.len());
        let mut highlights = Vec::with_capacity(spans.spans.len());
        let mut column = 0;

        for (range, class) in spans.runs() {
            let start = text.len();

            for ch in spans.text[range].chars() {
                match ch {
                    '\t' => {
                        let width = TAB_SIZE - column % TAB_SIZE;
                        text.extend(std::iter::repeat_n(' ', width));
                        column += width;
                    }
                    '\n' => {
                        text.push('\n');
                        column = 0;
                    }
                    ch => {
                        text.push(ch);
                        column += 1;
                    }
                }
            }

            if let Some(class) = class {
                let (color, background) = theme::class_colors(class);

                highlights.push((
                    start..text.len(),
                    HighlightStyle {
                        color: Some(rgb(color).into()),
                        background_color: background.map(|bg| rgb(bg).into()),
                        ..Default::default()
                    },
                ));
            }
        }

        Self {
            text: text.into(),
            highlights: highlights.into(),
        }
    }

    /// Unhighlighted text, for errors.
    pub fn plain(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            highlights: Arc::new([]),
        }
    }

    pub fn styled_text(&self) -> StyledText {
        StyledText::new(self.text.clone()).with_highlights(self.highlights.iter().cloned())
    }
}

impl From<Result<Spans, Box<dyn core::error::Error>>> for Highlighted {
    fn from(result: Result<Spans, Box<dyn core::error::Error>>) -> Self {
        match result {
            Ok(spans) => Self::new(&spans),
            Err(error) => Self::plain(format!("Unable to render documentation: {error}")),
        }
    }
}

/// Monospace text styles shared by code blocks and hover docs.
pub fn code_text<E: Styled>(element: E) -> E {
    element
        .font_family(theme::CODE_FONT)
        .text_color(rgb(theme::CODE_TEXT))
        .line_height(relative(1.5))
}

/// A bordered code block: `px-4 py-3 border border-std-800`.
#[derive(IntoElement)]
pub struct Code(pub Highlighted);

impl RenderOnce for Code {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let block = div()
            .px_4()
            .py_3()
            .border_1()
            .border_color(rgb(theme::STD_800));

        code_text(block).child(self.0.styled_text())
    }
}

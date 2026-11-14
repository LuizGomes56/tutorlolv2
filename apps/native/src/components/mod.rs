pub mod cached_table;
pub mod code;
pub mod docs;
pub mod hover_docs;
pub mod image;
pub mod selector;
pub mod sidebar;
pub mod slider;
pub mod stack;
pub mod tables;
pub mod text_input;
pub mod tray;
pub mod ui;

use crate::theme;
use gpui::{
    BoxShadow, Div, FontWeight, IntoElement, ParentElement, SharedString, Styled, div, point, px,
    rgb,
};

/// The web `.box` shadow.
pub fn box_shadow() -> Vec<BoxShadow> {
    vec![BoxShadow {
        color: rgb(theme::BOX_SHADOW).into(),
        offset: point(px(0.0), px(0.0)),
        blur_radius: px(10.0),
        spread_radius: px(2.5),
    }]
}

/// A page section: `flex flex-col gap-6 p-6 box`.
pub fn section() -> Div {
    div()
        .flex()
        .flex_col()
        .gap_6()
        .p_6()
        // gpui paints shadows under the element too; CSS only paints them outside.
        .bg(rgb(theme::STD_900))
        .shadow(box_shadow())
}

/// The web `H2` component: `text-2xl text-std-200 font-medium`.
pub fn h2(text: impl Into<SharedString>) -> impl IntoElement {
    div()
        .text_2xl()
        .text_color(rgb(theme::STD_200))
        .font_weight(FontWeight::MEDIUM)
        .child(text.into())
}

/// A muted paragraph: `text-std-400`.
pub fn paragraph(text: impl Into<SharedString>) -> impl IntoElement {
    div().text_color(rgb(theme::STD_400)).child(text.into())
}

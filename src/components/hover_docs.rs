//! Documentation tooltips: the web app's `hover.rs`, `hover.js` and `#hoverdocs` element.
use crate::{
    components::code::{Highlighted, code_text},
    theme,
};
use gpui::{
    AnyView, App, AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div,
    prelude::*, rems, rgb,
};
use tutorlolv2::CastId;
use tutorlolv2_codec::Spans;

pub struct HoverDocs(Highlighted);

impl Render for HoverDocs {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        // `sm:max-w-md sm:max-h-96 p-2 leading-6 text-base border border-std-800 bg-std-900`
        let tooltip = div()
            .id("hover-docs")
            .max_w(rems(28.))
            .max_h(rems(24.))
            .overflow_y_scroll()
            .p_2()
            .text_base()
            .border_1()
            .border_color(rgb(theme::STD_800))
            .bg(rgb(theme::STD_900));

        code_text(tooltip).child(self.0.styled_text())
    }
}

/// Tooltip builder with the global definition of a champion, item or rune
/// (`Docs::ChampionGlobal`, `ItemGlobal`, `RuneGlobal` on the web).
pub fn global_docs<T: CastId>(id: T) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
    move |_, cx| {
        let docs = Highlighted::from(id.render_global_markup::<Spans>());
        cx.new(|_| HoverDocs(docs)).into()
    }
}

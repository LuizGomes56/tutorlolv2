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

/// Documentation shown only while Shift is held, so hovering alone does not cover the page.
///
/// gpui builds the tooltip after the hover delay regardless; until Shift is pressed the view is
/// empty (zero-sized). Modifier events only travel along the focus path, which never includes a
/// tooltip, so [`Shell`](crate::shell::Shell) refreshes the window when they change and this view
/// reads [`Window::modifiers`]. Once revealed it stays visible after Shift is released, so the
/// pointer can move into it and scroll.
pub struct HoverDocs {
    docs: Highlighted,
    revealed: bool,
}

impl Render for HoverDocs {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.revealed |= window.modifiers().shift;

        if !self.revealed {
            return div().into_any_element();
        }

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

        code_text(tooltip)
            .child(self.docs.styled_text())
            .into_any_element()
    }
}

/// Shift-gated tooltip builder with the global definition of a champion, item or rune
/// (`Docs::ChampionGlobal`, `ItemGlobal`, `RuneGlobal` on the web).
pub fn global_docs<T: CastId>(id: T) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
    move |_, cx| {
        let docs = Highlighted::from(id.render_global_markup::<Spans>());
        cx.new(|_| HoverDocs {
            docs,
            revealed: false,
        })
        .into()
    }
}

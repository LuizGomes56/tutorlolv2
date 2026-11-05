//! The Formulas page: the web app's `formulas/page.rs`.
mod champions;
mod values;

use crate::formulas::{champions::ChampionFormulas, values::ValueFormulas};
use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};
use tutorlolv2::{ItemId, RuneId};

pub struct Formulas {
    champions: Entity<ChampionFormulas>,
    items: Entity<ValueFormulas<ItemId>>,
    runes: Entity<ValueFormulas<RuneId>>,
}

impl Formulas {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            champions: cx.new(ChampionFormulas::new),
            items: cx.new(ValueFormulas::new),
            runes: cx.new(ValueFormulas::new),
        }
    }
}

impl Render for Formulas {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .child(self.champions.clone())
            .child(self.items.clone())
            .child(self.runes.clone())
    }
}

/// The web hides entry names below Tailwind's `md` breakpoint (`hidden md:inline`).
fn is_compact(window: &Window) -> bool {
    window.viewport_size().width < gpui::px(768.0)
}

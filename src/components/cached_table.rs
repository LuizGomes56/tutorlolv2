//! A table drawn as a cached view, redrawn only when its columns change.
//!
//! The live game recalculates once per second, but hover changes and tooltips draw frames in
//! between. Each live table is one of these views, like the calculator's `DamageTables`.
//! A cached view is laid out from its style alone, so the height comes from the row count:
//! every row, header included, is exactly [`ROW_HEIGHT`] tall.
use crate::components::{
    hover_docs,
    tables::{Column, ROW_HEIGHT, table},
};
use gpui::{
    AnyView, App, Context, Entity, IntoElement, ParentElement, Render, StyleRefinement, Styled,
    Window, div, px,
};
use std::rc::Rc;

pub type BuildColumns = Rc<dyn Fn() -> Vec<Column>>;

pub struct CachedTable {
    id: &'static str,
    rows: usize,
    build: Option<BuildColumns>,
}

impl CachedTable {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            rows: 0,
            build: None,
        }
    }

    /// Replaces the table. `rows` excludes the header.
    pub fn set(&mut self, rows: usize, build: BuildColumns, cx: &mut Context<Self>) {
        self.rows = rows;
        self.build = Some(build);
        cx.notify();
    }

    pub fn view(this: &Entity<Self>, cx: &mut App) -> AnyView {
        let rows = this.read(cx).rows;
        let style = StyleRefinement::default()
            .w_full()
            .h(px((rows + 1) as f32 * ROW_HEIGHT));

        hover_docs::cached_view(this, style, cx)
    }
}

impl Render for CachedTable {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        hover_docs::view_rendered(cx.entity_id(), cx);

        let columns = self.build.as_ref().map(|build| build()).unwrap_or_default();

        div().size_full().child(table(self.id, columns))
    }
}

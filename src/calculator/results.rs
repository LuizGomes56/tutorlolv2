//! The enemy, monster and turret tables, as their own view.
//!
//! gpui rebuilds the elements of every view on each frame it draws, unlike yew, which only
//! re-renders components whose props changed. Typing in an input, its cursor blinking or a
//! tooltip opening draws a frame, which would rebuild hundreds of table cells. The tables are
//! therefore a separate view that the calculator draws with [`hover_docs::cached_view`]: gpui
//! reuses the previous frame's layout and paint until [`DamageTables::set_game`] notifies it.
use crate::{
    calculator::{
        docs,
        tables::{Cell, Column, ROW_HEIGHT, damage_columns, empty_table, table, turret_columns},
    },
    components::{
        box_shadow, hover_docs,
        image::{Entry, image},
    },
    theme,
};
use gpui::{
    AnyElement, AnyView, App, Context, Entity, EventEmitter, IntoElement, ParentElement, Pixels,
    Render, StyleRefinement, Styled, Window, div, prelude::*, px, rgb,
};
use std::rc::Rc;
use tutorlolv2::{
    CtxVar, L_MSTR,
    model::{Damages, OutputGame},
};

/// `gap-4` between the tables.
const GAP: f32 = 16.0;

/// `MONSTER_HEADERS`: the monsters of each monster row. Only the row count and the widest row
/// matter until their images exist.
const MONSTER_HEADERS: [usize; L_MSTR] = [4, 4, 4, 2, 1, 1, 1];

const MONSTER_COUNT: usize = {
    let mut i = 0;
    let mut max = 0;

    while i < L_MSTR {
        if MONSTER_HEADERS[i] > max {
            max = MONSTER_HEADERS[i];
        }

        i += 1;
    }

    max
};

/// A click on an enemy's portrait, which selects it in the enemy panel.
pub struct SelectEnemy(pub usize);

#[derive(Default)]
pub struct DamageTables {
    /// The last successful result. `None` shows the placeholders.
    game: Option<Rc<OutputGame>>,
}

impl EventEmitter<SelectEnemy> for DamageTables {}

impl DamageTables {
    pub fn set_game(&mut self, game: Option<Rc<OutputGame>>, cx: &mut Context<Self>) {
        self.game = game;
        cx.notify();
    }

    /// A cached view is laid out from its style alone, so its height is computed here. Every
    /// row is exactly [`ROW_HEIGHT`] tall.
    fn height(&self) -> Pixels {
        let table = |rows: usize| (rows + 1) as f32 * ROW_HEIGHT;
        let monsters = table(L_MSTR);
        let turrets = table(1);

        // Before the first result only the monster and turret placeholders are shown.
        match &self.game {
            Some(game) => px(table(game.enemies.len()) + monsters + turrets + 2.0 * GAP),
            None => px(monsters + turrets + GAP),
        }
    }

    /// The tables as an element that is redrawn only after [`Self::set_game`].
    pub fn view(this: &Entity<Self>, cx: &mut App) -> AnyView {
        let style = StyleRefinement::default()
            .w_full()
            .h(this.read(cx).height());

        hover_docs::cached_view(this, style, cx)
    }

    fn tables(&self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let Some(game) = self.game.clone() else {
            return vec![
                table_box(empty_table("monsters-empty", MONSTER_HEADERS.len())),
                table_box(empty_table("turrets-empty", 1)),
            ];
        };

        let player = &game.current_player;
        let damages = &game.damages;
        let enemy_rows = (0..game.enemies.len())
            .map(|i| damages.row(L_MSTR + i))
            .collect::<Vec<Damages<'_>>>();

        let mut enemy_columns = damage_columns(
            &enemy_rows,
            player.champion_id,
            player.adaptive_type,
            &game.items_meta,
            &game.runes_meta,
        );

        // The first column: each enemy's portrait, which selects it in the enemy panel.
        let portraits = game
            .enemies
            .iter()
            .zip(&enemy_rows)
            .enumerate()
            .map(|(i, (enemy, damages))| {
                let champion = enemy.champion_id;
                let ctx = damages.ctx;

                Cell::new(
                    div()
                        .id(("enemy-portrait", i))
                        .cursor_pointer()
                        .on_click(cx.listener(move |_, _, _, cx| cx.emit(SelectEnemy(i))))
                        .child(image(Entry::image_source(&champion)).size(px(32.))),
                )
                .docs(move || {
                    docs::cell_blocks(&CtxVar::ENEMY_VARS, &ctx, docs::Docs::global(champion))
                })
            })
            .collect();

        enemy_columns.insert(
            0,
            Column {
                header: Cell::empty(),
                cells: portraits,
            },
        );

        let monster_rows = (0..L_MSTR).map(|i| damages.row(i)).collect::<Vec<_>>();
        let mut monster_columns = damage_columns(
            &monster_rows,
            player.champion_id,
            player.adaptive_type,
            &game.items_meta,
            &game.runes_meta,
        );

        // Monster portraits exist only as AVIF: their columns stay empty for now.
        for _ in 0..MONSTER_COUNT {
            monster_columns.insert(
                0,
                Column {
                    header: Cell::empty(),
                    cells: (0..L_MSTR).map(|_| Cell::empty()).collect(),
                },
            );
        }

        vec![
            table_box(table("enemy-damages", enemy_columns)),
            table_box(table("monster-damages", monster_columns)),
            table_box(table(
                "turret-damages",
                turret_columns(&game.tower_damages, player.adaptive_type),
            )),
        ]
    }
}

/// `box overflow-auto` around a table.
fn table_box(table: impl IntoElement) -> AnyElement {
    div()
        .bg(rgb(theme::STD_900))
        .shadow(box_shadow())
        .child(table)
        .into_any_element()
}

impl Render for DamageTables {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        hover_docs::view_rendered(cx.entity_id(), cx);

        div()
            .flex()
            .flex_col()
            .size_full()
            .gap(px(GAP))
            .children(self.tables(cx))
    }
}

//! The enemy, monster and turret tables, as their own view.
//!
//! gpui rebuilds the elements of every view on each frame it draws, unlike yew, which only
//! re-renders components whose props changed. Typing in an input, its cursor blinking or a
//! tooltip opening draws a frame, which would rebuild hundreds of table cells. The tables are
//! therefore a separate view that the calculator draws with [`hover_docs::cached_view`]: gpui
//! reuses the previous frame's layout and paint until [`DamageTables::set_game`] notifies it.
use crate::{
    components::{
        box_shadow, docs, hover_docs,
        image::{Entry, image, monster_image},
        tables::{Cell, Column, ROW_HEIGHT, damage_columns, empty_table, table, turret_columns},
    },
    theme,
};
use gpui::{
    AnyElement, AnyView, App, Context, Entity, EventEmitter, IntoElement, ParentElement, Pixels,
    Render, StyleRefinement, Styled, Window, div, prelude::*, px, rgb,
};
use math::{
    CtxVar, L_MSTR,
    model::{Damages, OutputGame},
};
use std::rc::Rc;

/// `gap-4` between the tables.
const GAP: f32 = 16.0;

/// Rows of the monster table before the first result, as many as its usual groups.
const MONSTER_PLACEHOLDER_ROWS: usize = 7;

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
        let turrets = table(1);

        // Before the first result only the monster and turret placeholders are shown.
        match &self.game {
            Some(game) => {
                let monsters = table(game.damages.monster_groups().len());
                px(table(game.enemies.len()) + monsters + turrets + 2.0 * GAP)
            }
            None => px(table(MONSTER_PLACEHOLDER_ROWS) + turrets + GAP),
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
                table_box(empty_table("monsters-empty", MONSTER_PLACEHOLDER_ROWS)),
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

        // Monsters whose damages are all the same share one row
        let monster_groups = damages.monster_groups();
        let monster_rows = monster_groups
            .iter()
            .map(|group| damages.row(group.row))
            .collect::<Vec<_>>();

        let mut monster_columns = damage_columns(
            &monster_rows,
            player.champion_id,
            player.adaptive_type,
            &game.items_meta,
            &game.runes_meta,
        );

        // The portraits of each row's monsters, then their names
        monster_columns.insert(
            0,
            Column {
                header: Cell::empty(),
                cells: monster_groups
                    .iter()
                    .map(|group| {
                        let names = group
                            .monsters
                            .iter()
                            .map(|monster| monster.name())
                            .collect::<Vec<_>>()
                            .join(", ");

                        Cell::new(
                            div()
                                .px_2()
                                .flex()
                                .items_center()
                                .gap_1()
                                .children(
                                    group
                                        .monsters
                                        .iter()
                                        .map(|&monster| monster_image(monster).size(px(24.))),
                                )
                                .child(div().text_xs().whitespace_nowrap().child(names)),
                        )
                    })
                    .collect(),
            },
        );

        vec![
            table_box(table("enemy-damages", enemy_columns)),
            table_box(table("monster-damages", monster_columns)),
            table_box(table(
                "turret-damages",
                turret_columns(&game.tower_damages, player.tower_damage_type()),
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

//! The combo stack: the web app's `components/stack/` (`mod.rs`, `insert.rs`, `remover.rs`,
//! `table.rs` and `storage.rs`).
//!
//! A stack is a list of damage sources (abilities, items, runes, attacks and Ignite) whose
//! damages are summed against each enemy. Each champion's stack can be saved; it is loaded
//! when a result for that champion arrives, or else the champion's default combo is used.
use crate::{
    calculator::{
        Calculator,
        docs::{Builtin, DocView, Docs},
        state::Tray,
        tables::{Cell, Column, table},
    },
    components::{
        box_shadow,
        hover_docs::HoverDocsExt,
        image::{Entry, ability_image, asset_image, image},
    },
    theme,
};
use bincode::{Decode, Encode};
use gpui::{
    AnyElement, ClickEvent, Context, ElementId, FontWeight, IntoElement, ParentElement, Pixels,
    Styled, Window, div, prelude::*, px, rgb, rgba,
};
use std::num::NonZeroU8;
use tutorlolv2::{
    AbilityId, ChampionId, ComboElement, ItemId, L_MSTR, RuneId, TypeMetadata, ignite,
    model::{Damages, OutputGame},
};

/// Tailwind's `2xl` breakpoint: insert, remover and table side by side.
const WIDE_BREAKPOINT: f32 = 1536.0;

/// One damage source. The variant order is the web's, so saved records share its encoding.
#[derive(Clone, Copy, Debug, Decode, Encode, PartialEq)]
pub enum StackValue {
    Ability {
        slot: usize,
        ability_id: AbilityId,
    },
    /// An index into the damage row's items: two per item, the second for its max damage.
    Item(usize, ItemId),
    /// An index into the damage row's runes, two per rune like items.
    Rune(usize, RuneId),
    BasicAttack,
    CritStrike,
    OnhitMin,
    OnhitMax,
    Ignite,
}

impl StackValue {
    /// The damage of this source in a row of the damage table.
    fn damage(self, damages: &Damages<'_>, level: NonZeroU8) -> i32 {
        let at = |values: &[i32], index: usize| values.get(index).copied().unwrap_or(0);

        match self {
            Self::Ability { slot, .. } => at(damages.abilities, slot),
            Self::Item(index, _) => at(damages.items, index),
            Self::Rune(index, _) => at(damages.runes, index),
            Self::BasicAttack => damages.attacks.basic_attack,
            Self::CritStrike => damages.attacks.critical_strike,
            Self::OnhitMin => damages.attacks.onhit_damage.minimum_damage,
            Self::OnhitMax => damages.attacks.onhit_damage.maximum_damage,
            Self::Ignite => ignite(level),
        }
    }

    /// Whether this is the max damage of an item or of on-hit, which shows `MAX`.
    fn is_max(self) -> bool {
        match self {
            Self::Item(index, item) => index % 2 == 1 && item.deals_max_damage(),
            Self::OnhitMax => true,
            _ => false,
        }
    }

    fn icon(self, champion_id: ChampionId) -> AnyElement {
        match self {
            Self::Ability { ability_id, .. } => ability_image(champion_id, ability_id)
                .size_full()
                .into_any_element(),
            Self::Item(_, item) => image(item.image_source()).size_full().into_any_element(),
            Self::Rune(_, rune) => image(rune.image_source()).size_full().into_any_element(),
            Self::BasicAttack => asset_image("other/basic_attack.png")
                .size_full()
                .into_any_element(),
            Self::CritStrike => asset_image("stats/crit_chance.svg")
                .size_full()
                .into_any_element(),
            Self::OnhitMin | Self::OnhitMax => asset_image("stats/onhit.svg")
                .size_full()
                .into_any_element(),
            // `other/ignite.avif`: AVIF only, so the slot stays empty.
            Self::Ignite => div().into_any_element(),
        }
    }

    /// `ImageType::docs`.
    fn docs(self, champion_id: ChampionId) -> Docs {
        match self {
            Self::Ability { ability_id, .. } => Docs::Champion(champion_id, ability_id),
            Self::Item(_, item) => Docs::global(item),
            Self::Rune(_, rune) => Docs::global(rune),
            Self::BasicAttack => Docs::Builtin(Builtin::BasicAttack, DocView::Global),
            Self::CritStrike => Docs::Builtin(Builtin::CriticalStrike, DocView::Global),
            Self::OnhitMin | Self::OnhitMax => Docs::Builtin(Builtin::Onhit, DocView::Global),
            Self::Ignite => Docs::Builtin(Builtin::Ignite, DocView::Global),
        }
    }
}

/// Saved stacks, by champion (`components/stack/storage.rs`).
mod saved {
    use super::StackValue;
    use crate::storage;
    use std::collections::HashMap;
    use tutorlolv2::ChampionId;

    const KIND: u8 = 1;
    const FILE: &str = "stack.bin";

    type Store = HashMap<ChampionId, Vec<StackValue>>;

    pub fn load(champion_id: ChampionId) -> Option<Vec<StackValue>> {
        storage::read_record::<Store>(FILE, KIND)?.remove(&champion_id)
    }

    pub fn save(champion_id: ChampionId, values: &[StackValue]) -> std::io::Result<()> {
        let mut store = storage::read_record::<Store>(FILE, KIND).unwrap_or_default();

        store.insert(champion_id, values.to_vec());
        storage::write_record(FILE, KIND, &store)
    }
}

#[derive(Default)]
pub struct Stack {
    /// The champion of the result the stack was last synced with.
    champion_id: Option<ChampionId>,
    values: Tray<StackValue>,
    /// The champion combo that "Default" uses next.
    combo_index: usize,
}

/// One item or rune per metadata entry, at the even (minimum damage) index.
fn metadata_values<T: Copy>(
    metadata: &[TypeMetadata<T>],
    value: fn(usize, T) -> StackValue,
) -> impl Iterator<Item = StackValue> + '_ {
    metadata
        .iter()
        .enumerate()
        .map(move |(i, metadata)| value(i * 2, metadata.kind))
}

impl Stack {
    /// `Stack::new`: a combo of the champion, then every item and rune of the result.
    fn default_values(
        &self,
        champion_id: ChampionId,
        combo_index: usize,
        game: &OutputGame,
    ) -> Tray<StackValue> {
        let combos = champion_id.combos();
        let combo = combos.get(combo_index % combos.len().max(1));

        combo
            .into_iter()
            .flat_map(|combo| combo.iter())
            .filter_map(|&element| match element {
                ComboElement::Ability(ability_id) => champion_id
                    .index_of_ability(ability_id)
                    .map(|slot| StackValue::Ability { slot, ability_id }),
                ComboElement::Attack => Some(StackValue::BasicAttack),
            })
            .chain(metadata_values(&game.items_meta, StackValue::Item))
            .chain(metadata_values(&game.runes_meta, StackValue::Rune))
            .collect()
    }

    /// `Stack::reconcile`: whether a value still indexes the same source in `game`.
    fn is_valid(value: StackValue, champion_id: ChampionId, game: &OutputGame) -> bool {
        match value {
            StackValue::Item(index, item) => {
                game.items_meta
                    .get(index / 2)
                    .is_some_and(|metadata| metadata.kind == item)
                    && (index % 2 == 0 || item.deals_max_damage())
            }
            StackValue::Rune(index, rune) => {
                game.runes_meta
                    .get(index / 2)
                    .is_some_and(|metadata| metadata.kind == rune)
                    && (index % 2 == 0 || rune.deals_max_damage())
            }
            StackValue::Ability { slot, ability_id } => {
                champion_id.index_of_ability(ability_id) == Some(slot)
            }
            _ => true,
        }
    }

    /// Follows a new result. For another champion, loads its saved stack or its default
    /// combo. Otherwise drops values whose item or rune is gone or moved, as items change.
    pub fn sync(&mut self, game: &OutputGame) {
        let champion_id = game.current_player.champion_id;

        if self.champion_id != Some(champion_id) {
            self.champion_id = Some(champion_id);
            self.values = match saved::load(champion_id) {
                Some(values) => values.into_iter().collect(),
                None => self.default_values(champion_id, self.combo_index, game),
            };
        }

        self.values
            .retain(|value| Self::is_valid(value, champion_id, game));
    }

    fn total(&self, damages: &Damages<'_>, level: NonZeroU8) -> i32 {
        self.values
            .values()
            .iter()
            .map(|value| value.damage(damages, level))
            .sum()
    }
}

/// `.btn-stack`: `w-10 h-10 border border-std-800 bg-std-900/60`, with a 32px icon and `MAX`.
fn stack_button(
    id: impl Into<ElementId>,
    value: StackValue,
    champion_id: ChampionId,
    rounded: bool,
    hover_color: u32,
    hover_border: u32,
) -> gpui::Stateful<gpui::Div> {
    let docs = value.docs(champion_id);

    div()
        .id(id.into())
        .relative()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size_10()
        .border_1()
        .border_color(rgb(theme::STD_800))
        .bg(rgba(0x18181b99))
        .when(rounded, |button| button.rounded_lg())
        .cursor_pointer()
        .hover(move |style| style.bg(rgba(hover_color)).border_color(rgba(hover_border)))
        .docs(move || docs.blocks())
        .child(
            div()
                .size_8()
                .overflow_hidden()
                .rounded_sm()
                .child(value.icon(champion_id)),
        )
        .when(value.is_max(), |button| {
            // `img-letter text-sm`
            button.child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(theme::TEXT))
                    .child("MAX"),
            )
        })
}

/// A title with a hint below it, as both stack panels start.
fn heading(title: &'static str, hint: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .whitespace_nowrap()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme::STD_100))
                .child(title),
        )
        .child(
            div()
                .whitespace_nowrap()
                .text_xs()
                .text_color(rgb(theme::STD_400))
                .child(hint),
        )
}

/// `StackButton`: `px-3 py-1.5 rounded-lg border border-std-800 bg-std-900/70 text-xs`.
fn action_button(
    id: &'static str,
    text: &'static str,
    hover_border: u32,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .px_3()
        .py_1p5()
        .rounded_lg()
        .border_1()
        .border_color(rgb(theme::STD_800))
        .bg(rgba(0x18181bb3))
        .text_xs()
        .text_center()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(theme::STD_200))
        .cursor_pointer()
        .hover(move |style| {
            style
                .bg(rgba(theme::HOVER))
                .border_color(rgba(hover_border))
        })
        .on_click(on_click)
        .child(text)
}

/// Hover colors (`/10` backgrounds, `/75` borders).
const BLUE_BORDER: u32 = 0x3b82f6ff;
const ROSE_BACKGROUND: u32 = 0xf43f5e1a;
const ROSE_BORDER: u32 = 0xf43f5ebf;
const SKY_BORDER: u32 = 0x0ea5e9bf;
const EMERALD_BORDER: u32 = 0x10b981bf;

impl Calculator {
    fn push_stack(&mut self, value: StackValue, cx: &mut Context<Self>) {
        self.stack.values.push(value);
        cx.notify();
    }

    fn remove_stack(&mut self, id: u64, cx: &mut Context<Self>) {
        self.stack.values.remove_by_id(id);
        cx.notify();
    }

    fn clear_stack(&mut self, cx: &mut Context<Self>) {
        self.stack.values.clear();
        cx.notify();
    }

    fn save_stack(&mut self, cx: &mut Context<Self>) {
        if let Some(champion_id) = self.stack.champion_id {
            self.stack_error = saved::save(champion_id, self.stack.values.values())
                .err()
                .map(|error| format!("Could not save: {error}").into());
            cx.notify();
        }
    }

    /// The champion's next combo, cycling through them on each click.
    fn default_stack(&mut self, cx: &mut Context<Self>) {
        let (Some(champion_id), Some(game)) = (self.stack.champion_id, self.game.clone()) else {
            return;
        };

        let index = self.stack.combo_index % champion_id.combos().len().max(1);

        self.stack.values = self.stack.default_values(champion_id, index, &game);
        self.stack.combo_index = index + 1;
        cx.notify();
    }

    /// `box overflow-auto` with the insert panel, the remover and the totals table.
    pub(super) fn stack_box(
        &self,
        game: &OutputGame,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let insert = self.stack_insert(game, cx);
        let remover = self.stack_remover(cx);
        let totals = self.stack_table(game, cx);

        let wide = window.viewport_size().width >= px(WIDE_BREAKPOINT);

        // `grid grid-cols-1 items-start gap-4 2xl:grid-cols-3`
        div()
            .flex()
            .gap_4()
            .bg(rgb(theme::STD_900))
            .shadow(box_shadow())
            .map(|grid| match wide {
                true => grid.items_start(),
                false => grid.flex_col(),
            })
            .child(column(wide).child(insert))
            .child(column(wide).child(remover))
            .child(column(wide).child(totals))
            .into_any_element()
    }

    /// `StackInsert`.
    fn stack_insert(&self, game: &OutputGame, cx: &mut Context<Self>) -> gpui::Div {
        let champion_id = game.current_player.champion_id;
        let mut next_id: usize = 0;
        let mut button = |value: StackValue| {
            next_id += 1;

            stack_button(
                ("stack-insert", next_id),
                value,
                champion_id,
                false,
                theme::HOVER,
                BLUE_BORDER,
            )
            .on_click(cx.listener(move |this, _, _, cx| this.push_stack(value, cx)))
            .into_any_element()
        };

        let abilities = champion_id
            .abilities()
            .iter()
            .enumerate()
            .map(|(slot, metadata)| {
                button(StackValue::Ability {
                    slot,
                    ability_id: metadata.kind,
                })
            })
            .collect::<Vec<_>>();

        let mut items = Vec::new();

        for (i, metadata) in game.items_meta.iter().enumerate() {
            items.push(button(StackValue::Item(i * 2, metadata.kind)));

            if metadata.kind.deals_max_damage() {
                items.push(button(StackValue::Item(i * 2 + 1, metadata.kind)));
            }
        }

        let runes = metadata_values(&game.runes_meta, StackValue::Rune)
            .map(&mut button)
            .collect::<Vec<_>>();

        let other = [
            StackValue::BasicAttack,
            StackValue::CritStrike,
            StackValue::OnhitMin,
            StackValue::OnhitMax,
            StackValue::Ignite,
        ]
        .map(&mut button);

        let sections = [
            ("Abilities", abilities),
            ("Items", items),
            ("Runes", runes),
            ("Other", other.into()),
        ]
        .into_iter()
        .filter(|(_, buttons)| !buttons.is_empty())
        .enumerate()
        .map(|(i, (title, buttons))| {
            // `flex flex-col gap-2 pt-4 border-t border-std-800/70 first:pt-0 first:border-t-0`
            div()
                .flex()
                .flex_col()
                .gap_2()
                .when(i > 0, |section| {
                    section.pt_4().border_t_1().border_color(rgba(0x27272ab3))
                })
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme::STD_400))
                        .child(title.to_uppercase()),
                )
                .child(div().flex().flex_wrap().gap_2().children(buttons))
        });

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .child(heading("Add to combo list", "Click icons to add effects"))
            .child(div().flex().flex_col().gap_4().children(sections))
    }

    /// `StackRemover`.
    fn stack_remover(&self, cx: &mut Context<Self>) -> gpui::Div {
        let champion_id = self.stack.champion_id.unwrap_or_default();

        let values = self.stack.values.iter().map(|(id, value)| {
            stack_button(
                ("stack-value", id as usize),
                value,
                champion_id,
                true,
                ROSE_BACKGROUND,
                ROSE_BORDER,
            )
            .on_click(cx.listener(move |this, _, _, cx| this.remove_stack(id, cx)))
        });

        // `px-3 py-1.5 rounded-lg border border-std-800 bg-std-900/70 font-mono`
        let count = div()
            .px_3()
            .py_1p5()
            .rounded_lg()
            .border_1()
            .border_color(rgb(theme::STD_800))
            .bg(rgba(0x18181bb3))
            .text_xs()
            .text_center()
            .font_family(theme::CODE_FONT)
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(theme::STD_200))
            .child(self.stack.values.len().to_string());

        let buttons = div()
            .grid()
            .grid_cols(2)
            .gap_2()
            .child(count)
            .child(action_button(
                "stack-save",
                "Save",
                SKY_BORDER,
                cx.listener(|this, _, _, cx| this.save_stack(cx)),
            ))
            .child(action_button(
                "stack-default",
                "Default",
                EMERALD_BORDER,
                cx.listener(|this, _, _, cx| this.default_stack(cx)),
            ))
            .child(action_button(
                "stack-clear",
                "Clear",
                ROSE_BORDER,
                cx.listener(|this, _, _, cx| this.clear_stack(cx)),
            ));

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .gap_3()
                    .child(heading("Defined combo", "Click icons to remove"))
                    .child(buttons),
            )
            .children(
                self.stack_error
                    .clone()
                    .map(|error| div().text_xs().text_color(rgb(theme::STD_400)).child(error)),
            )
            // `flex-1 rounded-lg border border-dashed border-std-800 p-2 flex flex-wrap gap-2`
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .content_start()
                    .gap_2()
                    .min_h(px(56.))
                    .p_2()
                    .rounded_lg()
                    .border_1()
                    .border_dashed()
                    .border_color(rgb(theme::STD_800))
                    .children(values),
            )
    }

    /// `StackTable`: the stack's total damage against each enemy.
    fn stack_table(&self, game: &OutputGame, cx: &mut Context<Self>) -> gpui::Div {
        let level = game.current_player.level;
        let header = |text: &'static str| Cell::new(div().child(text));
        let mut portraits = Vec::new();
        let mut totals = Vec::new();
        let mut healths = Vec::new();
        let mut percents = Vec::new();

        for (i, enemy) in game.enemies.iter().enumerate() {
            let damages = game.damages.row(L_MSTR + i);
            let total = self.stack.total(&damages, level);
            let max_health = enemy.current_stats.max_health as i32;
            let percent = (total as f32 / max_health as f32 * 100.0) as i32;
            let champion = enemy.champion_id;

            portraits.push(Cell::new(
                div()
                    .id(("stack-portrait", i))
                    .cursor_pointer()
                    .hover_docs(champion)
                    .on_click(cx.listener(move |this, _, _, cx| this.select_enemy(i, cx)))
                    .child(image(champion.image_source()).size(px(32.))),
            ));
            totals.push(Cell::new(div().child(total.to_string())));
            healths.push(Cell::new(div().child((max_health - total).to_string())));
            percents.push(Cell::new(div().child(format!("{percent}%"))));
        }

        let columns = vec![
            Column {
                header: Cell::empty(),
                cells: portraits,
            },
            Column {
                header: header("Damage"),
                cells: totals,
            },
            Column {
                header: header("Health"),
                cells: healths,
            },
            Column {
                header: header("% HP"),
                cells: percents,
            },
        ];

        div().p_4().child(table("stack-totals", columns))
    }
}

/// A column of the stack box: equal thirds when wide, full width otherwise.
fn column(wide: bool) -> gpui::Div {
    div().min_w_0().map(|column| match wide {
        true => column.flex_1().flex_basis(Pixels::ZERO),
        false => column.w_full(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn stack_values_keep_the_web_encoding_order() {
        let config = bincode::config::standard();
        let encode = |value: StackValue| bincode::encode_to_vec(value, config).unwrap()[0];

        assert_eq!(encode(StackValue::BasicAttack), 3);
        assert_eq!(encode(StackValue::Ignite), 7);

        let values = vec![
            StackValue::Item(1, ItemId::default()),
            StackValue::Rune(2, RuneId::default()),
            StackValue::OnhitMax,
        ];
        let store = HashMap::from([(ChampionId::default(), values.clone())]);
        let bytes = bincode::encode_to_vec(&store, config).unwrap();
        let (decoded, _): (HashMap<ChampionId, Vec<StackValue>>, _) =
            bincode::decode_from_slice(&bytes, config).unwrap();

        assert_eq!(decoded[&ChampionId::default()], values);
    }
}

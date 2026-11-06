//! Item and rune pickers: the web app's `inputs/item_selector.rs` and `inputs/rune_selector.rs`.
//!
//! A modal over the whole window with a search box, the options (items also get stat filters)
//! and, on the right, the tray of every player: their champion, recommendation buttons per
//! position and their current items or runes, which are removed by clicking them. Options are
//! added to the highlighted player, which is changed by clicking a champion in the tray.
//! It closes with Escape or a click outside.
use crate::{
    calculator::{Calculator, Target, state::Tray},
    components::{
        hover_docs::HoverDocsExt,
        image::{Entry, asset_image, image, position_image},
        selector::contains_ascii_case_insensitive,
        text_input::{TextInput, TextInputEvent},
    },
    theme,
};
use gpui::{
    AnyElement, Context, Entity, Focusable, FontWeight, IntoElement, KeyDownEvent, MouseButton,
    ParentElement, SharedString, Styled, Subscription, Window, anchored, deferred, div, point,
    prelude::*, px, rems, rgb, rgba, svg,
};
use std::sync::OnceLock;
use strum::VariantArray;
use tutorlolv2::{ChampionId, GameMap, ItemId, Position, RuneId, StatName, ValueId};

/// Above the page, below the tooltip layer (priority 2).
const MODAL_PRIORITY: usize = 1;
/// `w-9 h-9`.
const OPTION_SIZE: f32 = 36.0;
/// `grid-cols-6 gap-2`.
const TRAY_COLUMNS: f32 = 6.0;
const TRAY_GAP: f32 = 8.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PickerKind {
    Items,
    Runes,
}

pub struct Picker {
    kind: PickerKind,
    /// The player that options are added to.
    target: Target,
    search: Entity<TextInput>,
    query: SharedString,
    /// Stat filters: items must have every selected stat.
    filters: Vec<StatName>,
    _search_changes: Subscription,
}

/// `ItemFilter`'s buttons, in order.
const FILTERS: [StatName; 22] = [
    StatName::AdaptiveForce,
    StatName::AttackDamage,
    StatName::CritChance,
    StatName::CritDamage,
    StatName::AttackSpeed,
    StatName::ArmorPenetration,
    StatName::Lethality,
    StatName::AbilityPower,
    StatName::Mana,
    StatName::BaseManaRegen,
    StatName::MagicPenetration,
    StatName::Health,
    StatName::BaseHealthRegen,
    StatName::HealAndShieldPower,
    StatName::Armor,
    StatName::MagicResist,
    StatName::AbilityHaste,
    StatName::MoveSpeed,
    StatName::Tenacity,
    StatName::Omnivamp,
    StatName::LifeSteal,
    StatName::GoldPer10Seconds,
];

/// `ImageType::StatsFilter`.
const fn filter_icon(stat: StatName) -> &'static str {
    match stat {
        StatName::AbilityHaste => "stats/ability_haste.svg",
        StatName::AbilityPower => "stats/ability_power.svg",
        StatName::AdaptiveForce => "stats/adaptive_force.svg",
        StatName::Armor => "stats/armor.svg",
        StatName::ArmorPenetration | StatName::Lethality => "stats/armor_penetration.svg",
        StatName::AttackDamage => "stats/attack_damage.svg",
        StatName::AttackSpeed => "stats/attack_speed.svg",
        StatName::BaseHealthRegen => "stats/health_regeneration.svg",
        StatName::BaseManaRegen => "stats/mana_regeneration.svg",
        StatName::CritChance => "stats/crit_chance.svg",
        StatName::CritDamage => "stats/crit_damage.svg",
        StatName::GoldPer10Seconds => "stats/gold.svg",
        StatName::HealAndShieldPower => "stats/heal_and_shield_power.svg",
        StatName::Health => "stats/health.svg",
        StatName::LifeSteal => "stats/life_steal.svg",
        StatName::MagicPenetration | StatName::MagicPenetrationPercent => {
            "stats/magic_penetration.svg"
        }
        StatName::MagicResist => "stats/magic_resist.svg",
        StatName::Mana => "stats/mana.svg",
        StatName::MoveSpeed | StatName::MoveSpeedPercent => "stats/move_speed.svg",
        StatName::Omnivamp => "stats/omnivamp.svg",
        StatName::Tenacity => "stats/tenacity.svg",
    }
}

/// Items that can be bought on Summoner's Rift, the only ones the web offers.
fn purchasable_items() -> &'static [ItemId] {
    static ITEMS: OnceLock<Vec<ItemId>> = OnceLock::new();

    ITEMS.get_or_init(|| {
        ItemId::VARIANTS
            .iter()
            .copied()
            .filter(|item| {
                !item.data().custom && item.purchasable() && item.has_map(GameMap::SummonersRift)
            })
            .collect()
    })
}

/// `w-9 h-9 border-2 border-std-700`.
fn option_image(source: gpui::ImageSource) -> impl IntoElement {
    image(source)
        .size(px(OPTION_SIZE))
        .border_2()
        .border_color(rgb(theme::STD_700))
}

impl Calculator {
    pub(super) fn open_picker(
        &mut self,
        kind: PickerKind,
        target: Target,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let search = cx.new(|cx| TextInput::new("Click Here to Search", cx));
        let changes = cx.subscribe(&search, |this, _, event, cx| {
            if let (TextInputEvent::Changed(text), Some(picker)) = (event, &mut this.picker) {
                picker.query = text.clone();
                cx.notify();
            }
        });

        // Typing searches right away, as the web's input takes focus on click.
        window.focus(&search.read(cx).focus_handle(cx));

        self.picker = Some(Picker {
            kind,
            target,
            search,
            query: SharedString::default(),
            filters: Vec::new(),
            _search_changes: changes,
        });

        cx.notify();
    }

    fn close_picker(&mut self, cx: &mut Context<Self>) {
        self.picker = None;
        cx.notify();
    }

    fn insert_item(&mut self, target: Target, item: ItemId, cx: &mut Context<Self>) {
        match target {
            Target::Player => self.player.data.insert_item(item, true),
            Target::Enemy(i) => match self.enemies.get_mut(i) {
                Some(enemy) => enemy.insert_item(item, false),
                None => return,
            },
        }

        self.recalculate(cx);
    }

    fn remove_item(&mut self, target: Target, id: u64, cx: &mut Context<Self>) {
        match target {
            Target::Player => self.player.data.items.remove_by_id(id),
            Target::Enemy(i) => match self.enemies.get_mut(i) {
                Some(enemy) => enemy.items.remove_by_id(id),
                None => return,
            },
        }

        self.recalculate(cx);
    }

    /// The web replaces the tray without resetting exceptions, so recommended items with
    /// stacks got no stack input. Inserting them one by one gives them one.
    fn replace_items(&mut self, target: Target, items: &[ItemId], cx: &mut Context<Self>) {
        match target {
            Target::Player => self.player.data.replace_items(items, true),
            Target::Enemy(i) => match self.enemies.get_mut(i) {
                Some(enemy) => enemy.replace_items(items, false),
                None => return,
            },
        }

        self.recalculate(cx);
    }

    fn insert_rune(&mut self, rune: RuneId, cx: &mut Context<Self>) {
        self.player.runes.push(rune);
        self.recalculate(cx);
    }

    fn remove_rune(&mut self, id: u64, cx: &mut Context<Self>) {
        self.player.runes.remove_by_id(id);
        self.recalculate(cx);
    }

    fn replace_runes(&mut self, runes: &[RuneId], cx: &mut Context<Self>) {
        self.player.runes = runes.iter().copied().collect();
        self.recalculate(cx);
    }

    /// The open picker, drawn over the whole window.
    pub(super) fn picker(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let picker = self.picker.as_ref()?;

        let options = match picker.kind {
            PickerKind::Items => self.item_options(picker, cx),
            PickerKind::Runes => self.rune_options(picker, cx),
        };

        let search_focused = picker.search.read(cx).is_focused(window);

        // `Search`: `flex items-center gap-2 border border-transparent
        // focus-within:border-std-800 px-4`.
        let search = div()
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .border_1()
            .border_color(match search_focused {
                true => rgb(theme::STD_800),
                false => rgba(0),
            })
            .child(
                svg()
                    .path("svgs/search.svg")
                    .size_4()
                    .flex_none()
                    .text_color(rgb(match search_focused {
                        true => theme::TEXT,
                        false => theme::STD_300,
                    })),
            )
            .child(div().flex_1().p_3().text_sm().child(picker.search.clone()));

        // `flex flex-wrap gap-4 content-start min-h-0 h-full overflow-y-auto`
        let options = div()
            .id("picker-options")
            .flex()
            .flex_wrap()
            .content_start()
            .gap_4()
            .flex_1()
            .min_w_0()
            .h_full()
            .overflow_y_scroll()
            .children(options);

        let body = div()
            .flex()
            .gap_4()
            .flex_1()
            .min_h_0()
            .when(picker.kind == PickerKind::Items, |body| {
                body.child(self.item_filters(picker, cx))
            })
            .child(options);

        let tray = div()
            .id("picker-tray")
            .flex()
            .flex_col()
            .flex_none()
            .gap_4()
            .overflow_y_scroll()
            .children(self.tray_rows(picker, cx));

        let viewport = window.viewport_size();

        // `grid grid-cols-[1fr_auto] bg-std-950 w-full max-w-7xl h-[80vh] overflow-hidden`
        let modal = div()
            .id("picker")
            .flex()
            .w_full()
            .max_w(rems(80.))
            .h(viewport.height * 0.8)
            .overflow_hidden()
            .bg(rgb(theme::STD_950))
            // Clicks inside do not reach the backdrop, which closes the picker.
            .occlude()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .flex_1()
                    .min_w_0()
                    .p_4()
                    .child(search)
                    .child(body),
            )
            .child(tray);

        // `fixed inset-0 z-50 flex items-center justify-center bg-black/70`
        let backdrop = div()
            .id("picker-backdrop")
            .flex()
            .items_center()
            .justify_center()
            .w(viewport.width)
            .h(viewport.height)
            .bg(rgba(0x000000b3))
            .occlude()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.close_picker(cx)),
            )
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                if event.keystroke.key == "escape" {
                    this.close_picker(cx);
                }
            }))
            .child(modal);

        Some(
            deferred(anchored().position(point(px(0.), px(0.))).child(backdrop))
                .with_priority(MODAL_PRIORITY)
                .into_any_element(),
        )
    }

    fn item_options(&self, picker: &Picker, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let target = picker.target;

        purchasable_items()
            .iter()
            .copied()
            .filter(|item| contains_ascii_case_insensitive(item.name(), &picker.query))
            .filter(|item| {
                picker
                    .filters
                    .iter()
                    .all(|&stat| ItemId::filter(stat).contains(item))
            })
            .map(|item| {
                // `flex flex-col gap-1 w-fit`
                div()
                    .id(("picker-item", item as usize))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .cursor_pointer()
                    .hover_docs(item)
                    .on_click(cx.listener(move |this, _, _, cx| this.insert_item(target, item, cx)))
                    .child(option_image(item.image_source()))
                    // `text-std-500 font-medium text-center`
                    .child(
                        div()
                            .text_center()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(theme::STD_500))
                            .child(item.price().to_string()),
                    )
                    .into_any_element()
            })
            .collect()
    }

    fn rune_options(&self, picker: &Picker, cx: &mut Context<Self>) -> Vec<AnyElement> {
        RuneId::VARIANTS
            .iter()
            .copied()
            .filter(|rune| contains_ascii_case_insensitive(rune.name(), &picker.query))
            .map(|rune| {
                div()
                    .id(("picker-rune", rune as usize))
                    .cursor_pointer()
                    .hover_docs(rune)
                    .on_click(cx.listener(move |this, _, _, cx| this.insert_rune(rune, cx)))
                    .child(option_image(rune.image_source()))
                    .into_any_element()
            })
            .collect()
    }

    /// `ItemFilter`: a clear button and a toggle per stat.
    fn item_filters(&self, picker: &Picker, cx: &mut Context<Self>) -> impl IntoElement {
        let clear = div()
            .id("picker-filter-clear")
            .p_1()
            .text_center()
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(picker) = &mut this.picker {
                    picker.filters.clear();
                    cx.notify();
                }
            }))
            .child("x");

        let buttons = FILTERS.into_iter().map(|stat| {
            let selected = picker.filters.contains(&stat);

            // `border-r-3 py-1.5 px-4`, cyan and bright when selected. The web's
            // `contrast-0` greys unselected icons out; gpui has grayscale and opacity.
            div()
                .id(("picker-filter", stat as usize))
                .py_1p5()
                .px_4()
                .border_r(px(3.))
                .border_color(match selected {
                    true => rgb(theme::CYAN_400),
                    false => rgba(0),
                })
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(picker) = &mut this.picker {
                        match picker.filters.iter().position(|&other| other == stat) {
                            Some(index) => _ = picker.filters.remove(index),
                            None => picker.filters.push(stat),
                        }

                        cx.notify();
                    }
                }))
                .child(
                    asset_image(filter_icon(stat))
                        .size_4()
                        .grayscale(!selected)
                        .when(!selected, |icon| icon.opacity(0.5)),
                )
        });

        div()
            .id("picker-filters")
            .flex()
            .flex_col()
            .flex_none()
            .overflow_y_scroll()
            .child(clear)
            .children(buttons)
    }

    /// Every player that the options can go to: only the player for runes.
    fn tray_rows(&self, picker: &Picker, cx: &mut Context<Self>) -> Vec<AnyElement> {
        match picker.kind {
            PickerKind::Runes => vec![tray_row(
                picker,
                Target::Player,
                self.player.data.champion_id,
                &self.player.runes,
                cx,
                |this, id, cx| this.remove_rune(id, cx),
                |this, runes, cx| this.replace_runes(runes, cx),
            )],
            PickerKind::Items => {
                let mut rows = vec![tray_row(
                    picker,
                    Target::Player,
                    self.player.data.champion_id,
                    &self.player.data.items,
                    cx,
                    |this, id, cx| this.remove_item(Target::Player, id, cx),
                    |this, items, cx| this.replace_items(Target::Player, items, cx),
                )];

                for (i, enemy) in self.enemies.iter().enumerate() {
                    let target = Target::Enemy(i);

                    rows.push(tray_row(
                        picker,
                        target,
                        enemy.champion_id,
                        &enemy.items,
                        cx,
                        move |this, id, cx| this.remove_item(target, id, cx),
                        move |this, items, cx| this.replace_items(target, items, cx),
                    ));
                }

                rows
            }
        }
    }
}

/// `make_row`: the champion, which selects this player, recommendation buttons per position,
/// and the current values.
fn tray_row<T: ValueId + Entry + Copy + 'static>(
    picker: &Picker,
    target: Target,
    champion_id: ChampionId,
    values: &Tray<T>,
    cx: &mut Context<Calculator>,
    remove: impl Fn(&mut Calculator, u64, &mut Context<Calculator>) + Copy + 'static,
    replace: impl Fn(&mut Calculator, &[T], &mut Context<Calculator>) + Copy + 'static,
) -> AnyElement {
    let row_id = match target {
        Target::Player => 0,
        Target::Enemy(i) => i + 1,
    };

    let champion = div()
        .id(("tray-champion", row_id))
        .size_7()
        .cursor_pointer()
        .hover_docs(champion_id)
        .on_click(cx.listener(move |this, _, _, cx| {
            if let Some(picker) = &mut this.picker {
                picker.target = target;
                cx.notify();
            }
        }))
        .child(image(champion_id.image_source()).size_full());

    let recommendations = Position::VARIANTS.iter().map(|&position| {
        div()
            .id(("tray-position", row_id * 8 + position as usize))
            .size_7()
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                replace(this, T::recommendations(champion_id, position), cx)
            }))
            .child(position_image(position).size_full())
    });

    let current = values.iter().map(|(id, value)| {
        div()
            .id(SharedString::from(format!("tray-value-{row_id}-{id}")))
            .cursor_pointer()
            .hover_docs(value)
            .on_click(cx.listener(move |this, _, _, cx| remove(this, id, cx)))
            .child(option_image(value.image_source()))
    });

    let grid = || {
        div().flex().flex_wrap().gap(px(TRAY_GAP)).w(px(
            TRAY_COLUMNS * OPTION_SIZE + (TRAY_COLUMNS - 1.) * TRAY_GAP
        ))
    };

    // `flex flex-col gap-4 p-2`, `bg-std-800` for the player that options go to.
    div()
        .flex()
        .flex_col()
        .gap_4()
        .p_2()
        .when(picker.target == target, |row| row.bg(rgb(theme::STD_800)))
        .child(
            grid()
                .items_center()
                // Same columns as the values below: 28px icons in 36px cells.
                .children(
                    std::iter::once(champion.into_any_element())
                        .chain(recommendations.map(IntoElement::into_any_element))
                        .map(|icon| div().flex().justify_center().w(px(OPTION_SIZE)).child(icon)),
                ),
        )
        .child(grid().children(current))
        .into_any_element()
}

//! The player and enemy panels: `inputs/player.rs`, `inputs/enemies.rs` and their parts
//! (`banner.rs`, `abilities.rs`, `stats.rs`, `dragon.rs`, `exceptions.rs`).
use crate::{
    calculator::{
        Calculator, Field, Target,
        picker::PickerKind,
        state::{ExceptionMap, StatField, Tray, item_exceptions},
        widgets::{checkbox, input_row, selector_button, small_button},
    },
    components::{
        box_shadow,
        hover_docs::HoverDocsExt,
        image::{Entry, asset_image, centered_image, image},
        tables::ability_icon,
    },
    theme,
};
use gpui::{
    AnyElement, Context, Entity, FontWeight, IntoElement, ObjectFit, ParentElement, SharedString,
    Styled, StyledImage, Window, div, prelude::*, rems, rgb,
};
use math::{
    AbilityId, ChampionId, ItemId, RuneId, ValueId,
    bitset::{BitSet, BitSetExc},
    model::AbilityLevels,
};

/// `flex flex-col w-full max-w-80 sm:w-64 box m-2`.
fn panel() -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .w_64()
        .flex_none()
        .m_2()
        .bg(rgb(theme::STD_900))
        .shadow(box_shadow())
}

/// `grid grid-cols-[auto_1fr_1fr] gap-x-2 px-4 py-3`; each row sets its own `gap-y`.
fn input_grid(rows: Vec<AnyElement>) -> Option<gpui::Div> {
    // `empty:hidden`
    (!rows.is_empty()).then(|| div().flex().flex_col().px_4().py_3().children(rows))
}

/// `Banner`: the champion's splash art with its selector over the bottom edge.
fn banner(
    id: &'static str,
    champion_id: ChampionId,
    selector: Entity<crate::components::selector::Selector<ChampionId>>,
) -> impl IntoElement {
    div()
        .id(id)
        .relative()
        .w_full()
        .h(rems(9.))
        .flex_none()
        .bg(rgb(theme::STD_900))
        .shadow(box_shadow())
        .hover_docs(champion_id)
        .child(
            // `.clip img { object-fit: cover; object-position: 15% 15% }`. gpui centers it.
            centered_image(champion_id)
                .size_full()
                .object_fit(ObjectFit::Cover),
        )
        .child(
            div()
                .absolute()
                .left_0()
                .bottom_0()
                .w_full()
                .child(selector),
        )
}

/// The stats of a number input row: `text-center min-w-0 bg-transparent`, white unless
/// disabled (`text-std-400`).
fn number(input: Entity<crate::components::text_input::TextInput>, disabled: bool) -> AnyElement {
    div()
        .w_full()
        .text_color(rgb(if disabled {
            theme::STD_400
        } else {
            theme::TEXT
        }))
        .child(input)
        .into_any_element()
}

impl Calculator {
    /// `Abilities`: Q, W, E and R levels.
    fn abilities(&mut self, window: &mut Window, cx: &mut Context<Self>) -> gpui::Div {
        let champion = self.player.data.champion_id;

        let cells = AbilityLevels::ABILITIES.map(|key| {
            let input = self.field(Field::Ability(key), false, window, cx);

            // `label.grid.grid-cols-2`: the icon and a `text-sm bg-std-800 text-std-200` input.
            div()
                .flex()
                .flex_1()
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .items_center()
                        .justify_center()
                        .child(ability_icon(champion, AbilityId::from(key), 32.)),
                )
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .items_center()
                        .bg(rgb(theme::STD_800))
                        .text_sm()
                        .text_color(rgb(theme::STD_200))
                        .child(input),
                )
        });

        div().flex().children(cells)
    }

    /// `ChampionExceptionSelector`: the stacks of the champion's stacking ability, if any.
    fn champion_stacks(
        &mut self,
        target: Target,
        champion: ChampionId,
        ally: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let key = champion.exceptions(ally)?;
        let input = self.field(Field::ChampionStacks(target), false, window, cx);

        Some(
            input_row(
                ability_icon(champion, AbilityId::from(key), 24.),
                "Stacks",
                number(input, false),
                6.,
            )
            .into_any_element(),
        )
    }

    /// `ExceptionSelector`: stacks of each item or rune that has them. Duplicates of items in
    /// `unique` are listed once.
    fn value_stacks<T: ValueId + Entry + std::hash::Hash + Eq>(
        &mut self,
        values: &Tray<T>,
        exceptions: &ExceptionMap<T>,
        unique: BitSetExc,
        field: impl Fn(T) -> Field,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let mut seen = BitSet::EMPTY;
        let mut rows = Vec::new();

        for &value in values.values() {
            let index = value.index() as u32;

            if exceptions.get(&value).is_none()
                || (unique.contains_const(index) && seen.contains_const(index))
            {
                continue;
            }

            seen.insert_const(index);

            let input = self.field(field(value), false, window, cx);
            let icon = div()
                .id(SharedString::from(format!("stacks-{:?}", value.entity())))
                .hover_docs(value)
                .child(image(value.image_source()).size_6());

            rows.push(input_row(icon, value.name(), number(input, false), 6.).into_any_element());
        }

        rows
    }

    /// `DragonInput`. Dragon icons exist only as AVIF, so the slot stays empty.
    fn dragon(
        &mut self,
        field: Field,
        title: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let input = self.field(field, false, window, cx);

        input_row(div().size_6(), title, number(input, false), 6.).into_any_element()
    }

    /// `Stats` with the level first: `StatCell` rows.
    fn stat_rows(
        &mut self,
        target: Target,
        fields: &[StatField],
        infer: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let level = self.field(Field::Level(target), false, window, cx);
        let mut rows = vec![
            input_row(
                asset_image("stats/level.svg").size(rems(0.875)),
                "Level",
                number(level, false),
                2.,
            )
            .into_any_element(),
        ];

        for &stat in fields {
            // The player's current health stays editable while the stats are inferred.
            let disabled = infer && !(target == Target::Player && stat == StatField::CurrentHealth);
            let input = self.field(Field::Stat(target, stat), disabled, window, cx);

            rows.push(
                input_row(
                    asset_image(stat.icon()).size(rems(0.875)),
                    stat.name(),
                    number(input, disabled),
                    2.,
                )
                .into_any_element(),
            );
        }

        div().flex().flex_col().px_4().py_3().children(rows)
    }

    /// `PlayerInput`.
    pub(super) fn player_panel(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let data = &self.player.data;
        let champion = data.champion_id;
        let infer = data.infer_stats;
        let is_mega_gnar = data.is_mega_gnar;
        let items = data.items.clone();
        let item_exceptions_map = data.item_exceptions.clone();
        let runes = self.player.runes.clone();
        let rune_exceptions = self.player.rune_exceptions.clone();
        let item_count = items.len();
        let rune_count = runes.len();

        let abilities = self.abilities(window, cx);

        let mut rows = Vec::new();

        if infer {
            rows.push(self.dragon(Field::AllyFire, "Fire dragons", window, cx));
            rows.push(self.dragon(Field::AllyEarth, "Earth dragons", window, cx));
        }

        rows.extend(self.champion_stacks(Target::Player, champion, true, window, cx));
        rows.extend(self.value_stacks(
            &items,
            &item_exceptions_map,
            item_exceptions(true),
            |item: ItemId| Field::ItemStacks(Target::Player, item),
            window,
            cx,
        ));
        rows.extend(self.value_stacks(
            &runes,
            &rune_exceptions,
            RuneId::exceptions(),
            Field::RuneStacks,
            window,
            cx,
        ));

        let stats = self.stat_rows(Target::Player, &StatField::PLAYER, infer, window, cx);

        panel()
            .child(banner(
                "player-banner",
                champion,
                self.player_selector.clone(),
            ))
            .child(abilities)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .my_2()
                    .w_full()
                    .child(selector_button(
                        "player-items",
                        "Items",
                        item_count,
                        cx.listener(|this, _, window, cx| {
                            this.open_picker(PickerKind::Items, Target::Player, window, cx)
                        }),
                    ))
                    .child(selector_button(
                        "player-runes",
                        "Runes",
                        rune_count,
                        cx.listener(|this, _, window, cx| {
                            this.open_picker(PickerKind::Runes, Target::Player, window, cx)
                        }),
                    )),
            )
            .children(input_grid(rows))
            .when(champion == ChampionId::Gnar, |panel| {
                panel.child(checkbox(
                    "player-mega-gnar",
                    "Mega Gnar",
                    is_mega_gnar,
                    cx.listener(|this, checked: &bool, _, cx| {
                        this.player.data.is_mega_gnar = *checked;
                        this.recalculate(cx);
                    }),
                ))
            })
            .child(checkbox(
                "player-infer-stats",
                "Infer Stats",
                infer,
                cx.listener(|this, checked: &bool, _, cx| {
                    this.player.data.infer_stats = *checked;
                    this.recalculate(cx);
                }),
            ))
            .child(stats)
    }

    /// `EnemiesInput`: the selected enemy.
    pub(super) fn enemy_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) -> gpui::Div {
        let index = self.enemy_index.min(self.enemies.len() - 1);
        let target = Target::Enemy(index);
        let count = self.enemies.len();
        let enemy = &self.enemies[index];
        let champion = enemy.champion_id;
        let infer = enemy.infer_stats;
        let is_mega_gnar = enemy.is_mega_gnar;
        let items = enemy.items.clone();
        let exceptions = enemy.item_exceptions.clone();

        let mut rows = Vec::new();

        if infer {
            rows.push(self.dragon(Field::EnemyEarth, "Earth dragons", window, cx));
        }

        rows.extend(self.champion_stacks(target, champion, false, window, cx));
        rows.extend(self.value_stacks(
            &items,
            &exceptions,
            item_exceptions(false),
            move |item: ItemId| Field::ItemStacks(target, item),
            window,
            cx,
        ));

        let stats = self.stat_rows(target, &StatField::ENEMY, infer, window, cx);

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .gap_2()
            .px_4()
            .py_3()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(theme::STD_200))
                    .whitespace_nowrap()
                    .child(format!("Enemy {}", index + 1)),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(small_button(
                        "enemy-add",
                        "Add",
                        count < super::state::MAX_ENEMIES,
                        cx.listener(|this, _, _, cx| this.add_enemy(cx)),
                    ))
                    .child(small_button(
                        "enemy-remove",
                        "Remove",
                        count > 1,
                        cx.listener(|this, _, _, cx| this.remove_enemy(cx)),
                    )),
            );

        panel()
            .child(header)
            .child(banner(
                "enemy-banner",
                champion,
                self.enemy_selector.clone(),
            ))
            .child(div().w_full().my_2().child(selector_button(
                "enemy-items",
                "Items",
                items.len(),
                cx.listener(move |this, _, window, cx| {
                    this.open_picker(PickerKind::Items, Target::Enemy(index), window, cx)
                }),
            )))
            .children(input_grid(rows))
            .when(champion == ChampionId::Gnar, |panel| {
                panel.child(checkbox(
                    "enemy-mega-gnar",
                    "Mega Gnar",
                    is_mega_gnar,
                    cx.listener(move |this, checked: &bool, _, cx| {
                        if let Some(enemy) = this.enemies.get_mut(index) {
                            enemy.is_mega_gnar = *checked;
                            this.recalculate(cx);
                        }
                    }),
                ))
            })
            .child(checkbox(
                "enemy-infer-stats",
                "Infer Stats",
                infer,
                cx.listener(move |this, checked: &bool, _, cx| {
                    if let Some(enemy) = this.enemies.get_mut(index) {
                        enemy.infer_stats = *checked;
                        this.recalculate(cx);
                    }
                }),
            ))
            .child(stats)
    }
}

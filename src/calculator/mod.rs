//! The Calculator page: the web app's `calculator/page.rs` and `calculator/components/inputs`.
//!
//! The web keeps inputs in reducers and recalculates in an effect that watches them, with a
//! `LastAction` flag so that inferred stats written back by a reply do not trigger another
//! request. Here every input handler updates the state and calls [`Calculator::recalculate`]
//! explicitly, so replies can write the inferred stats directly.
mod backend;
mod docs;
mod panels;
mod picker;
mod results;
mod state;
mod tables;
mod widgets;

use crate::{
    components::{
        box_shadow,
        selector::{Selected, Selector},
        text_input::{TextInput, TextInputEvent},
    },
    theme,
};
use backend::Snapshot;
use gpui::{
    AppContext, Context, Entity, FontWeight, IntoElement, ParentElement, Render, SharedString,
    Styled, Subscription, Task, Window, div, px, rgb,
};
use results::{DamageTables, SelectEnemy};
use state::{Enemy, Player, StatField, Stats, default_enemies};
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
    rc::Rc,
};
use tutorlolv2::{
    ChampionId, GameMap, ItemId, Key, RuneId,
    model::{Dragons, OutputGame},
};

/// Tailwind's `xl` breakpoint: the tables move between the two panels.
const XL_BREAKPOINT: f32 = 1280.0;

/// Which player an input belongs to.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Target {
    Player,
    Enemy(usize),
}

/// Every number input, so its text input survives re-renders.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Field {
    Ability(Key),
    Level(Target),
    Stat(Target, StatField),
    AllyFire,
    AllyEarth,
    EnemyEarth,
    ChampionStacks(Target),
    ItemStacks(Target, ItemId),
    RuneStacks(RuneId),
}

struct NumberField {
    input: Entity<TextInput>,
    _changes: Subscription,
}

pub struct Calculator {
    player: Player,
    enemies: Vec<Enemy>,
    dragons: Dragons,
    /// The enemy shown in the enemy panel.
    enemy_index: usize,
    game_map: GameMap,

    /// The last request's error. Kept until the next reply, like the tables, so the page does
    /// not change while a calculation runs.
    error: Option<SharedString>,
    /// Replaced by each new calculation, which cancels the previous one.
    _calculation: Option<Task<()>>,

    player_selector: Entity<Selector<ChampionId>>,
    enemy_selector: Entity<Selector<ChampionId>>,
    tables: Entity<DamageTables>,
    /// The open item or rune picker.
    picker: Option<picker::Picker>,
    fields: HashMap<Field, NumberField>,
    /// Fields rendered this frame. Others are dropped after rendering.
    rendered_fields: HashSet<Field>,
    _subscriptions: [Subscription; 3],
}

impl Calculator {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let player = Player::random();
        let enemies = default_enemies();

        let player_selector = cx.new(|cx| Selector::banner(player.data.champion_id, cx));
        let enemy_selector = cx.new(|cx| Selector::banner(enemies[0].champion_id, cx));
        let tables = cx.new(|_| DamageTables::default());

        let subscriptions = [
            cx.subscribe(&tables, |this, _, SelectEnemy(index), cx| {
                this.select_enemy(*index, cx)
            }),
            cx.subscribe(&player_selector, |this, _, Selected(id), cx| {
                this.player.data.champion_id = *id;
                this.recalculate(cx);
            }),
            cx.subscribe(&enemy_selector, |this, _, Selected(id), cx| {
                if let Some(enemy) = this.enemies.get_mut(this.enemy_index) {
                    enemy.champion_id = *id;
                    this.recalculate(cx);
                }
            }),
        ];

        let mut calculator = Self {
            player,
            enemies,
            dragons: Dragons::default(),
            enemy_index: 0,
            game_map: GameMap::SummonersRift,
            error: None,
            _calculation: None,
            player_selector,
            enemy_selector,
            tables,
            picker: None,
            fields: HashMap::new(),
            rendered_fields: HashSet::new(),
            _subscriptions: subscriptions,
        };

        calculator.recalculate(cx);
        calculator
    }

    /// Calculates the current inputs in the background; a newer request cancels this one.
    fn recalculate(&mut self, cx: &mut Context<Self>) {
        let snapshot = Snapshot::new(&self.player, &self.enemies, self.dragons, self.game_map);

        // The previous result stays on screen until this one arrives.
        self._calculation = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_spawn(async move { snapshot.calculate() })
                .await;

            this.update(cx, |this, cx| this.finish(result, cx)).ok();
        }));

        // The panels show the new input now; the tables view is not redrawn until the reply.
        cx.notify();
    }

    fn finish(&mut self, result: Result<OutputGame, backend::Error>, cx: &mut Context<Self>) {
        let game = match result {
            Ok(game) => {
                // Every reply is a full snapshot: write back all inferred stats.
                if self.player.data.infer_stats {
                    self.player.data.stats = game.current_player.current_stats;
                }

                for (enemy, output) in self.enemies.iter_mut().zip(&game.enemies) {
                    if enemy.infer_stats {
                        enemy.stats = output.current_stats;
                    }
                }

                self.error = None;
                Some(Rc::new(game))
            }
            Err(error) => {
                self.error = Some(error.to_string().into());
                None
            }
        };

        self.tables
            .update(cx, |tables, cx| tables.set_game(game, cx));

        cx.notify();
    }

    fn enemy(&self) -> &Enemy {
        &self.enemies[self.enemy_index.min(self.enemies.len() - 1)]
    }

    fn target_data<R>(
        &mut self,
        target: Target,
        player: impl FnOnce(&mut state::PlayerData<tutorlolv2::model::PlayerStats>) -> R,
        enemy: impl FnOnce(&mut Enemy) -> R,
    ) -> Option<R> {
        match target {
            Target::Player => Some(player(&mut self.player.data)),
            Target::Enemy(i) => self.enemies.get_mut(i).map(enemy),
        }
    }

    /// The text a field shows for the current state.
    fn field_text(&self, field: Field) -> String {
        let data = |target: Target| match target {
            Target::Player => (
                self.player.data.level,
                self.player.data.stacks,
                &self.player.data.item_exceptions,
            ),
            Target::Enemy(i) => {
                let enemy = &self.enemies[i];
                (enemy.level, enemy.stacks, &enemy.item_exceptions)
            }
        };

        match field {
            Field::Ability(key) => self.player.abilities[key].to_string(),
            Field::Level(target) => data(target).0.to_string(),
            Field::Stat(Target::Player, stat) => stat.format(self.player.data.stats.get(stat)),
            Field::Stat(Target::Enemy(i), stat) => stat.format(self.enemies[i].stats.get(stat)),
            Field::AllyFire => self.dragons.ally_fire_dragons.to_string(),
            Field::AllyEarth => self.dragons.ally_earth_dragons.to_string(),
            Field::EnemyEarth => self.dragons.enemy_earth_dragons.to_string(),
            Field::ChampionStacks(target) => data(target).1.to_string(),
            Field::ItemStacks(target, item) => data(target)
                .2
                .get(&item)
                .map_or(0, |exception| exception.stacks())
                .to_string(),
            Field::RuneStacks(rune) => self
                .player
                .rune_exceptions
                .get(&rune)
                .map_or(0, |exception| exception.stacks())
                .to_string(),
        }
    }

    /// Applies typed text. Invalid numbers count as zero (or level 1), like the web inputs.
    fn field_changed(&mut self, field: Field, text: &str, cx: &mut Context<Self>) {
        let text = text.trim();
        let int = || text.parse::<u32>().unwrap_or(0);

        match field {
            Field::Ability(key) => {
                let value = text.parse::<u8>().unwrap_or(0);
                let abilities = &mut self.player.abilities;

                match key {
                    Key::Q => abilities.q = value,
                    Key::W => abilities.w = value,
                    Key::E => abilities.e = value,
                    Key::R => abilities.r = value,
                    Key::P => {}
                }
            }
            Field::Level(target) => {
                let level = text.parse::<NonZeroU8>().unwrap_or(NonZeroU8::MIN);
                self.target_data(target, |data| data.level = level, |data| data.level = level);
            }
            Field::Stat(target, stat) => {
                // The web parses these as integers, which turns `1.25` attack speed into 0.
                let value = text.parse::<f32>().unwrap_or(0.0);

                if target == Target::Player && stat == StatField::CurrentHealth {
                    // Inferred replies overwrite the stats, so the typed health is kept apart.
                    self.player.current_health = value;
                }

                self.target_data(
                    target,
                    |data| data.stats.set(stat, value),
                    |data| data.stats.set(stat, value),
                );
            }
            Field::AllyFire => self.dragons.ally_fire_dragons = int() as u16,
            Field::AllyEarth => self.dragons.ally_earth_dragons = int() as u16,
            Field::EnemyEarth => self.dragons.enemy_earth_dragons = int() as u16,
            Field::ChampionStacks(target) => {
                let stacks = int();
                self.target_data(
                    target,
                    |data| data.stacks = stacks,
                    |data| data.stacks = stacks,
                );
            }
            Field::ItemStacks(target, item) => {
                let stacks = int();
                self.target_data(
                    target,
                    |data| data.item_exceptions.insert(item, stacks),
                    |data| data.item_exceptions.insert(item, stacks),
                );
            }
            Field::RuneStacks(rune) => self.player.rune_exceptions.insert(rune, int()),
        }

        self.recalculate(cx);
    }

    /// The text input of a field, created on first use and kept in sync with the state while
    /// it is not being edited.
    fn field(
        &mut self,
        field: Field,
        disabled: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<TextInput> {
        self.rendered_fields.insert(field);

        let text = self.field_text(field);
        let number = self.fields.entry(field).or_insert_with(|| {
            let input = cx.new(|cx| {
                TextInput::new("0", cx)
                    .centered()
                    .filter(|c| c.is_ascii_digit() || c == '.' || c == '-')
            });

            let changes = cx.subscribe(&input, move |this, _, event, cx| {
                if let TextInputEvent::Changed(text) = event {
                    this.field_changed(field, text, cx);
                }
            });

            NumberField {
                input,
                _changes: changes,
            }
        });

        let input = number.input.clone();

        input.update(cx, |input, cx| {
            input.set_disabled(disabled, window, cx);

            if !input.is_focused(window) {
                input.set_text(text, cx);
            }
        });

        input
    }

    fn select_enemy(&mut self, index: usize, cx: &mut Context<Self>) {
        self.enemy_index = index;
        cx.notify();
    }

    fn add_enemy(&mut self, cx: &mut Context<Self>) {
        if self.enemies.len() < state::MAX_ENEMIES {
            self.enemies.push(Enemy::random(false));
            self.enemy_index = self.enemies.len() - 1;
            self.recalculate(cx);
        }
    }

    fn remove_enemy(&mut self, cx: &mut Context<Self>) {
        let count = self.enemies.len();

        if count > 1 {
            let selected = self.enemy_index.min(count - 1);

            self.enemies.swap_remove(selected);
            self.enemy_index = selected.min(count - 2);
            self.recalculate(cx);
        }
    }
}

/// `errorlog`: the request error, with hints.
fn error_box(error: SharedString) -> impl IntoElement {
    let hint = |text: &'static str| {
        div()
            .flex()
            .gap_2()
            .child("•")
            .child(div().flex_1().min_w_0().child(text))
    };

    div()
        .flex()
        .gap_6()
        .px_6()
        .py_4()
        .bg(rgb(theme::STD_900))
        .shadow(box_shadow())
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .gap_4()
                .child(
                    div()
                        .text_2xl()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme::STD_200))
                        .child("Request error"),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .ml_4()
                        .text_color(rgb(theme::STD_400))
                        .child(hint("Servers might be down due to an internal error"))
                        .child(hint("This application might be outdated"))
                        .child(hint("Refresh the page or come back later")),
                ),
        )
        .child(
            // `whitespace-pre-wrap`: flex items keep their content width unless allowed to
            // shrink, which would push long errors past the panel.
            div()
                .flex_1()
                .min_w_0()
                .overflow_hidden()
                .p_2()
                .border_1()
                .border_color(rgb(theme::STD_800))
                .font_family(theme::CODE_FONT)
                .child(error),
        )
}

impl Render for Calculator {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.rendered_fields.clear();

        let enemy_champion = self.enemy().champion_id;
        self.enemy_selector
            .update(cx, |selector, cx| selector.set_value(enemy_champion, cx));

        let player = self.player_panel(window, cx);
        let enemies = self.enemy_panel(window, cx);

        // `flex flex-col gap-4 p-2 overflow-hidden flex-1 min-w-full xl:min-w-0`
        let middle = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap_4()
            .p_2()
            .children(self.error.clone().map(error_box))
            .child(DamageTables::view(&self.tables, cx));

        let rendered = std::mem::take(&mut self.rendered_fields);
        self.fields.retain(|field, _| rendered.contains(field));
        self.rendered_fields = rendered;

        // `flex flex-wrap w-full px-2 mt-2 mb-96`: on `xl` the tables sit between the panels;
        // below it both panels come first and the tables take a full row.
        let wide = window.viewport_size().width >= px(XL_BREAKPOINT);
        let page = div().flex().flex_wrap().w_full().px_2().mt_2().pb_96();

        let page = match wide {
            true => page.child(player).child(middle).child(enemies),
            false => page
                .child(player)
                .child(enemies)
                .child(div().w_full().flex().child(middle)),
        };

        page.children(self.picker(window, cx))
    }
}

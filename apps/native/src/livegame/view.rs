//! The live game layout: the web app's `LivegameView` and `livegame/components`.
//!
//! Shared by the Livegame page (the game being played) and the replay viewer (a recorded frame).
//! The tables are [`CachedTable`] views, rebuilt once per result, so the frames drawn between
//! results (hovering, tooltips, typing in the item selector) reuse their layout and paint.
use crate::{
    components::{
        box_shadow,
        cached_table::CachedTable,
        docs::{self, Docs},
        hover_docs::HoverDocsExt,
        image::{Entry, ability_image, centered_image, image, position_image},
        selector::{Selected, Selector},
        stack::{StackGame, StackPanel},
        tables::{Cell, Column, damage_columns, damage_columns_with_diff, empty_table},
    },
    livegame::game::{Game, Score},
    theme,
};
use gpui::{
    AnyElement, AppContext, Context, Entity, FontWeight, IntoElement, ObjectFit, ParentElement,
    Render, SharedString, Styled, StyledImage, Subscription, Window, div, prelude::*, px, rems,
    rgb,
};
use math::{Ctx, CtxVar, ItemId, Position, TypeMetadata, bitset::BitSet, model::AbilityLevels};
use std::rc::Rc;
use strum::{IntoEnumIterator, VariantArray};

/// Tailwind's `xl`: the player column sits left of the tables.
const XL_BREAKPOINT: f32 = 1280.0;

pub struct LivegameView {
    game: Option<Rc<Game>>,
    error: Option<SharedString>,
    damages: Entity<CachedTable>,
    recommendations: Entity<CachedTable>,
    simulated: Entity<CachedTable>,
    stack: Entity<StackPanel>,
    simulated_selector: Entity<Selector<ItemId>>,
    simulated_item: ItemId,
    _selection: Subscription,
}

/// `simulated_items_meta`: the selected purchase gets a column only when some enemy takes
/// damage from it. Every row shares this decision.
fn simulated_items_meta(game: &Game, index: usize) -> Vec<TypeMetadata<ItemId>> {
    let owned = &game.items_meta;
    let offset = owned.len() * 2;
    let visible = (0..game.enemies.len()).any(|e| {
        let damages = game.siml_damages(e, index);
        let at = |i: usize| damages.items.get(i).copied().unwrap_or(0);

        at(offset) != 0 || at(offset + 1) != 0
    });

    let mut meta = owned.clone();

    if visible {
        meta.push(ItemId::SIML[index].data().metadata);
    }

    meta
}

/// The first column of each live table: the enemy portraits, with their context values.
fn portraits(game: &Game) -> Column {
    let cells = game
        .enemies
        .iter()
        .enumerate()
        .map(|(e, enemy)| {
            let champion = enemy.champion_id;
            let ctx = game.enemy_damages(e).ctx;

            Cell::new(image(champion.image_source()).size(px(32.)))
                .docs(move || docs::cell_blocks(&CtxVar::ENEMY_VARS, &ctx, Docs::global(champion)))
        })
        .collect();

    Column {
        header: Cell::empty(),
        cells,
    }
}

impl LivegameView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let simulated_item = ItemId::SIML[0];
        let simulated_selector = cx.new(|cx| Selector::compact(simulated_item, &ItemId::SIML, cx));

        let selection = cx.subscribe(&simulated_selector, |this, _, Selected(item), cx| {
            this.simulated_item = *item;
            this.rebuild_simulated(cx);
        });

        Self {
            game: None,
            error: None,
            damages: cx.new(|_| CachedTable::new("live-damages")),
            recommendations: cx.new(|_| CachedTable::new("live-recommendations")),
            simulated: cx.new(|_| CachedTable::new("live-simulated")),
            stack: cx.new(|_| StackPanel::new(false)),
            simulated_selector,
            simulated_item,
            _selection: selection,
        }
    }

    /// Shows a new result, or the placeholder with `error`.
    pub fn set_game(
        &mut self,
        game: Option<Rc<Game>>,
        error: Option<SharedString>,
        cx: &mut Context<Self>,
    ) {
        self.game = game.clone();
        self.error = error;

        if let Some(game) = game {
            let rows = game.enemies.len();

            let damages = {
                let game = game.clone();

                Rc::new(move || {
                    let rows = (0..game.enemies.len())
                        .map(|e| game.enemy_damages(e))
                        .collect::<Vec<_>>();

                    let mut columns = damage_columns(
                        &rows,
                        game.current_player.champion_id,
                        game.current_player.adaptive_type,
                        &game.items_meta,
                        &game.runes_meta,
                    );

                    columns.insert(0, portraits(&game));
                    columns
                })
            };

            self.damages
                .update(cx, |table, cx| table.set(rows, damages, cx));

            let recommendations = {
                let game = game.clone();
                Rc::new(move || recommendation_columns(&game))
            };

            self.recommendations
                .update(cx, |table, cx| table.set(rows, recommendations, cx));

            self.stack.update(cx, |stack, cx| {
                stack.set_game(Some(game.clone() as Rc<dyn StackGame>), cx)
            });

            self.rebuild_simulated(cx);
        }

        cx.notify();
    }

    fn rebuild_simulated(&mut self, cx: &mut Context<Self>) {
        let Some(game) = self.game.clone() else {
            return;
        };

        let index = ItemId::SIML
            .iter()
            .position(|&item| item == self.simulated_item)
            .unwrap_or(0);

        let rows = game.enemies.len();
        let build = Rc::new(move || {
            let meta = simulated_items_meta(&game, index);
            let rows = (0..game.enemies.len())
                .map(|e| game.siml_damages(e, index))
                .collect::<Vec<_>>();
            let bases = (0..game.enemies.len())
                .map(|e| game.enemy_damages(e))
                .collect::<Vec<_>>();

            let mut columns = damage_columns_with_diff(
                &rows,
                Some(&bases),
                game.current_player.champion_id,
                game.current_player.adaptive_type,
                &meta,
                &game.runes_meta,
            );

            columns.insert(0, portraits(&game));
            columns
        });

        self.simulated
            .update(cx, |table, cx| table.set(rows, build, cx));
    }
}

/// "Bonus damage after buying new item": each simulated item's extra damage per enemy.
fn recommendation_columns(game: &Game) -> Vec<Column> {
    let scores = (0..game.enemies.len())
        .map(|e| (game.total_damage(e), game.item_scores(e)))
        .collect::<Vec<_>>();

    // Items in the order they first appear in any enemy's ranking.
    let mut seen = BitSet::EMPTY;
    let items = scores
        .iter()
        .flat_map(|(_, list)| list.iter())
        .filter_map(|&(_, item)| seen.insert_const(item.index() as _).then_some(item))
        .collect::<Vec<_>>();

    let mut columns = vec![portraits(game)];

    columns.extend(items.into_iter().map(|item| {
        Column {
            header: Cell::new(image(item.image_source()).size(px(32.)))
                .docs(move || Docs::global(item).blocks()),
            cells: scores
                .iter()
                .map(|(base, list)| {
                    let damage = list
                        .iter()
                        .find(|(_, other)| *other == item)
                        .map(|(damage, _)| (damage - base).to_string());

                    Cell::new(div().text_sm().children(damage))
                })
                .collect(),
        }
    }));

    columns
}

/// `box`: the panel background with its shadow.
fn panel() -> gpui::Div {
    div().bg(rgb(theme::STD_900)).shadow(box_shadow())
}

/// `Banner`: the splash art, then the player and game time. Hovering shows the player's
/// non-zero context values above the champion's definition.
fn banner(game: &Game, wide: bool) -> AnyElement {
    let player = &game.current_player;
    let champion = player.champion_id;
    let stats = player.current_stats;
    let base = player.base_stats;

    let ctx = Ctx {
        base_armor: base.armor,
        base_ad: base.attack_damage,
        base_attack_speed: base.attack_speed,
        base_magic_resist: base.magic_resist,
        base_health: base.max_health,
        base_mana: base.max_mana,
        ability_power: stats.ability_power,
        armor: stats.armor,
        armor_penetration_flat: stats.armor_penetration_flat,
        armor_penetration_percent: stats.armor_penetration_percent,
        attack_damage: stats.attack_damage,
        attack_speed: stats.attack_speed,
        crit_chance: stats.crit_chance,
        crit_damage: stats.crit_damage,
        current_health: stats.current_health,
        magic_penetration_flat: stats.magic_penetration_flat,
        magic_penetration_percent: stats.magic_penetration_percent,
        magic_resist: stats.magic_resist,
        max_health: stats.max_health,
        max_mana: stats.max_mana,
        current_mana: stats.current_mana,
        level: player.level.get() as _,
        ..Default::default()
    };

    let minutes = game.game_time / 60;
    let seconds = game.game_time % 60;

    panel()
        .id("live-banner")
        .flex()
        .flex_col()
        .docs(move || {
            let idents = CtxVar::iter()
                .filter(|var| docs::display_value(*var, ctx.get(*var)) != 0.0)
                .collect::<Vec<_>>();

            docs::cell_blocks(&idents, &ctx, Docs::global(champion))
        })
        .child(
            // `img-clipped h-24 sm:h-40 md:h-48 lg:h-64 xl:h-32`. A covering image paints
            // beyond its bounds, so its box clips it.
            div()
                .w_full()
                .h(match wide {
                    true => rems(8.),
                    false => rems(12.),
                })
                .overflow_hidden()
                .child(
                    centered_image(champion)
                        .size_full()
                        .object_fit(ObjectFit::Cover),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_4()
                .p_4()
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(theme::STD_300))
                .child(
                    div()
                        .truncate()
                        .child(format!("{} - {}", player.riot_id, champion.name())),
                )
                .child(
                    div()
                        .flex_none()
                        .child(format!("{minutes:02}m {seconds:02}s")),
                ),
        )
        .into_any_element()
}

/// One scoreboard row: champion, name, position and CS, then K / D / A.
fn score_row(score: &Score) -> AnyElement {
    let name = score
        .riot_id
        .split_once('#')
        .map_or(score.riot_id.as_ref(), |(name, _)| name)
        .to_owned();

    let muted = |text: String| div().text_color(rgb(theme::STD_400)).child(text);

    div()
        .flex()
        .items_center()
        .gap_2()
        .w_full()
        .child(
            div()
                .id(SharedString::from(format!("score-{}", score.riot_id)))
                .flex_none()
                .hover_docs(score.champion_id)
                .child(image(score.champion_id.image_source()).size_8()),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .text_xs()
                .child(div().truncate().text_color(rgb(theme::STD_100)).child(name))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_1p5()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .min_w_0()
                                .child(position_image(score.position).size(px(14.)).flex_none())
                                .child(muted(score.champion_id.name().to_owned()).truncate()),
                        )
                        .child(muted(format!("(CS: {})", score.creep_score)).flex_none()),
                ),
        )
        .child(
            div()
                .flex()
                .flex_none()
                .w_24()
                .text_sm()
                .text_color(rgb(theme::STD_200))
                .child(score.kills.to_string())
                .child(muted(" / ".into()))
                .child(score.deaths.to_string())
                .child(muted(" / ".into()))
                .child(score.assists.to_string()),
        )
        .into_any_element()
}

/// `ScoreboardDisplay`: allies and enemies, each in position order.
fn scoreboard(game: &Game) -> [AnyElement; 2] {
    let team = game.current_player.team;
    let side = |id: &'static str, title: &'static str, allies: bool| {
        let cells = Position::VARIANTS
            .iter()
            .flat_map(|&position| {
                game.scoreboard.iter().filter(move |score| {
                    score.position == position && (score.team == team) == allies
                })
            })
            .map(|score| Cell::new(score_row(score)))
            .collect();

        panel()
            .child(crate::components::tables::table(
                id,
                vec![Column {
                    header: Cell::new(div().child(title)),
                    cells,
                }],
            ))
            .into_any_element()
    };

    [
        side("scoreboard-allies", "Allies", true),
        side("scoreboard-enemies", "Enemies", false),
    ]
}

/// `DragonDisplay`. Dragon icons are AVIF only, so each row is labeled with its name.
fn dragons(game: &Game) -> AnyElement {
    const UNKNOWN: &str = "\u{2013}";

    let dragons = game.dragons;
    let rows = [
        (
            "Earth",
            dragons.ally_earth_dragons.to_string(),
            dragons.enemy_earth_dragons.to_string(),
        ),
        (
            "Fire",
            dragons.ally_fire_dragons.to_string(),
            UNKNOWN.to_owned(),
        ),
        ("Ocean", UNKNOWN.to_owned(), UNKNOWN.to_owned()),
        ("Chemtech", UNKNOWN.to_owned(), UNKNOWN.to_owned()),
    ];

    let text = |text: String| Cell::new(div().child(text));
    let mut columns = [Vec::new(), Vec::new(), Vec::new()];

    for (name, allies, enemies) in rows {
        columns[0].push(Cell::new(
            div().text_sm().text_color(rgb(theme::STD_400)).child(name),
        ));
        columns[1].push(text(allies));
        columns[2].push(text(enemies));
    }

    let [names, allies, enemies] = columns;

    panel()
        .child(crate::components::tables::table(
            "live-dragons",
            vec![
                Column {
                    header: Cell::empty(),
                    cells: names,
                },
                Column {
                    header: Cell::new(div().child("Allies")),
                    cells: allies,
                },
                Column {
                    header: Cell::new(div().child("Enemies")),
                    cells: enemies,
                },
            ],
        ))
        .into_any_element()
}

/// `AbilityLevelsDisplay`: Q, W, E and R with their levels.
fn ability_levels(game: &Game) -> AnyElement {
    let champion = game.current_player.champion_id;
    let levels = game.ability_levels;

    let (icons, values) = AbilityLevels::ABILITIES
        .iter()
        .map(|&key| {
            let ability = champion
                .abilities()
                .iter()
                .find(|metadata| metadata.kind.as_char() == key.as_char())
                .map(|metadata| metadata.kind);

            let icon = match ability {
                Some(ability) => Cell::new(ability_image(champion, ability).size(px(32.))),
                None => Cell::new(div().child(key.as_char().to_string())),
            };

            (icon, Cell::new(div().child(levels[key].to_string())))
        })
        .unzip();

    panel()
        .child(crate::components::tables::table(
            "live-abilities",
            vec![
                Column {
                    header: Cell::empty(),
                    cells: icons,
                },
                Column {
                    header: Cell::new(div().child("Level")),
                    cells: values,
                },
            ],
        ))
        .into_any_element()
}

impl Render for LivegameView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(game) = self.game.clone() else {
            // `flex gap-4 p-4`: the error, an empty table and the stack placeholder.
            return div()
                .flex()
                .flex_col()
                .gap_4()
                .p_4()
                .children(
                    self.error
                        .clone()
                        .map(|error| div().text_sm().text_color(rgb(theme::STD_400)).child(error)),
                )
                .child(panel().child(empty_table("live-empty", 5)))
                .child(panel().h_96())
                .into_any_element();
        };

        let wide = window.viewport_size().width >= px(XL_BREAKPOINT);
        let [allies, enemies] = scoreboard(&game);

        // `flex py-4 pl-4 pr-2 flex-col gap-4 min-w-80`
        let player = div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .map(|column| match wide {
                true => column.w(px(360.)).flex_none(),
                false => column.w_full(),
            })
            .child(banner(&game, wide))
            .child(allies)
            .child(enemies)
            .child(
                div()
                    .flex()
                    .gap_4()
                    .items_start()
                    .child(div().flex_1().child(dragons(&game)))
                    .child(div().flex_1().child(ability_levels(&game))),
            );

        let title = |text: &'static str| {
            div()
                .px_6()
                .py_5()
                .text_xl()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(theme::STD_400))
                .child(text)
        };

        // `flex flex-col py-4 pl-2 pr-4 gap-4 flex-1 overflow-auto`
        let tables = div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .flex_1()
            .min_w_0()
            .child(panel().child(CachedTable::view(&self.damages, cx)))
            .child(self.stack.clone())
            .child(
                panel()
                    .child(title("Bonus damage after buying new item"))
                    .child(CachedTable::view(&self.recommendations, cx)),
            )
            .child(
                panel()
                    .child(div().p_4().child(self.simulated_selector.clone()))
                    .child(CachedTable::view(&self.simulated, cx)),
            );

        div()
            .flex()
            .w_full()
            .map(|page| match wide {
                true => page.items_start(),
                false => page.flex_col(),
            })
            .child(player)
            .child(tables)
            .into_any_element()
    }
}

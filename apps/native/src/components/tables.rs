//! Damage tables: the web app's `components/tables/{header,body,empty,turret}.rs`.
//!
//! gpui has no table layout. Tables are laid out by column: every column sizes to its widest
//! cell, and every cell has the same height, so rows stay aligned. Rows get their background
//! and borders cell by cell (`tbody tr:nth-child(odd|even)` in `main.css`).
use crate::{
    components::{
        code::Highlighted,
        docs::{Builtin, DocView, Docs, cell_blocks},
        hover_docs::HoverDocsExt,
        image::{Entry, ability_image, asset_image, image},
    },
    theme,
};
use gpui::{
    AnyElement, ElementId, FontWeight, IntoElement, ParentElement, SharedString, Styled, div,
    prelude::*, px, rems, rgb,
};
use math::{
    AbilityId, AbilityName, AdaptiveType, ChampionId, CtxVar, DamageType, EntityId, ItemId,
    MergeData, RuneId, TypeMetadata, ValueId,
    model::{Attacks, Damages},
};
use std::rc::Rc;

/// `table tr { height: 48px }`. Rows are exactly this tall, so the cached tables view can
/// compute its height (see `results.rs`).
pub const ROW_HEIGHT: f32 = 48.0;
/// `table.data-table img { height: 32px }`.
const ICON_SIZE: f32 = 32.0;

pub type BuildDocs = Rc<dyn Fn() -> Vec<Highlighted>>;

/// One table cell: its content and, optionally, its documentation.
pub struct Cell {
    content: AnyElement,
    docs: Option<BuildDocs>,
}

impl Cell {
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            content: content.into_any_element(),
            docs: None,
        }
    }

    pub fn empty() -> Self {
        Self::new(div())
    }

    pub fn docs(mut self, docs: impl Fn() -> Vec<Highlighted> + 'static) -> Self {
        self.docs = Some(Rc::new(docs));
        self
    }
}

pub struct Column {
    pub header: Cell,
    pub cells: Vec<Cell>,
}

/// Lays out columns of equal length as a table that scrolls horizontally when too wide.
pub fn table(id: &'static str, columns: Vec<Column>) -> impl IntoElement {
    let rows = columns.first().map_or(0, |column| column.cells.len());

    let columns = columns.into_iter().enumerate().map(|(x, column)| {
        let header = cell_frame((id, x << 4), column.header).bg(rgb(theme::STD_900));

        let cells = column.cells.into_iter().enumerate().map(move |(y, cell)| {
            let odd = y % 2 == 0;

            cell_frame((id, x << 4 | (y + 1)), cell).map(|frame| match odd {
                true => frame.bg(rgb(theme::ROW_ODD)),
                false => frame
                    .bg(rgb(theme::ROW_EVEN))
                    .border_color(rgb(theme::STD_700))
                    .border_t_1()
                    // `tbody tr:last-child { border-bottom: none }`
                    .when(y + 1 < rows, |frame| frame.border_b_1()),
            })
        });

        div()
            .flex()
            .flex_col()
            .flex_grow()
            .flex_shrink_0()
            .child(header)
            .children(cells)
    });

    div()
        .id(id)
        .w_full()
        .overflow_x_scroll()
        .child(div().flex().min_w_full().children(columns))
}

fn cell_frame(id: (&'static str, usize), cell: Cell) -> gpui::Stateful<gpui::Div> {
    let frame = div()
        .id(ElementId::from(id))
        .flex()
        .items_center()
        .justify_center()
        .h(px(ROW_HEIGHT))
        // `table th, table td { padding: 0 8px }`
        .px_2()
        .whitespace_nowrap();

    let frame = match cell.docs {
        Some(docs) => frame.docs(move || docs()),
        None => frame,
    };

    frame.child(cell.content)
}

/// The placeholder shown before the first result: `EmptyTable`.
pub fn empty_table(id: &'static str, rows: usize) -> impl IntoElement {
    table(
        id,
        vec![Column {
            header: Cell::empty(),
            cells: (0..rows).map(|_| Cell::empty()).collect(),
        }],
    )
}

/// The web's `model::AbilityKind`: an ability, or the alias of a merged min/max pair.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbilityKind {
    Alias(MergeData),
    Normal(AbilityId),
}

impl AbilityKind {
    pub const fn ability_id(&self) -> AbilityId {
        match self {
            Self::Alias(merge) => merge.alias,
            Self::Normal(ability_id) => *ability_id,
        }
    }
}

/// An ability icon with its letter and name drawn over it (`ImageType::header`).
pub fn ability_icon(champion: ChampionId, ability: AbilityId, size: f32) -> impl IntoElement {
    let name = ability.ability_name();
    let label = match name {
        AbilityName::Mega => Some(("MEGA", "")),
        AbilityName::Void => None,
        name if name < AbilityName::Mega => {
            let index = name as u8;
            let number =
                AbilityName::from_repr(index % AbilityName::JMP).map_or("", |name| name.short());

            let suffix = match index / AbilityName::JMP {
                1 => "MIN",
                2 => "MAX",
                _ => "",
            };

            Some(match number.is_empty() {
                true => (suffix, ""),
                false => (number, suffix),
            })
        }
        _ => None,
    };

    // `.img-letter`: bold white text centered over the image. gpui has no text shadow.
    let letter = div()
        .absolute()
        .inset_0()
        .flex()
        .items_center()
        .justify_center()
        .font_weight(FontWeight::BOLD)
        .text_sm()
        .text_color(rgb(theme::TEXT))
        .child(ability.as_char().to_string())
        .children(label.map(|(first, suffix)| {
            // `<sub>{first}<sub>{suffix}</sub></sub>`
            div()
                .flex()
                .items_end()
                .mt(px(8.))
                .text_size(px(9.))
                .child(first)
                .when(!suffix.is_empty(), |sub| {
                    sub.child(div().mt(px(6.)).text_size(px(7.)).child(suffix))
                })
        }));

    div()
        .relative()
        .size(px(size))
        .flex_none()
        .child(ability_image(champion, ability).size_full())
        .child(letter)
}

/// Header icons: `TableHeader`.
#[derive(Clone, Copy)]
enum Header {
    BasicAttack,
    CritStrike,
    Onhit,
    Ability(ChampionId, AbilityKind),
    Item(ItemId),
    Rune(RuneId),
}

impl Header {
    fn list(
        champion_id: ChampionId,
        items_meta: &[TypeMetadata<ItemId>],
        runes_meta: &[TypeMetadata<RuneId>],
    ) -> Vec<Self> {
        let abilities = champion_id.abilities();
        let merge_data = champion_id.merge_data();
        let mut headers = vec![Self::BasicAttack, Self::CritStrike, Self::Onhit];

        for (i, metadata) in abilities.iter().enumerate() {
            // The max of a merged pair shares the min's column.
            if merge_data.iter().any(|merge| merge.max as usize == i) {
                continue;
            }

            let kind = match merge_data.iter().find(|merge| merge.min as usize == i) {
                Some(&merge) => AbilityKind::Alias(merge),
                None => AbilityKind::Normal(metadata.kind),
            };

            headers.push(Self::Ability(champion_id, kind));
        }

        headers.extend(items_meta.iter().map(|meta| Self::Item(meta.kind)));
        headers.extend(runes_meta.iter().map(|meta| Self::Rune(meta.kind)));
        headers
    }

    /// `ImageType::docs`.
    fn docs(self) -> Docs {
        match self {
            Self::BasicAttack => Docs::Builtin(Builtin::BasicAttack, DocView::Global),
            Self::CritStrike => Docs::Builtin(Builtin::CriticalStrike, DocView::Global),
            Self::Onhit => Docs::Builtin(Builtin::Onhit, DocView::Global),
            Self::Ability(champion, AbilityKind::Normal(ability)) => {
                Docs::Champion(champion, ability)
            }
            Self::Ability(champion, AbilityKind::Alias(merge)) => {
                let abilities = champion.abilities();

                Docs::ChampionMerged(
                    champion,
                    abilities[merge.min as usize].kind,
                    abilities[merge.max as usize].kind,
                )
            }
            Self::Item(item) => Docs::global(item),
            Self::Rune(rune) => Docs::global(rune),
        }
    }

    fn cell(self) -> Cell {
        let icon = match self {
            Self::BasicAttack => asset_image("other/basic_attack.png")
                .size(px(ICON_SIZE))
                .into_any_element(),
            Self::CritStrike => asset_image("stats/crit_chance.svg")
                .size(px(ICON_SIZE))
                .into_any_element(),
            Self::Onhit => asset_image("stats/onhit.svg")
                .size(px(ICON_SIZE))
                .into_any_element(),
            Self::Ability(champion, kind) => {
                ability_icon(champion, kind.ability_id(), ICON_SIZE).into_any_element()
            }
            Self::Item(item) => image(item.image_source())
                .size(px(ICON_SIZE))
                .into_any_element(),
            Self::Rune(rune) => image(rune.image_source())
                .size(px(ICON_SIZE))
                .into_any_element(),
        };

        let docs = self.docs();

        Cell::new(icon).docs(move || docs.blocks())
    }
}

/// One damage cell: `body::Cell`. The calculator shows no differences, so there is no `diff`.
struct Damage {
    damage_type: DamageType,
    min: i32,
    max: Option<i32>,
    idents: &'static [CtxVar],
    docs: Docs,
}

impl Damage {
    fn value(value: i32) -> String {
        match value < 0 {
            true => format!("({value})"),
            false => value.to_string(),
        }
    }

    /// `Cell::render_range`: `None` for a zero value.
    fn range(min: i32, max: Option<i32>) -> Option<String> {
        match max {
            Some(max) if max != 0 && max > min => {
                Some(format!("{} - {}", Self::value(min), Self::value(max)))
            }
            Some(max) if max != 0 && max < min => {
                Some(format!("{} - {}", Self::value(max), Self::value(min)))
            }
            Some(max) if max == min && min != 0 => Some(min.to_string()),
            Some(0) | None if min != 0 => Some(min.to_string()),
            _ => None,
        }
    }

    /// The main line, falling back to the minimum like `Cell::render`.
    fn text(min: i32, max: Option<i32>) -> String {
        Self::range(min, max).unwrap_or_else(|| min.to_string())
    }

    /// The difference from the same cell of another row, as the simulated item table shows.
    fn diff(&self, base: &Self) -> (i32, Option<i32>) {
        (
            self.min - base.min,
            self.max.map(|max| max - base.max.unwrap_or(0)),
        )
    }

    fn cell(
        self,
        damages: &Damages<'_>,
        adaptive_type: AdaptiveType,
        diff: Option<(i32, Option<i32>)>,
    ) -> Cell {
        let color = match self.damage_type {
            DamageType::Adaptive => theme::adaptive_color(adaptive_type),
            damage_type => theme::damage_color(damage_type),
        };

        let ctx = damages.ctx;
        let Self { idents, docs, .. } = self;
        let diff = diff.and_then(|(min, max)| Self::range(min, max));

        // `flex flex-col items-center leading-tight`, the difference in `text-std-400 text-xs`.
        Cell::new(
            div()
                .flex()
                .flex_col()
                .items_center()
                .line_height(rems(1.25))
                .text_color(rgb(color))
                .child(Self::text(self.min, self.max))
                .children(diff.map(|diff| {
                    div()
                        .text_xs()
                        .line_height(rems(1.))
                        .text_color(rgb(theme::STD_400))
                        .child(diff)
                })),
        )
        .docs(move || cell_blocks(idents, &ctx, docs))
    }
}

/// A damage by index, zero when the row has no such column: a new purchase has no baseline.
fn at(values: &[i32], index: usize) -> i32 {
    values.get(index).copied().unwrap_or(0)
}

/// Damage cells of one row, in header order: `DamageRenderer`.
fn damage_row(
    damages: &Damages<'_>,
    champion_id: ChampionId,
    items_meta: &[TypeMetadata<ItemId>],
    runes_meta: &[TypeMetadata<RuneId>],
) -> Vec<Damage> {
    let Attacks {
        basic_attack,
        critical_strike,
        onhit_damage,
    } = damages.attacks;

    let mut cells = vec![
        Damage {
            damage_type: DamageType::Physical,
            min: basic_attack,
            max: None,
            idents: &[CtxVar::AttackDamage, CtxVar::PhysicalMultiplier],
            docs: Docs::Builtin(Builtin::BasicAttack, DocView::Function),
        },
        Damage {
            damage_type: DamageType::Physical,
            min: critical_strike,
            max: None,
            idents: &[CtxVar::AttackDamage, CtxVar::CritDamage],
            docs: Docs::Builtin(Builtin::CriticalStrike, DocView::Function),
        },
        Damage {
            damage_type: DamageType::Mixed,
            min: onhit_damage.minimum_damage,
            max: Some(onhit_damage.maximum_damage),
            idents: &[],
            docs: Docs::Builtin(Builtin::Onhit, DocView::Function),
        },
    ];

    let abilities = champion_id.abilities();
    let merge_data = champion_id.merge_data();
    let identifiers = champion_id.identifiers();
    // Cell of each ability, so the max of a merged pair can find its min.
    let mut cell_of = vec![usize::MAX; abilities.len()];

    for (i, meta) in abilities.iter().enumerate() {
        if let Some(merge) = merge_data.iter().find(|merge| merge.max as usize == i) {
            let min = merge.min as usize;
            let cell = &mut cells[cell_of[min]];

            cell.max = Some(at(damages.abilities, i));
            cell.docs = Docs::ChampionMerged(champion_id, abilities[min].kind, meta.kind);
            continue;
        }

        cell_of[i] = cells.len();
        cells.push(Damage {
            damage_type: meta.damage_type,
            min: at(damages.abilities, i),
            max: None,
            idents: identifiers[i],
            docs: Docs::Champion(champion_id, meta.kind),
        });
    }

    fn values<T: ValueId + Copy>(
        cells: &mut Vec<Damage>,
        damages: &[i32],
        metadata: &[TypeMetadata<T>],
        idents: fn(T) -> &'static [CtxVar],
    ) {
        // Damages hold a min/max pair per metadata entry.
        for (k, meta) in metadata.iter().enumerate() {
            let id = meta.kind;

            cells.push(Damage {
                damage_type: meta.damage_type,
                min: at(damages, k * 2),
                max: Some(at(damages, k * 2 + 1)),
                idents: idents(id),
                docs: match id.entity() {
                    EntityId::Item(item) => Docs::Item(item),
                    EntityId::Rune(rune) => Docs::Rune(rune),
                    EntityId::Champion(_) => unreachable!("values are items or runes"),
                },
            });
        }
    }

    values(&mut cells, damages.items, items_meta, |id| id.identifiers());
    values(&mut cells, damages.runes, runes_meta, |id| id.identifiers());
    cells
}

/// Header and damage columns, for rows of damages against several targets.
pub fn damage_columns(
    rows: &[Damages<'_>],
    champion_id: ChampionId,
    adaptive_type: AdaptiveType,
    items_meta: &[TypeMetadata<ItemId>],
    runes_meta: &[TypeMetadata<RuneId>],
) -> Vec<Column> {
    damage_columns_with_diff(
        rows,
        None,
        champion_id,
        adaptive_type,
        items_meta,
        runes_meta,
    )
}

/// [`damage_columns`], with each cell's difference from the same cell of `bases` (one per row)
/// below it, like `DamageRenderer::new(.., Some(other))`.
pub fn damage_columns_with_diff(
    rows: &[Damages<'_>],
    bases: Option<&[Damages<'_>]>,
    champion_id: ChampionId,
    adaptive_type: AdaptiveType,
    items_meta: &[TypeMetadata<ItemId>],
    runes_meta: &[TypeMetadata<RuneId>],
) -> Vec<Column> {
    let mut columns = Header::list(champion_id, items_meta, runes_meta)
        .into_iter()
        .map(|header| Column {
            header: header.cell(),
            cells: Vec::with_capacity(rows.len()),
        })
        .collect::<Vec<_>>();

    for (row, damages) in rows.iter().enumerate() {
        let cells = damage_row(damages, champion_id, items_meta, runes_meta);
        let base = bases
            .and_then(|bases| bases.get(row))
            .map(|base| damage_row(base, champion_id, items_meta, runes_meta));

        debug_assert_eq!(cells.len(), columns.len());

        for (i, (column, cell)) in columns.iter_mut().zip(cells).enumerate() {
            let diff = base.as_ref().map(|base| cell.diff(&base[i]));
            column.cells.push(cell.cell(damages, adaptive_type, diff));
        }
    }

    columns
}

/// The turret table: one column per tower, with its number over the icon (`TurretTable`).
pub fn turret_columns(tower_damages: &[i32], adaptive_type: AdaptiveType) -> Vec<Column> {
    let color = theme::adaptive_color(adaptive_type);

    tower_damages
        .iter()
        .enumerate()
        .map(|(i, &damage)| Column {
            header: Cell::new(
                div()
                    .relative()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(asset_image("other/tower.webp").size(px(ICON_SIZE)))
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .child(SharedString::from(i.to_string())),
                    ),
            )
            .docs(|| Docs::Builtin(Builtin::Tower, DocView::Global).blocks()),
            cells: vec![
                Cell::new(div().text_color(rgb(color)).child(damage.to_string()))
                    .docs(|| Docs::Builtin(Builtin::Tower, DocView::Function).blocks()),
            ],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::Damage;

    /// `nonzero_differences_remain_visible_with_zero_or_absent_upper_bound` and the ordering of
    /// `Cell::render_range`.
    #[test]
    fn ranges_order_values_and_keep_single_values() {
        assert_eq!(Damage::text(10, Some(20)), "10 - 20");
        assert_eq!(Damage::text(20, Some(10)), "10 - 20");
        assert_eq!(Damage::text(-15, Some(5)), "(-15) - 5");
        assert_eq!(Damage::text(25, Some(0)), "25");
        assert_eq!(Damage::text(25, None), "25");
        assert_eq!(Damage::text(0, None), "0");
    }
}

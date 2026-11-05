//! Champion documentation: the web app's `formulas/components/champions.rs`.
use crate::{
    components::{
        code::{Code, Highlighted},
        h2,
        hover_docs::global_docs,
        image::{Entry, image, position_image},
        paragraph, section,
        selector::{Selected, Selector},
    },
    formulas::is_compact,
    random::random_variant,
    theme,
};
use gpui::{
    Context, Div, Entity, FontWeight, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Window, div, prelude::*, px, rgb,
};
use strum::VariantArray;
use tutorlolv2::{CastId, ChampionId, Position};
use tutorlolv2_codec::Spans;

/// `table tr { height: 48px }`.
const ROW_HEIGHT: f32 = 48.0;

pub struct ChampionFormulas {
    champion: ChampionId,
    selector: Entity<Selector<ChampionId>>,
    global: Highlighted,
    functions: Vec<Highlighted>,
    generator: Option<Highlighted>,
    _selected: Subscription,
}

impl ChampionFormulas {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let champion = random_variant::<ChampionId>();
        let selector = cx.new(|cx| Selector::new(champion, cx));

        let selected = cx.subscribe(&selector, |this, _, Selected(champion), cx| {
            this.show(*champion);
            cx.notify();
        });

        let mut this = Self {
            champion,
            selector,
            global: Highlighted::plain(""),
            functions: Vec::new(),
            generator: None,
            _selected: selected,
        };

        this.show(champion);
        this
    }

    fn show(&mut self, champion: ChampionId) {
        self.champion = champion;
        self.global = champion.render_global_markup::<Spans>().into();
        self.functions = champion
            .metadata()
            .iter()
            .map(|meta| champion.render_fn_markup::<Spans>(meta.kind).into())
            .collect();

        self.generator = champion
            .render_generator_markup::<Spans>()
            .map(|spans| Highlighted::new(&spans));
    }

    /// The "Recommended items and runes per position" table.
    fn recommendations(&self, compact: bool) -> impl IntoElement {
        let champion = self.champion;

        // `table th, table td { padding: 0 8px }`, equal columns (`table-fixed`).
        let cell = || div().flex_1().min_w_0().px_2();

        let header = row()
            .bg(rgb(theme::STD_900))
            .children(Position::VARIANTS.iter().map(|&position| {
                cell().flex().items_center().child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .when(compact, |header| header.mx_auto())
                        .child(position_image(position).size_8().flex_none())
                        .when(!compact, |header| {
                            header.child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(theme::STD_200))
                                    .truncate()
                                    .child(position.to_string()),
                            )
                        }),
                )
            }));

        let items = row()
            .bg(rgb(theme::ROW_ODD))
            .children(Position::VARIANTS.iter().map(|&position| {
                cell().child(column(
                    position,
                    "item",
                    champion.recommended_items(position),
                    compact,
                ))
            }));

        let runes = row()
            .bg(rgb(theme::ROW_EVEN))
            .border_t_1()
            .border_color(rgb(theme::STD_700))
            .children(Position::VARIANTS.iter().map(|&position| {
                cell().child(column(
                    position,
                    "rune",
                    champion.recommended_runes(position),
                    compact,
                ))
            }));

        div()
            .flex()
            .flex_col()
            .child(header)
            .child(items)
            .child(runes)
    }
}

/// Recommended entries of one position: `flex flex-col gap-2 py-2`.
fn column<T: Entry>(
    position: Position,
    prefix: &'static str,
    entries: &'static [T],
    compact: bool,
) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .py_2()
        .children(entries.iter().map(move |&entry| {
            div()
                .id(SharedString::from(format!(
                    "{prefix}-{position:?}-{}",
                    entry.index()
                )))
                .flex()
                .items_center()
                .gap_3()
                .overflow_hidden()
                .when(compact, |entry| entry.justify_center())
                .hoverable_tooltip(global_docs(entry))
                .child(image(entry.image_source()).size_7().flex_none())
                .when(!compact, |row| {
                    row.child(
                        div()
                            .text_color(rgb(theme::STD_300))
                            .truncate()
                            .child(entry.name()),
                    )
                })
        }))
}

/// One table row: `table tr { height: 48px }` is a minimum for rows with entries.
fn row() -> Div {
    div().flex().w_full().min_h(px(ROW_HEIGHT))
}

impl Render for ChampionFormulas {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        section()
            .child(h2("Champions"))
            .child(paragraph(concat!(
                "Documentation for formulas being used to calculate the damage of ",
                "a champion's abilities, its recommended items, runes per position, ",
                "and other basic information about it"
            )))
            .child(self.selector.clone())
            .child(h2("Recommended items and runes per position"))
            .child(self.recommendations(is_compact(window)))
            .child(h2("Source code definition"))
            .child(Code(self.global.clone()))
            // The web leaves this section empty: its content is commented out.
            .child(h2("Abilities virtual definiton"))
            .child(h2("Internal ability functions"))
            .children(self.functions.iter().cloned().map(Code))
            .child(h2("Champion generator implementation"))
            .children(self.generator.clone().map(Code))
    }
}

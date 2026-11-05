//! Searchable champion/item/rune picker: the web app's `components/selector.rs`.
use crate::{
    components::{
        hover_docs::HoverDocsExt,
        image::{Entry, image},
        text_input::{TextInput, TextInputEvent},
    },
    theme,
};
use gpui::{
    Context, Entity, EventEmitter, FontWeight, IntoElement, ParentElement, Render, Styled,
    Subscription, Window, anchored, deferred, div, prelude::*, px, rems, rgb, uniform_list,
};

/// `py-0.5` around a `h-6` image.
const OPTION_HEIGHT: f32 = 28.0;
/// `max-h-72`.
const DROPDOWN_MAX_HEIGHT: f32 = 288.0;

/// Emitted when the user picks an entry.
pub struct Selected<T>(pub T);

pub struct Selector<T: Entry> {
    value: T,
    input: Entity<TextInput>,
    /// Entries matching the current query, in enum order.
    options: Vec<T>,
    open: bool,
    dropdown_hovered: bool,
    _input_events: Subscription,
}

impl<T: Entry> EventEmitter<Selected<T>> for Selector<T> {}

impl<T: Entry> Selector<T> {
    pub fn new(value: T, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| TextInput::new(value.name(), cx));

        let input_events = cx.subscribe(&input, |this, _, event, cx| {
            match event {
                TextInputEvent::Changed(query) => {
                    this.filter(query.trim());
                    this.open = !query.is_empty();
                }
                TextInputEvent::Focused => this.open = true,
            }

            cx.notify();
        });

        Self {
            value,
            input,
            options: T::VARIANTS.to_vec(),
            open: false,
            dropdown_hovered: false,
            _input_events: input_events,
        }
    }

    fn filter(&mut self, query: &str) {
        self.options = T::VARIANTS
            .iter()
            .copied()
            .filter(|entry| contains_ascii_case_insensitive(entry.name(), query))
            .collect();
    }

    fn select(&mut self, value: T, cx: &mut Context<Self>) {
        self.value = value;
        self.close(cx);
        self.input.update(cx, |input, cx| {
            input.clear(cx);
            input.set_placeholder(value.name(), cx);
        });

        cx.emit(Selected(value));
    }

    fn close(&mut self, cx: &mut Context<Self>) {
        self.open = false;
        self.dropdown_hovered = false;
        self.filter("");

        cx.notify();
    }

    fn dropdown(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.options.len();
        let height = (count as f32 * OPTION_HEIGHT).min(DROPDOWN_MAX_HEIGHT);

        let list = uniform_list(
            "selector-options",
            count,
            cx.processor(|this, range: std::ops::Range<usize>, _, cx| {
                this.options[range]
                    .iter()
                    .map(|&entry| {
                        div()
                            .id(entry.index())
                            .flex()
                            .items_center()
                            .gap_2()
                            .h(px(OPTION_HEIGHT))
                            .cursor_pointer()
                            .hover_docs(entry)
                            .on_click(cx.listener(move |this, _, _, cx| this.select(entry, cx)))
                            .child(image(entry.image_source()).size_6().flex_none())
                            .child(div().truncate().child(entry.name()))
                    })
                    .collect::<Vec<_>>()
            }),
        )
        .h(px(height));

        // `mt-2 px-1.5 py-1 w-96 bg-[#16161c] border border-std-800`
        div()
            .id("selector-dropdown")
            .mt_2()
            .px_1p5()
            .py_1()
            .w_96()
            .bg(rgb(theme::DROPDOWN))
            .border_1()
            .border_color(rgb(theme::STD_800))
            .occlude()
            .on_hover(cx.listener(|this, hovered: &bool, _, _| this.dropdown_hovered = *hovered))
            .child(list)
    }
}

impl<T: Entry> Render for Selector<T> {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let value = self.value;

        let label = div()
            .flex()
            .items_center()
            .gap_4()
            .child(
                div()
                    .id("selector-image")
                    .flex_none()
                    .hover_docs(value)
                    .child(image(value.image_source()).size_12()),
            )
            .child(
                // `text-std-200 placeholder:text-std-500 text-3xl font-medium w-full`
                div()
                    .flex_1()
                    .min_w_0()
                    .text_3xl()
                    .line_height(rems(2.25))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(theme::STD_200))
                    .child(self.input.clone()),
            )
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                if this.open && !this.dropdown_hovered {
                    this.close(cx);
                }
            }));

        // `empty:hidden`: no dropdown when nothing matches. The zero-height wrapper places the
        // anchored dropdown right below the label; `deferred` paints it above later content.
        let dropdown = (self.open && !self.options.is_empty()).then(|| {
            div()
                .h_0()
                .child(deferred(anchored().child(self.dropdown(cx))).with_priority(1))
        });

        div().flex().flex_col().child(label).children(dropdown)
    }
}

/// Match the web app's ASCII-only search (`utils/search.rs`).
fn contains_ascii_case_insensitive(name: &str, query: &str) -> bool {
    query.is_empty()
        || name
            .as_bytes()
            .windows(query.len())
            .any(|part| part.eq_ignore_ascii_case(query.as_bytes()))
}

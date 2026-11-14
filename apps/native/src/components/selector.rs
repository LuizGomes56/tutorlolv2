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

/// The web passes different classes per use; these are the two in use.
#[derive(Clone, Copy, PartialEq)]
enum Variant {
    /// The formulas page: a large name next to a large image.
    Page,
    /// Champion banners in the calculator: `gap-2 p-2`, a `w-8` image and
    /// `font-bold text-lg text-std-400 placeholder:text-white`.
    Banner,
    /// The live game's simulated item: a `w-10` image and `text-xl font-medium`.
    Compact,
}

pub struct Selector<T: Entry> {
    value: T,
    input: Entity<TextInput>,
    /// Every entry that can be picked.
    all: &'static [T],
    /// Entries matching the current query, in enum order.
    options: Vec<T>,
    open: bool,
    dropdown_hovered: bool,
    variant: Variant,
    _input_events: Subscription,
}

impl<T: Entry> EventEmitter<Selected<T>> for Selector<T> {}

impl<T: Entry> Selector<T> {
    pub fn new(value: T, cx: &mut Context<Self>) -> Self {
        Self::with_variant(value, Variant::Page, cx)
    }

    pub fn banner(value: T, cx: &mut Context<Self>) -> Self {
        Self::with_variant(value, Variant::Banner, cx)
    }

    /// A smaller selector limited to `options`.
    pub fn compact(value: T, options: &'static [T], cx: &mut Context<Self>) -> Self {
        let mut selector = Self::with_variant(value, Variant::Compact, cx);

        selector.all = options;
        selector.filter("");
        selector
    }

    /// Shows another value without emitting [`Selected`].
    pub fn set_value(&mut self, value: T, cx: &mut Context<Self>) {
        if self.value != value {
            self.value = value;
            self.input
                .update(cx, |input, cx| input.set_placeholder(value.name(), cx));
            cx.notify();
        }
    }

    fn with_variant(value: T, variant: Variant, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let input = TextInput::new(value.name(), cx);

            match variant {
                Variant::Page | Variant::Compact => input,
                Variant::Banner => input.placeholder_color(rgb(theme::TEXT)),
            }
        });

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
            all: T::VARIANTS,
            options: T::VARIANTS.to_vec(),
            open: false,
            dropdown_hovered: false,
            variant,
            _input_events: input_events,
        }
    }

    fn filter(&mut self, query: &str) {
        self.options = self
            .all
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
                            // List items size to their content: make the whole row clickable.
                            .w_full()
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
            .map(|dropdown| match self.variant {
                Variant::Page | Variant::Compact => dropdown.w_96(),
                // `w-full` of the banner. An anchored element has no width of its own, so
                // a relative width would be zero: use the panel's `w-64`.
                Variant::Banner => dropdown.w_64(),
            })
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

        let variant = self.variant;
        let label = div()
            .flex()
            .items_center()
            .map(|label| match variant {
                Variant::Banner => label.gap_2().p_2().overflow_hidden(),
                Variant::Page | Variant::Compact => label.gap_4(),
            })
            .child(
                div()
                    .id("selector-image")
                    .flex_none()
                    .hover_docs(value)
                    .child(image(value.image_source()).map(|image| match variant {
                        Variant::Banner => image.size_8(),
                        Variant::Compact => image.size_10(),
                        Variant::Page => image.size_12(),
                    })),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .map(|input| match variant {
                        // `font-bold text-lg text-std-400`
                        Variant::Banner => input
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(theme::STD_400)),
                        // `text-xl font-medium`
                        Variant::Compact => input
                            .text_xl()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(theme::STD_200)),
                        // `text-std-200 text-3xl font-medium`
                        Variant::Page => input
                            .text_3xl()
                            .line_height(rems(2.25))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(theme::STD_200)),
                    })
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
pub fn contains_ascii_case_insensitive(name: &str, query: &str) -> bool {
    query.is_empty()
        || name
            .as_bytes()
            .windows(query.len())
            .any(|part| part.eq_ignore_ascii_case(query.as_bytes()))
}

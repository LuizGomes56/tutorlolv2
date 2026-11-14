//! Small calculator controls: `Checkbox`, `SelectorButton` and the enemy panel buttons.
use crate::theme;
use gpui::{
    App, ClickEvent, ElementId, FontWeight, IntoElement, ParentElement, SharedString, Styled,
    Window, div, prelude::*, px, rems, rgb, rgba, svg,
};

type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// `inputs/checkbox.rs`: a violet box with a check mark, and its label.
pub fn checkbox(
    id: impl Into<ElementId>,
    label: &'static str,
    checked: bool,
    on_toggle: impl Fn(&bool, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let mark = div()
        .flex()
        .flex_none()
        .size(rems(0.875))
        .items_center()
        .justify_center()
        .border_1()
        .map(|mark| match checked {
            true => mark
                .border_color(rgb(theme::VIOLET_500))
                .bg(rgb(theme::VIOLET_600)),
            false => mark
                .border_color(rgb(theme::STD_600))
                .bg(rgb(theme::STD_900)),
        })
        .when(checked, |mark| {
            mark.child(
                svg()
                    .path("svgs/check.svg")
                    .size(rems(0.875))
                    .text_color(rgb(theme::TEXT)),
            )
        });

    // `inline-flex items-center gap-2 px-4 py-2 hover:bg-std-800/60`
    div()
        .id(id.into())
        .flex()
        .items_center()
        .gap_2()
        .px_4()
        .py_2()
        .cursor_pointer()
        .hover(|style| style.bg(rgba(theme::HOVER)))
        .on_click(move |_, window, cx| on_toggle(&!checked, window, cx))
        .child(mark)
        .child(
            div()
                .text_sm()
                .text_color(rgb(if checked { theme::TEXT } else { theme::STD_200 }))
                .child(label),
        )
}

/// `components/selector.rs::SelectorButton`: opens the item or rune picker.
pub fn selector_button(
    id: impl Into<ElementId>,
    title: &'static str,
    length: usize,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let on_click: OnClick = Box::new(on_click);

    div()
        .id(id.into())
        .group("selector-button")
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .w_full()
        .p_2()
        .border_y_1()
        .border_color(rgb(theme::STD_800))
        .cursor_pointer()
        .hover(|style| {
            style
                .bg(rgba(theme::HOVER))
                .border_color(rgb(theme::STD_700))
        })
        .on_click(on_click)
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .pl_2()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme::STD_100))
                        .child(title),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(theme::STD_400))
                        .child("Add / Remove"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_none()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .min_w_8()
                        .px_1p5()
                        .py_0p5()
                        .bg(rgba(theme::SKY_BADGE))
                        .border_1()
                        .border_color(rgba(theme::SKY_BADGE_BORDER))
                        .text_color(rgb(theme::SKY_300))
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .font_family(theme::CODE_FONT)
                        .flex()
                        .justify_center()
                        .child(SharedString::from(length.to_string())),
                )
                .child(
                    svg()
                        .path("svgs/rchev.svg")
                        .size_4()
                        .text_color(rgb(theme::STD_500))
                        .group_hover("selector-button", |style| {
                            style.text_color(rgb(theme::STD_300))
                        }),
                ),
        )
}

/// The enemy panel's `Add` and `Remove` buttons.
pub fn small_button(
    id: impl Into<ElementId>,
    label: &'static str,
    enabled: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    // `px-3 py-1.5 rounded-md border border-std-800 bg-std-900/70 text-xs font-semibold
    // text-std-200 enabled:hover:bg-std-800/60 disabled:opacity-40`
    div()
        .id(id.into())
        .px_3()
        .py_1p5()
        .rounded_md()
        .border_1()
        .border_color(rgb(theme::STD_800))
        .bg(rgba(theme::STD_900 << 8 | 0xb3))
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(theme::STD_200))
        .map(|button| match enabled {
            true => button
                .cursor_pointer()
                .hover(|style| style.bg(rgba(theme::HOVER)))
                .on_click(on_click),
            false => button.opacity(0.4),
        })
        .child(label)
}

/// One `[icon] [name] [input]` row of a `grid-cols-[auto_1fr_1fr]` grid: stats, dragons and
/// stack exceptions.
pub fn input_row(
    icon: impl IntoElement,
    name: impl Into<SharedString>,
    input: impl IntoElement,
    gap_y: f32,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_x_2()
        .py(px(gap_y / 2.))
        .child(
            div()
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .child(icon),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_sm()
                .truncate()
                .child(name.into()),
        )
        .child(div().flex_1().min_w_0().ml_2().child(input))
}

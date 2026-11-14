//! A horizontal slider: the web's `<input type="range">`, used by the replay timeline and the
//! charts. Clicking or dragging along the track sets the value. Optional `marks` shade ranges
//! of the track (missing capture in replays).
use crate::theme;
use gpui::{
    App, Bounds, ElementId, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent,
    ParentElement, Pixels, Point, Styled, Window, canvas, div, prelude::*, px, relative, rgb, rgba,
};
use std::{cell::Cell, rc::Rc};

type OnChange = Rc<dyn Fn(&f32, &mut Window, &mut App)>;

/// A slider from 0 to `max`. `marks` are `(from, to)` ranges in the same unit.
pub fn slider(
    id: impl Into<ElementId>,
    value: f32,
    max: f32,
    disabled: bool,
    marks: &[(f32, f32)],
    on_change: impl Fn(&f32, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let max = max.max(f32::EPSILON);
    let fraction = (value / max).clamp(0., 1.);
    let bounds = Rc::new(Cell::new(None::<Bounds<Pixels>>));
    let on_change: OnChange = Rc::new(on_change);

    let value_at = {
        let bounds = bounds.clone();

        move |position: Point<Pixels>| {
            bounds.get().map(|bounds: Bounds<Pixels>| {
                let x = (position.x - bounds.left()) / bounds.size.width;
                x.clamp(0., 1.) * max
            })
        }
    };

    let marks = marks.iter().map(|&(from, to)| {
        let left = (from / max).clamp(0., 1.);
        let width = ((to - from) / max).clamp(0., 1. - left);

        div()
            .absolute()
            .top(px(8.))
            .h(px(8.))
            .left(relative(left))
            .w(relative(width))
            .bg(rgba(0xf43f5e66))
    });

    let track = div()
        .absolute()
        .left_0()
        .right_0()
        .top(px(10.))
        .h(px(4.))
        .rounded_full()
        .bg(rgb(theme::STD_700));

    let fill = div()
        .absolute()
        .left_0()
        .top(px(10.))
        .h(px(4.))
        .w(relative(fraction))
        .rounded_full()
        .bg(rgb(0x38bdf8));

    let thumb = div()
        .absolute()
        .top(px(4.))
        .left(relative(fraction))
        .ml(px(-8.))
        .size_4()
        .rounded_full()
        .bg(rgb(theme::STD_100))
        .border_2()
        .border_color(rgb(0x0ea5e9));

    let measure = {
        let bounds = bounds.clone();

        canvas(move |frame, _, _| bounds.set(Some(frame)), |_, _, _, _| {})
            .absolute()
            .size_full()
    };

    div()
        .id(id.into())
        .relative()
        .h(px(24.))
        .w_full()
        .child(measure)
        .child(track)
        .child(fill)
        .children(marks)
        .child(thumb)
        .when(disabled, |slider| slider.opacity(0.45))
        .when(!disabled, |slider| {
            let down = {
                let value_at = value_at.clone();
                let on_change = on_change.clone();

                move |event: &MouseDownEvent, window: &mut Window, cx: &mut App| {
                    if let Some(value) = value_at(event.position) {
                        on_change(&value, window, cx);
                    }
                }
            };

            slider
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, down)
                .on_mouse_move(move |event: &MouseMoveEvent, window, cx| {
                    if event.pressed_button == Some(MouseButton::Left)
                        && let Some(value) = value_at(event.position)
                    {
                        on_change(&value, window, cx);
                    }
                })
        })
}

//! Shared controls for the pages that are not ports of a web layout (history, watch, settings,
//! recording and the overlay): buttons, cards, headings and status lines.
//!
//! The web styles these pages with a few global rules (`watch-page`, `watch-actions`, plain
//! `<button>`s). Natively they use one small set of styles in the app's palette instead.
use crate::theme;
use gpui::{
    App, ClickEvent, Div, ElementId, FontWeight, Hsla, ParentElement, SharedString, Stateful,
    Styled, Window, div, prelude::*, px, rgb, rgba,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    /// Sky, for the main action of a section.
    Primary,
    /// Neutral.
    Secondary,
    /// Rose, for removing things.
    Danger,
}

/// A button. Disabled buttons are dimmed and ignore clicks. `selected` marks the active
/// choice of a group (the web's `aria-pressed`).
pub fn button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    tone: Tone,
    disabled: bool,
    selected: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let (background, border, hover) = match (tone, selected) {
        (_, true) => (0x0ea5e933, 0x38bdf8ff, 0x0ea5e94d),
        (Tone::Primary, false) => (0x0284c7ff, 0x0ea5e9ff, 0x0ea5e9ff),
        (Tone::Secondary, false) => (0x27272aff, 0x3f3f46ff, 0x3f3f46ff),
        (Tone::Danger, false) => (0x27272aff, 0xf43f5e80, 0xf43f5e33),
    };

    div()
        .id(id.into())
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .px_3()
        .py_1p5()
        .rounded_md()
        .border_1()
        .border_color(rgba(border))
        .bg(rgba(background))
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgb(theme::STD_100))
        .child(label.into())
        .map(|button| match disabled {
            true => button.opacity(0.45),
            false => button
                .cursor_pointer()
                .hover(move |style| style.bg(rgba(hover)))
                .on_click(on_click),
        })
}

/// A bordered panel: `bg-std-900 border border-std-800 rounded-lg p-4`.
pub fn card() -> Div {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .p_4()
        .rounded_lg()
        .border_1()
        .border_color(rgb(theme::STD_800))
        .bg(rgb(theme::STD_900))
}

/// A page column with a title.
pub fn page(title: impl Into<SharedString>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .p_6()
        .pb_96()
        .max_w(px(1100.))
        .w_full()
        .child(
            div()
                .text_2xl()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme::STD_100))
                .child(title.into()),
        )
}

/// A section heading inside a card.
pub fn heading(text: impl Into<SharedString>) -> Div {
    div()
        .text_lg()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(theme::STD_100))
        .child(text.into())
}

/// Secondary text: hints and descriptions.
pub fn muted(text: impl Into<SharedString>) -> Div {
    div()
        .text_sm()
        .text_color(rgb(theme::STD_400))
        .child(text.into())
}

/// A status line (`role="status"`), or an error (`role="alert"`) when `alert` is set.
pub fn status(text: impl Into<SharedString>, alert: bool) -> Div {
    let color: Hsla = match alert {
        true => rgb(0xfda4af).into(),
        false => rgb(theme::SKY_300).into(),
    };

    div().text_sm().text_color(color).child(text.into())
}

/// A row of buttons that wraps.
pub fn actions() -> Div {
    div().flex().flex_wrap().items_center().gap_2()
}

/// `m:ss`, the web's `history::list::duration`.
pub fn duration(seconds: f32) -> String {
    let seconds = seconds.max(0.) as u32;
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

/// A Unix time in milliseconds as local date and time, `YYYY-MM-DD HH:MM`.
pub fn local_time(ms: i64) -> String {
    let (year, month, day, hour, minute) = local_parts(ms);
    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}")
}

#[cfg(windows)]
fn local_parts(ms: i64) -> (i32, u32, u32, u32, u32) {
    use windows::Win32::{
        Foundation::{FILETIME, SYSTEMTIME},
        System::Time::{FileTimeToSystemTime, SystemTimeToTzSpecificLocalTime},
    };

    // FILETIME counts 100 ns intervals since 1601.
    let ticks = (ms.max(0) as u64 + 11_644_473_600_000) * 10_000;
    let file = FILETIME {
        dwLowDateTime: ticks as u32,
        dwHighDateTime: (ticks >> 32) as u32,
    };

    let mut utc = SYSTEMTIME::default();
    let mut local = SYSTEMTIME::default();

    unsafe {
        if FileTimeToSystemTime(&file, &mut utc).is_err()
            || SystemTimeToTzSpecificLocalTime(None, &utc, &mut local).is_err()
        {
            return utc_parts(ms);
        }
    }

    (
        local.wYear as i32,
        local.wMonth as u32,
        local.wDay as u32,
        local.wHour as u32,
        local.wMinute as u32,
    )
}

#[cfg(not(windows))]
fn local_parts(ms: i64) -> (i32, u32, u32, u32, u32) {
    utc_parts(ms)
}

/// Civil date from days since 1970 (Howard Hinnant's algorithm).
fn utc_parts(ms: i64) -> (i32, u32, u32, u32, u32) {
    let seconds = ms.div_euclid(1000);
    let days = seconds.div_euclid(86_400);
    let time = seconds.rem_euclid(86_400);

    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = (yoe + era * 400 + i64::from(month <= 2)) as i32;

    (
        year,
        month,
        day,
        (time / 3600) as u32,
        (time % 3600 / 60) as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_and_dates_format() {
        assert_eq!(duration(0.), "0:00");
        assert_eq!(duration(-5.), "0:00");
        assert_eq!(duration(1865.4), "31:05");
        assert_eq!(utc_parts(0), (1970, 1, 1, 0, 0));
        // 2026-11-05 21:48 UTC
        assert_eq!(utc_parts(1_793_915_280_000), (2026, 11, 5, 21, 48));
    }
}

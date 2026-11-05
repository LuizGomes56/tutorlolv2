//! The application layout from the web app's `App` component and `index.html`:
//! a sidebar next to a scrollable page area.
use crate::{components::sidebar::Sidebar, route::Route, theme};
use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, prelude::*, px, rgb};

/// Tailwind's `md` breakpoint. Below it the web sidebar is `hidden`.
const SIDEBAR_BREAKPOINT: f32 = 768.0;

pub struct Shell {
    route: Route,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            route: Route::Homepage,
        }
    }

    pub fn navigate(&mut self, route: Route, cx: &mut Context<Self>) {
        if self.route != route {
            self.route = route;
            cx.notify();
        }
    }
}

impl Render for Shell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let show_sidebar = window.viewport_size().width >= px(SIDEBAR_BREAKPOINT);

        // Pages are not ported yet; show which route is active.
        let page = div().p_4().child(format!("{:?}", self.route));

        div()
            .flex()
            .size_full()
            .bg(rgb(theme::STD_900))
            .text_color(rgb(theme::TEXT))
            .when(show_sidebar, |shell| {
                shell.child(Sidebar::new(cx.entity().downgrade()))
            })
            .child(
                div()
                    .id("page")
                    .flex_1()
                    .h_full()
                    .overflow_y_scroll()
                    .child(page),
            )
    }
}

//! The application layout from the web app's `App` component and `index.html`:
//! a sidebar next to a scrollable page area.
use crate::{components::sidebar::Sidebar, formulas::Formulas, route::Route, theme};
use gpui::{
    AnyView, AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div,
    prelude::*, px, rgb,
};

/// Tailwind's `md` breakpoint. Below it the web sidebar is `hidden`.
const SIDEBAR_BREAKPOINT: f32 = 768.0;

pub struct Shell {
    route: Route,
    /// The current page. Pages are created on navigation, like a route change on the web,
    /// so their state (such as the random initial selection) starts fresh each visit.
    page: Option<AnyView>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            route: Route::Homepage,
            page: None,
        }
    }

    pub fn navigate(&mut self, route: Route, cx: &mut Context<Self>) {
        if self.route == route {
            return;
        }

        self.page = match route {
            Route::Formulas => Some(cx.new(Formulas::new).into()),
            _ => None,
        };
        self.route = route;

        cx.notify();
    }
}

impl Render for Shell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let show_sidebar = window.viewport_size().width >= px(SIDEBAR_BREAKPOINT);

        let page = match &self.page {
            Some(page) => page.clone().into_any_element(),
            // Not ported yet: show which route is active.
            None => div()
                .p_4()
                .child(format!("{:?}", self.route))
                .into_any_element(),
        };

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
                    // Flex items default to their content width; let the page shrink instead.
                    .min_w_0()
                    .h_full()
                    .overflow_y_scroll()
                    .child(page),
            )
    }
}

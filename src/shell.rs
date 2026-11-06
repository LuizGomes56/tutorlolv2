//! The application layout from the web app's `App` component and `index.html`:
//! a sidebar next to a scrollable page area.
use crate::{
    calculator::Calculator,
    components::{hover_docs, sidebar::Sidebar},
    formulas::Formulas,
    route::Route,
    theme,
};
use gpui::{
    AnyView, AppContext, Context, FocusHandle, IntoElement, KeyDownEvent, ParentElement, Render,
    Styled, Subscription, Window, div, prelude::*, px, rgb,
};

/// Tailwind's `md` breakpoint. Below it the web sidebar is `hidden`.
const SIDEBAR_BREAKPOINT: f32 = 768.0;

pub struct Shell {
    route: Route,
    /// The current page. Pages are created on navigation, like a route change on the web,
    /// so their state (such as the random initial selection) starts fresh each visit.
    page: Option<AnyView>,
    /// Keeps the shell on the keyboard dispatch path while no input is focused, so it sees
    /// Shift and Escape for the documentation tooltips.
    focus_handle: FocusHandle,
    _activation: Subscription,
}

impl Shell {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle);

        // `hover.js` hides the tooltip on `blur`.
        let activation = cx.observe_window_activation(window, |_, window, cx| {
            if !window.is_window_active() {
                hover_docs::hide(window, cx);
            }
        });

        Self {
            route: Route::Homepage,
            page: None,
            focus_handle,
            _activation: activation,
        }
    }

    pub fn navigate(&mut self, route: Route, cx: &mut Context<Self>) {
        if self.route == route {
            return;
        }

        self.page = match route {
            Route::Formulas => Some(cx.new(Formulas::new).into()),
            Route::Calculator => Some(cx.new(Calculator::new).into()),
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
            .track_focus(&self.focus_handle)
            .bg(rgb(theme::STD_900))
            .text_color(rgb(theme::TEXT))
            .on_modifiers_changed(hover_docs::modifiers_changed)
            .on_key_down(|event: &KeyDownEvent, window, cx| {
                if event.keystroke.key == "escape" {
                    hover_docs::hide(window, cx);
                }
            })
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
            .child(hover_docs::layer())
    }
}

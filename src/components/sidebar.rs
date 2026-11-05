//! Port of the web app's `components/sidebar.rs`.
use crate::{route::Route, shell::Shell, theme};
use gpui::{
    App, BoxShadow, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, WeakEntity, Window,
    div, point, prelude::*, px, rgb, svg,
};

/// Label, icon asset path and target route of one sidebar link.
type Link = (&'static str, Option<&'static str>, Route);

#[derive(IntoElement)]
pub struct Sidebar {
    shell: WeakEntity<Shell>,
}

impl Sidebar {
    pub fn new(shell: WeakEntity<Shell>) -> Self {
        Self { shell }
    }

    fn link(&self, link: Link) -> impl IntoElement {
        let (label, icon, route) = link;
        let shell = self.shell.clone();

        // `h-5 w-5`. Links without an icon keep the same text alignment.
        let icon = match icon {
            Some(path) => svg()
                .path(path)
                .size_5()
                .flex_none()
                .text_color(rgb(theme::STD_200))
                .into_any_element(),
            None => div().size_5().flex_none().into_any_element(),
        };

        div()
            .id(label)
            .flex()
            .items_center()
            .gap_4()
            .py_3()
            .px_4()
            .line_height(px(20.0))
            .rounded_md()
            .text_color(rgb(theme::STD_200))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(theme::STD_800)))
            .child(icon)
            .child(div().font_weight(FontWeight::MEDIUM).child(label))
            .on_click(move |_, _, cx| {
                let _ = shell.update(cx, |shell, cx| shell.navigate(route.clone(), cx));
            })
    }

    fn group<const N: usize>(&self, links: [Link; N]) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .children(links.map(|link| self.link(link)))
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let top = self.group([
            (
                "Homepage",
                Some("svgs/sidebar/Homepage.svg"),
                Route::Homepage,
            ),
            (
                "Calculator",
                Some("svgs/sidebar/Calculator.svg"),
                Route::Calculator,
            ),
            (
                "Livegame",
                Some("svgs/sidebar/Livegame.svg"),
                Route::Livegame,
            ),
            ("Watch", Some("svgs/sidebar/Watch.svg"), Route::Watch),
        ]);

        let bottom = self.group([
            (
                "Settings",
                Some("svgs/sidebar/Settings.svg"),
                Route::Settings,
            ),
            ("History", Some("svgs/sidebar/History.svg"), Route::History),
            (
                "Formulas",
                Some("svgs/sidebar/Formulas.svg"),
                Route::Formulas,
            ),
            ("Help & FAQ", Some("svgs/sidebar/Help.svg"), Route::Help),
            // The web app has no About or GitHub icons either.
            ("About", None, Route::About),
            ("GitHub", None, Route::GitHub),
        ]);

        let nav = div()
            .flex()
            .flex_col()
            .justify_between()
            .h_full()
            .py_4()
            .px_4()
            // gpui paints shadows under the element too; CSS only paints them outside.
            // The opaque background keeps the sidebar the same color as the web app.
            .bg(rgb(theme::STD_900))
            .shadow(vec![BoxShadow {
                color: rgb(theme::BOX_SHADOW).into(),
                offset: point(px(0.0), px(0.0)),
                blur_radius: px(10.0),
                spread_radius: px(2.5),
            }])
            .child(top)
            .child(bottom);

        div().w_48().h_full().flex_none().child(nav)
    }
}

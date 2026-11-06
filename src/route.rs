/// Pages of the web app's `yew_router` routes. Native has no URL bar, so this enum is
/// the router state held by [`crate::shell::Shell`].
// Variants without a page yet are constructed once their pages are ported.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Route {
    Homepage,
    Settings,
    Calculator,
    Watch,
    WatchCode { code: String },
    History,
    HistoryDetails { id: String },
    Livegame,
    Formulas,
    Help,
    About,
    Faq,
    GitHub,
}

/// The shell, so pages can change the route (the web's `Link<Route>` and `navigator.push`).
struct Navigator(gpui::WeakEntity<crate::shell::Shell>);

impl gpui::Global for Navigator {}

/// Installs the navigator. The shell calls this when it is created.
pub fn set_navigator(shell: gpui::WeakEntity<crate::shell::Shell>, cx: &mut gpui::App) {
    cx.set_global(Navigator(shell));
}

/// Opens `route`. Deferred so it can be called while a page is being updated.
pub fn navigate(route: Route, cx: &mut gpui::App) {
    let Some(shell) = cx
        .try_global::<Navigator>()
        .map(|navigator| navigator.0.clone())
    else {
        return;
    };

    cx.defer(move |cx| {
        let _ = shell.update(cx, |shell, cx| shell.navigate(route, cx));
    });
}

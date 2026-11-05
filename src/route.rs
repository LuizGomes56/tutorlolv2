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

mod charts;
mod details;
pub(crate) mod list;
pub(crate) mod source;
use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct HistoryProps {
    #[prop_or_default]
    pub id: String,
}
#[component]
pub fn History(props: &HistoryProps) -> Html {
    let account = use_context::<crate::account::AccountContext>().expect("account provider");
    if !cfg!(feature = "tauri") && account.state.email.is_none() {
        return html! {
            <main class={list::page_classes()}>
                <h1>{ "Match history" }</h1>
                <p>
                    { if account.loaded {"Sign in with Google to see your games."} else {"Checking account…"} }
                </p>
                <yew_router::components::Link<crate::Route> to={crate::Route::Watch}>
                    { "Watch a shared match" }
                </yew_router::components::Link<crate::Route>>
            </main>
        };
    }
    let key = format!("{:?}:{}", account.state.email, props.id);
    if props.id.is_empty() {
        html! { <list::HistoryListView key={key} /> }
    } else {
        html! { <details::HistoryDetailsView key={key} id={props.id.clone()} /> }
    }
}

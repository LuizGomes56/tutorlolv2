//! Shared safe identity state. Browser cookies and native session credentials never enter storage.
use crate::history::source::Command;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};
use tutorlolv2_protocol::{AccountInfo, AccountState};
use yew::{
    platform::{spawn_local, time::sleep},
    prelude::*,
};

thread_local! {
    static INVALIDATE: RefCell<Option<Callback<()>>> = const { RefCell::new(None) };
}

pub fn invalidate() {
    INVALIDATE.with(|callback| {
        if let Some(callback) = callback.borrow().as_ref() {
            callback.emit(());
        }
    });
}

#[derive(Clone, PartialEq)]
pub struct AccountContext {
    pub state: AccountState,
    pub loaded: bool,
    pub busy: bool,
    pub refresh: Callback<()>,
    pub sign_in: Callback<()>,
    pub sign_out: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct AccountProviderProps {
    pub children: Children,
}

async fn get_account() -> Result<AccountState, String> {
    if cfg!(feature = "tauri") {
        Command::GetAccount
            .invoke(serde_json::json!({}))
            .await
            .map_err(|e| e.message)
    } else {
        let response = crate::history::source::http_request("account", "GET", None)
            .await
            .map_err(|e| e.message)?;
        if response.status() == 401 {
            let cancelled = web_sys::window()
                .and_then(|window| {
                    let location =
                        crate::history::source::member(&window.into(), "location").ok()?;
                    crate::history::source::member(&location, "search")
                        .ok()?
                        .as_string()
                })
                .is_some_and(|query| {
                    query
                        .trim_start_matches('?')
                        .split('&')
                        .any(|part| part == "signin=cancelled")
                });
            return Ok(AccountState {
                last_error: cancelled.then(|| "Sign in cancelled. You can try again.".into()),
                ..Default::default()
            });
        }
        let info: AccountInfo = crate::history::source::http_decode(response)
            .await
            .map_err(|e| e.message)?;
        Ok(AccountState {
            email: Some(info.email),
            ..Default::default()
        })
    }
}

#[component]
pub fn AccountProvider(props: &AccountProviderProps) -> Html {
    let state = use_state(AccountState::default);
    let loaded = use_state(|| false);
    let busy = use_state(|| false);
    let revision = use_state(|| 0u32);
    let revision_counter = use_mut_ref(|| 0u32);
    let generation = use_mut_ref(|| 0u64);
    let active = use_mut_ref(|| true);
    {
        let (state, generation) = (state.clone(), generation.clone());
        use_effect_with((), move |_| {
            INVALIDATE.with(|v| {
                *v.borrow_mut() = Some(Callback::from(move |_| {
                    *generation.borrow_mut() += 1;
                    crate::history::source::clear_web_history();
                    state.set(AccountState {
                        last_error: Some("Your session expired. Sign in again.".into()),
                        ..Default::default()
                    });
                }))
            });
            || INVALIDATE.with(|v| *v.borrow_mut() = None)
        });
    }
    {
        let active = active.clone();
        use_effect_with((), move |_| move || *active.borrow_mut() = false);
    }
    {
        let (state, loaded, generation) = (state.clone(), loaded.clone(), generation.clone());
        use_effect_with(*revision, move |_| {
            let live = Rc::new(Cell::new(true));
            let running = live.clone();
            let before = *generation.borrow();
            spawn_local(async move {
                match get_account().await {
                    Ok(value) if running.get() && before == *generation.borrow() => {
                        if state.email != value.email {
                            crate::history::source::clear_web_history();
                        }
                        state.set(value);
                        loaded.set(true);
                    }
                    Err(message) if running.get() && before == *generation.borrow() => {
                        state.set(AccountState {
                            last_error: Some(message),
                            ..(*state).clone()
                        });
                        loaded.set(true);
                    }
                    _ => {}
                }
            });
            move || live.set(false)
        });
    }
    // Native account events invalidate the safe state; polling also recovers a missed event
    // during listener setup and reflects browser cancellation without carrying credentials.
    {
        let revision = revision.clone();
        let revision_counter = revision_counter.clone();
        use_effect_with((), move |_| {
            let live = Rc::new(Cell::new(true));
            let running = live.clone();
            spawn_local(async move {
                while running.get() && cfg!(feature = "tauri") {
                    sleep(Duration::from_millis(1000)).await;
                    if running.get() {
                        *revision_counter.borrow_mut() += 1;
                        revision.set(*revision_counter.borrow());
                    }
                }
            });
            move || live.set(false)
        });
    }
    let refresh = {
        let revision = revision.clone();
        let counter = revision_counter.clone();
        Callback::from(move |_| {
            *counter.borrow_mut() += 1;
            revision.set(*counter.borrow());
        })
    };
    let action = |command: Command| {
        let (state, busy, active, generation) = (
            state.clone(),
            busy.clone(),
            active.clone(),
            generation.clone(),
        );
        Callback::from(move |_| {
            if *busy {
                return;
            }
            busy.set(true);
            *generation.borrow_mut() += 1;
            let before = *generation.borrow();
            let (state, busy, active, generation) = (
                state.clone(),
                busy.clone(),
                active.clone(),
                generation.clone(),
            );
            spawn_local(async move {
                let result = if cfg!(feature = "tauri") {
                    command
                        .invoke::<AccountState>(serde_json::json!({}))
                        .await
                        .map_err(|e| e.message)
                } else {
                    match crate::history::source::http_request("auth/logout", "POST", Some(vec![]))
                        .await
                    {
                        Ok(response) if response.status() == 204 => Ok(AccountState::default()),
                        Ok(response) => Err(format!(
                            "Sign out failed (HTTP {}). Retry.",
                            response.status()
                        )),
                        Err(e) => Err(e.message),
                    }
                };
                if !*active.borrow() || before != *generation.borrow() {
                    return;
                }
                *generation.borrow_mut() += 1;
                busy.set(false);
                match result {
                    Ok(value) => {
                        crate::history::source::clear_web_history();
                        state.set(value)
                    }
                    Err(message) => state.set(AccountState {
                        last_error: Some(message),
                        ..(*state).clone()
                    }),
                }
            });
        })
    };
    let context = AccountContext {
        state: (*state).clone(),
        loaded: *loaded,
        busy: *busy,
        refresh,
        sign_in: action(Command::SignIn),
        sign_out: action(Command::SignOut),
    };
    html! {
        <ContextProvider<AccountContext> context={context}>
            { props.children.clone() }
        </ContextProvider<AccountContext>>
    }
}

#[component]
pub fn AccountWidget() -> Html {
    let account = use_context::<AccountContext>().expect("account provider");
    let retry = {
        let refresh = account.refresh.clone();
        Callback::from(move |_| refresh.emit(()))
    };
    let sign_in = {
        let action = account.sign_in.clone();
        Callback::from(move |_| action.emit(()))
    };
    let sign_out = {
        let action = account.sign_out.clone();
        Callback::from(move |_| action.emit(()))
    };
    html! {
        <section
            aria-label={"Account"}
            class={classes!("p-3","border-b","border-zinc-700","text-sm","flex","flex-wrap","items-center","gap-3","[&_button]:cursor-pointer","[&_a]:text-sky-400","[&_:focus-visible]:outline-2","[&_:focus-visible]:outline-sky-400","[&_button]:hover:text-sky-300","[&_button:disabled]:opacity-50")}
        >
            if let Some(email)=&account.state.email {
                <span>{ email }</span>
                <button disabled={account.busy} onclick={sign_out}>{ "Sign out" }</button>
            } else if account.state.signing_in {
                <span role={"status"}>
                    { "Complete sign in in your browser. The request expires after five minutes if you do not finish." }
                </span>
            } else if !account.loaded {
                <span role={"status"}>{ "Checking account…" }</span>
            } else if cfg!(feature="tauri") {
                <button disabled={account.busy} onclick={sign_in}>{ "Sign in with Google" }</button>
            } else {
                <a
                    href={format!("{}/auth/google/start?target=web&return_to=/history", crate::history::source::owner_api_base())}
                >
                    { "Sign in with Google" }
                </a>
            }
            if account.busy {
                <span role={"status"}>{ "Updating account…" }</span>
            }
            if let Some(error)=&account.state.last_error {
                <p role={"alert"}>
                    { error }
                    <button disabled={account.busy} onclick={retry}>{ "Retry" }</button>
                </p>
            }
        </section>
    }
}

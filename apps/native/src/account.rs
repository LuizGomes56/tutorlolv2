//! The account bar above every page: the web app's `AccountWidget`.
//!
//! "Sign in with Google" opens the browser on the server's Google login. The backend waits on a
//! loopback port for the callback (at most five minutes), redeems the ticket and stores the
//! session encrypted with DPAPI (`account.bin`). Signing out forgets it.
use crate::{
    components::ui::{self, Tone},
    session::Session,
    theme,
};
use gpui::{
    Context, IntoElement, ParentElement, Render, Styled, Subscription, Window, div, prelude::*, rgb,
};

pub struct AccountBar {
    _session: Subscription,
}

impl AccountBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            _session: cx.observe(&Session::global(cx), |_, _, cx| cx.notify()),
        }
    }
}

impl Render for AccountBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let session = Session::global(cx);
        let state = session.read(cx);
        let busy = state.account_busy;
        let account = state.account.clone();
        let available = state.start_error.is_none();

        let sign_in = {
            let session = session.clone();
            move |_: &gpui::ClickEvent, _: &mut Window, cx: &mut gpui::App| {
                session.update(cx, |session, cx| session.sign_in(cx))
            }
        };

        let sign_out = {
            let session = session.clone();
            move |_: &gpui::ClickEvent, _: &mut Window, cx: &mut gpui::App| {
                session.update(cx, |session, cx| session.sign_out(cx))
            }
        };

        let retry = {
            let session = session.clone();
            move |_: &gpui::ClickEvent, _: &mut Window, cx: &mut gpui::App| {
                session.update(cx, |session, cx| session.refresh_account(cx))
            }
        };

        let content = match &account {
            Some(account) if account.email.is_some() => div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(theme::STD_200))
                        .child(account.email.clone().unwrap_or_default()),
                )
                .child(ui::button(
                    "account-sign-out",
                    "Sign out",
                    Tone::Secondary,
                    busy,
                    false,
                    sign_out,
                )),
            Some(account) if account.signing_in => div().child(ui::status(
                "Complete sign in in your browser. The request expires after five minutes if \
                 you do not finish.",
                false,
            )),
            Some(_) => div().child(ui::button(
                "account-sign-in",
                "Sign in with Google",
                Tone::Primary,
                busy || !available,
                false,
                sign_in,
            )),
            None if available => div().child(ui::status("Checking account…", false)),
            None => div(),
        };

        let error = account
            .as_ref()
            .and_then(|account| account.last_error.clone());

        div()
            .flex()
            .flex_wrap()
            .items_center()
            .justify_end()
            .gap_3()
            .px_4()
            .py_2()
            .border_b_1()
            .border_color(rgb(theme::STD_800))
            .child(content)
            .when(busy, |bar| {
                bar.child(ui::status("Updating account…", false))
            })
            .children(error.map(|error| {
                ui::actions()
                    .child(ui::status(error, true))
                    .child(ui::button(
                        "account-retry",
                        "Retry",
                        Tone::Secondary,
                        busy,
                        false,
                        retry,
                    ))
            }))
    }
}

use crate::{assets::Assets, shell::Shell};
use gpui::{
    App, AppContext, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size,
};

mod account;
#[cfg(all(not(feature = "local-calculation"), feature = "api"))]
mod api;
mod assets;
mod backend;
mod calculator;
mod components;
mod formulas;
mod history;
mod livegame;
mod overlay;
mod random;
mod replay;
mod route;
mod session;
mod settings;
mod shell;
mod storage;
mod theme;
mod watch;

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        components::text_input::bind_keys(cx);
        session::Session::init(cx);

        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);

        let main = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Tutorlolv2".into()),
                        ..Default::default()
                    }),
                    app_id: Some("tutorlolv2".into()),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| Shell::new(window, cx)),
            )
            .expect("failed to open the main window");

        // The overlay window never closes by itself: closing the main window quits.
        cx.on_window_closed(|cx| cx.quit()).detach();

        // Showing the overlay window activates it: give the activation back to the main window.
        overlay::init(cx);
        cx.activate(true);
        let _ = main.update(cx, |_, window, _| window.activate_window());
    });
}

use crate::{assets::Assets, shell::Shell};
use gpui::{
    App, AppContext, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size,
};

mod assets;
mod calculator;
mod components;
mod formulas;
mod random;
mod route;
mod shell;
mod theme;

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        components::text_input::bind_keys(cx);

        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);

        cx.open_window(
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

        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        cx.activate(true);
    });
}

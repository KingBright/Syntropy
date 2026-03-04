mod app;
mod state;

use gpui::*;
use tracing::info;
use yororen_ui::assets::UiAsset;
use yororen_ui::component;
use yororen_ui::i18n::{I18n, Locale};
use yororen_ui::theme::GlobalTheme;

use crate::app::SyntropyApp;
use crate::state::AppState;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting Syntropy Desktop (GPUI / Yororen UI)");

    // Initialize application using Yororen UI Assets
    let app = Application::new().with_assets(UiAsset);

    app.run(|cx: &mut App| {
        // 1. Initialize Components & Theme
        component::init(cx);
        cx.set_global(GlobalTheme::new(cx.window_appearance()));
        cx.set_global(I18n::with_embedded(Locale::new("en").unwrap()));

        // 2. Initialize Global State
        cx.set_global(AppState::new());

        // 3. Configure and Open Main Window
        let options = WindowOptions {
            window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds::centered(
                None,
                size(px(800.0), px(600.0)),
                cx,
            ))),
            titlebar: Some(gpui::TitlebarOptions {
                title: None,
                appears_transparent: true,
                traffic_light_position: Some(gpui::point(px(9.0), px(9.0))),
            }),
            ..Default::default()
        };

        cx.open_window(options, |_, cx| cx.new(|cx| SyntropyApp::new(cx)))
            .unwrap();
    });
}

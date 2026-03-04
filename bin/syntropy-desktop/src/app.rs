use gpui::*;
use yororen_ui::component::{button, label};
use yororen_ui::theme::ActiveTheme;

use crate::state::AppState;

pub struct SyntropyApp;

impl SyntropyApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Register this view's entity ID in the global state so background tasks can notify it.
        let state = cx.global::<AppState>();
        *state.notify_entity.lock().unwrap() = Some(cx.entity().entity_id());
        Self
    }
}

impl Render for SyntropyApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let state = cx.global::<AppState>();

        let status_msg = state.status_message.lock().unwrap().clone();
        let is_processing = *state.is_processing.lock().unwrap();

        div()
            .flex()
            .flex_col()
            .bg(theme.surface.base)
            .size_full()
            .justify_center()
            .items_center()
            .gap(px(16.))
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.content.primary)
                    .child("Syntropy Agent"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(if is_processing {
                        theme.content.primary
                    } else {
                        theme.content.secondary
                    })
                    .child(format!("Status: {}", status_msg)),
            )
            .child(
                button("activate_agent")
                    .child("Run Agent Task")
                    .on_click(|_, _, cx| {
                        let id = {
                            let state = cx.global::<AppState>();
                            *state.is_processing.lock().unwrap() = true;
                            *state.status_message.lock().unwrap() =
                                "Processing background task...".to_string();
                            *state.notify_entity.lock().unwrap()
                        };

                        if let Some(entity_id) = id {
                            cx.notify(entity_id);
                        }

                        // TODO: Dispatch message to Kameo Actor here
                    }),
            )
    }
}

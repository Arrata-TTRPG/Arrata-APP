use dioxus::prelude::*;

use gloo_timers::future::TimeoutFuture;

use crate::components::popup::{POPUPS, PopupData, PopupType};

const POPUP_STYLE: Asset = asset!("/assets/popup.css");

#[component]
pub fn Popup(data: PopupData) -> Element {
    let onclick: EventHandler<MouseEvent> = use_hook(|| match data.kind {
        PopupType::ClickDismiss => EventHandler::new(move |_| {
            spawn(async move {
                fade_remove_popup(data.id).await;
            });
        }),
        PopupType::Timed(duration) => {
            let id = data.id;
            spawn(async move {
                TimeoutFuture::new(duration).await;
                fade_remove_popup(id).await;
            });
            EventHandler::new(|_| ())
        }
        PopupType::Signaled(waker) => {
            let id = data.id;
            spawn(async move {
                if let Err(e) = waker.wait().await {
                    tracing::error!("waker canceled: {e}");
                }
                fade_remove_popup(id).await;
            });
            EventHandler::new(|_| ())
        }
    });

    rsx! {
        div { class: "popup {data.class}", onclick, {data.children} }
    }
}

/// Applies the `popup-fading` class, waits
/// 500ms longer than the transition durations,
/// then deletes the popup.
///
/// # Clippy
/// Ignoring `clippy::future_not_send` here as
/// `gloo_timers::future::TimeoutFuture` is not
/// `Send`.
#[allow(clippy::future_not_send)]
async fn fade_remove_popup(id: u32) {
    if let Some(p) = POPUPS.write().iter_mut().find(|p| p.id == id) {
        p.class += " popup-fading";
    }
    TimeoutFuture::new(1550).await;
    POPUPS.write().retain(|p| p.id != id);
}

#[component]
pub fn PopupOverlay() -> Element {
    rsx! {
        document::Stylesheet { href: POPUP_STYLE }

        div { id: "popup-overlay",
            for popup in POPUPS() {
                Popup { key: "{popup.id}", data: popup }
            }
        }
    }
}

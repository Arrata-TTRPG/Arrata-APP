use dioxus::prelude::*;

pub mod view;
pub use view::{Popup, PopupOverlay};

pub static POPUPS: GlobalSignal<Vec<PopupData>> = GlobalSignal::new(Vec::new);

/// What properties a `Popup` has.
#[derive(Clone)]
pub enum PopupType {
    /// Persistent - dismissed only after clicking it.
    ClickDismiss,
    /// Persistent only until the timer runs out.
    Timed(u32),
    /// Persistent - waiting for a signal to dismiss it.
    Signaled(UseWaker<()>),
}

/// Helper struct for managing popups.
#[derive(Clone)]
pub struct PopupData {
    /// `Children` that are inserted inside of the popup.
    pub children: Element,
    /// The `class` attribute for the popup.
    pub class: String,
    /// Identifier value for the popup, used to remove it from the `POPUPS` global signal.
    pub id: u32,
    /// The type of popup, either `ClickDismiss` or `Timed(u32)`.
    pub kind: PopupType,
}

/// Mandatory impl to pass `PopupData` into a `#[component]`.
impl PartialEq for PopupData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.class == other.class && self.children == other.children
    }
}

/// Writes a new `PopupData` to the `POPUPS` global signal.
/// The `id` is generated from the current timestamp in milliseconds.
pub fn add_popup(class: String, children: Element, kind: PopupType) {
    let id = getrandom::u32().unwrap_or_else(|_| {
        tracing::error!("Failed to generate random id for popup");
        0
    });
    let popup = PopupData {
        children,
        class,
        id,
        kind,
    };
    POPUPS.write().push(popup);
}

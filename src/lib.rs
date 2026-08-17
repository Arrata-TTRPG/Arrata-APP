#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::perf,
    clippy::correctness,
    clippy::style,
    clippy::suspicious,
    clippy::nursery
)]
#![deny(warnings)]
#![allow(non_snake_case)]
// Nothing here runs off the main thread: this is a single-threaded wasm app and
// `document::eval` futures are inherently `!Send`.
#![allow(clippy::future_not_send)]
// Fired from inside `asset!`, not from anything we write.
#![allow(clippy::volatile_composites)]

//! Arrata character sheet manager.
//!
//! All state lives in two stores installed as context by [`App`]:
//!
//! * [`Roster`] — the characters the user owns and which one is on screen.
//!   Persisted to local storage.
//! * [`Ui`] — transient view state (open panels, pending dice roll). Deliberately
//!   not persisted.
//!
//! Components take the narrowest store they need as a prop, so editing one
//! field re-renders only the subtree that reads it.

pub mod components;

use dioxus::prelude::*;

use arrata_lib::{Character, Quirk, Stat};

pub use components::entrypoint::App;

/// Shown next to the title.
pub(crate) const VERSION: &str = "v0.0.0";

/// Where the community quirk lists are published.
const QUIRK_REPO: &str = "https://raw.githubusercontent.com/Arrata-TTRPG/Arrata-Quirks/main/";

/// Every character the user owns, plus the index of the one being edited.
#[derive(Store, Clone, PartialEq, Default)]
pub struct Roster {
    /// The saved characters, in sidebar order.
    pub characters: Vec<Character>,
    /// Index into `characters`. Always in range while the roster is non-empty.
    pub active: usize,
}

/// View state that intentionally does not survive a reload.
#[derive(Store, Clone, PartialEq, Default)]
pub struct Ui {
    /// The stat the dice roller is open for, if any.
    pub roll: Option<Stat>,
    /// Whether the roster sidebar is expanded.
    pub sidebar: bool,
    /// Whether the premade-quirk browser is open.
    pub browser: bool,
    /// Quirks fetched from [`QUIRK_REPO`], plus any the user saved or imported.
    pub premade: Vec<Quirk>,
}

/// A store scoped to the character currently being edited.
///
/// `None` only when the roster is empty, which the sidebar prevents.
pub(crate) fn use_active_character() -> Option<Store<Character>> {
    let roster = use_context::<Store<Roster>>();
    roster.characters().get(roster.active()()).map(Into::into)
}

/// Fetches the community quirk lists into `ui`.
///
/// Categories that fail to download or decode are skipped: premade quirks are a
/// convenience, and a network failure must not block editing a sheet.
pub(crate) async fn load_premade_quirks(ui: Store<Ui>) {
    let mut fetched = Vec::new();

    for category in ["ethos", "pathos", "logos"] {
        let Ok(response) = reqwest::get(format!("{QUIRK_REPO}{category}.quirks")).await else {
            continue;
        };
        let Ok(bytes) = response.bytes().await else {
            continue;
        };
        if let Ok(quirks) = bitcode::decode::<Vec<Quirk>>(&bytes) {
            fetched.extend(quirks);
        }
    }

    let mut premade = ui.premade();
    premade.write().extend(fetched);
    premade.write().sort_by(|a, b| a.name.cmp(&b.name));
    premade.write().dedup();
}

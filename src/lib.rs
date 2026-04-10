#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::perf,
    clippy::correctness,
    clippy::style,
    clippy::suspicious
)]
#![deny(warnings)]

pub mod render;

#[cfg(any(feature = "desktop", feature = "web"))]
pub mod storage;

use arrata_lib::{
    Quirk,
    character::{Character, Stat},
};

use dioxus::prelude::GlobalSignal;
use js_sys::Uint8Array;
use semver::Version;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// The active character, kept in sync with `CHARACTERS[ACTIVE_IDX]`.
/// All render components read/write this; switching characters updates it from the vec.
pub static CHARACTER: GlobalSignal<Character> = GlobalSignal::new(Character::default);
/// All characters in the roster.
pub static CHARACTERS: GlobalSignal<Vec<Character>> =
    GlobalSignal::new(|| vec![Character::default()]);
/// Index of the currently displayed character in `CHARACTERS`.
pub static ACTIVE_IDX: GlobalSignal<usize> = GlobalSignal::new(|| 0);
/// Whether the character sidebar is open.
pub(crate) static SIDEBAR_OPEN: GlobalSignal<bool> = GlobalSignal::new(|| false);
/// The `GlobalSignal` for rolling dice.
pub(crate) static DICE_ROLL_STATE: GlobalSignal<(bool, Option<Stat>)> =
    GlobalSignal::new(|| (false, None));
/// The `GlobalSignal` for the current version of the app.
pub(crate) static VERSION: GlobalSignal<Version> =
    GlobalSignal::new(|| Version::parse("v0.0.0".strip_prefix("v").unwrap()).unwrap());
/// The `GlobalSignal` for rendering the pre-made Quirks menu
pub(crate) static PREMADE_QUIRKS_MENU: GlobalSignal<bool> = GlobalSignal::new(|| false);
/// The `GlobalSignal` for the stored pre-made Quirks
pub(crate) static PREMADE_QUIRKS: GlobalSignal<Vec<Quirk>> = GlobalSignal::new(Vec::new);

/// The `GlobalSignal` for which category of `Quirks` to display.
/// The tuple is in the order of ethos, pathos, logos.
pub(crate) static SHOWN_CATEGORIES: GlobalSignal<(bool, bool, bool)> =
    GlobalSignal::new(|| (false, false, false));

/// Loads the initial pre-made quirks from the `Arrata-Quirks` GitHub repository.
pub(crate) async fn load_initial_quirks() {
    let url = "https://raw.githubusercontent.com/Arrata-TTRPG/Arrata-Quirks/main/";
    let categories = ["ethos", "pathos", "logos"];

    let window = web_sys::window().unwrap();

    for category in categories {
        let full_url = format!("{url}{category}.quirks");

        let Ok(resp) = JsFuture::from(window.fetch_with_str(&full_url)).await else {
            continue;
        };

        let resp: web_sys::Response = resp.dyn_into().unwrap();
        if !resp.ok() {
            continue;
        }

        let Ok(buf) = JsFuture::from(resp.array_buffer().unwrap()).await else {
            continue;
        };

        let bytes = Uint8Array::new(&buf).to_vec();
        let quirks: Vec<Quirk> = bitcode::decode(&bytes).unwrap();
        PREMADE_QUIRKS.write().extend(quirks);
    }

    PREMADE_QUIRKS.write().sort_by(|a, b| a.name.cmp(&b.name));
    PREMADE_QUIRKS.write().dedup();
}

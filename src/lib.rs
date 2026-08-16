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

pub mod components;

use arrata_lib::{
    Quirk,
    character::{Character, Stat},
};

use dioxus::prelude::*;

/// The `GlobalSignal` for rolling dice.
pub(crate) static DICE_ROLL_STATE: GlobalSignal<(bool, Option<Stat>)> =
    GlobalSignal::new(|| (false, None));
/// The `GlobalSignal` for the current version of the app.
pub(crate) static VERSION: &str = "v0.0.0";
/// The `GlobalSignal` for the stored pre-made Quirks
pub(crate) static PREMADE_QUIRKS: GlobalSignal<Vec<Store<Quirk>>> = GlobalSignal::new(Vec::new);

/// The `GlobalSignal` for which category of `Quirks` to display.
/// The tuple is in the order of ethos, pathos, logos.
pub(crate) static SHOWN_CATEGORIES: GlobalSignal<(bool, bool, bool)> =
    GlobalSignal::new(|| (false, false, false));

#[derive(Store, Clone, PartialEq, Default)]
pub struct Roster {
    pub characters: Vec<Character>,
    pub active: usize,
}

/// Loads the initial pre-made quirks from the `Arrata-Quirks` GitHub repository.
pub(crate) async fn load_initial_quirks() {
    let url = "https://raw.githubusercontent.com/Arrata-TTRPG/Arrata-Quirks/main/";
    let categories = ["ethos", "pathos", "logos"];

    for category in categories {
        let full_url = format!("{url}{category}.quirks");

        let Ok(resp) = reqwest::get(full_url).await else {
            continue;
        };

        let bytes = match resp.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => continue,
        };

        let mut quirks: Vec<Quirk> = bitcode::decode(&bytes.as_ref()).unwrap();
        quirks.dedup();
        let stored_quirks: Vec<Store<Quirk>> =
            quirks.into_iter().map(|q| use_store(|| q)).collect();
        PREMADE_QUIRKS.write().extend(stored_quirks);
    }
}

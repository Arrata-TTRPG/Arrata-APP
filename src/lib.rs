#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::perf,
    clippy::correctness,
    clippy::style,
    clippy::suspicious
)]

pub mod render;

#[cfg(any(feature = "desktop", feature = "web"))]
pub mod storage;

use arrata_lib::{
    character::{Character, Stat},
    Quirk,
};

use dioxus::prelude::GlobalSignal;
use reqwest::Client;
use semver::Version;

/// The `GlobalSignal` for the `Character`.
pub static CHARACTER: GlobalSignal<Character> = GlobalSignal::new(Character::default);
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
    GlobalSignal::new(|| (true, true, true));

/// Loads the initial pre-made quirks from the `Arrata-Quirks` GitHub repository.
pub(crate) async fn load_initial_quirks() {
    log::info!("Loading initial pre-made quirks");

    let url = "https://raw.githubusercontent.com/Arrata-TTRPG/Arrata-Quirks/main/";

    let categories = ["ethos", "pathos", "logos"];

    let client = Client::new();

    for category in categories {
        let file = format!("{category}.quirks");

        let request = client.get(format!("{url}{file}")).build().unwrap();

        let response = client.execute(request).await;

        if let Ok(response) = response {
            let quirks: Vec<Quirk> = bitcode::decode(&response.bytes().await.unwrap()).unwrap();
            PREMADE_QUIRKS.write().extend(quirks);
            // Add a sort and dedup to the pre-made quirks
            PREMADE_QUIRKS.write().sort_by(|a, b| a.name.cmp(&b.name));
            PREMADE_QUIRKS.write().dedup();
        } else {
            log::error!("Failed to load pre-made quirks from {url}{file}");
        }
    }
}

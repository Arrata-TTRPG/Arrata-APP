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
pub static CHARACTER: GlobalSignal<Character> = GlobalSignal::new(Character::new);
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

pub(crate) async fn load_initial_quirks() {
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
        } else {
            log::error!("Failed to load pre-made quirks from {url}{file}");
        }
    }
}

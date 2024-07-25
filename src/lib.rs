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

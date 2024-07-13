#![warn(clippy::all, clippy::pedantic)]

pub mod render;
pub mod storage;

use arrata_lib::character::{Character, Stat};

use dioxus::prelude::GlobalSignal;
use semver::Version;

/// The `GlobalSignal` for the `Character`.
pub static CHARACTER: GlobalSignal<Character> = GlobalSignal::new(Character::new);
/// The `GlobalSignal` for rolling dice.
pub(crate) static DICE_ROLL_STATE: GlobalSignal<(bool, Option<Stat>)> =
    GlobalSignal::new(|| (false, None));
/// The `GlobalSignal` for the current version of the app.
pub(crate) static VERSION: GlobalSignal<Version> = GlobalSignal::new(|| Version::new(0, 0, 0));

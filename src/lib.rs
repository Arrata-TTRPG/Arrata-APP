#![warn(clippy::all, clippy::pedantic)]

pub mod render;

use arrata_lib::character::{Character, Stat};

use dioxus::prelude::GlobalSignal;

/// The `GlobalSignal` for the `Character`.
pub(crate) static CHARACTER: GlobalSignal<Character> = GlobalSignal::new(Character::new);
/// The `GlobalSignal` for rolling dice.
pub(crate) static DICE_ROLL_STATE: GlobalSignal<(bool, Option<Stat>)> =
    GlobalSignal::new(|| (false, None));

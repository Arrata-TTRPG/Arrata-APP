#![warn(clippy::all, clippy::pedantic)]

pub mod character;
pub mod dice;

#[cfg(any(feature = "web", feature = "desktop"))]
pub mod render;
#[cfg(any(feature = "web", feature = "desktop"))]
use character::{Character, Stat};
#[cfg(any(feature = "web", feature = "desktop"))]
use dioxus::prelude::GlobalSignal;
#[cfg(any(feature = "web", feature = "desktop"))]
/// The `GlobalSignal` for the `Character`.
pub(crate) static CHARACTER: GlobalSignal<Character> = GlobalSignal::new(Character::new);
#[cfg(any(feature = "web", feature = "desktop"))]
/// The `GlobalSignal` for rolling dice.
pub(crate) static DICE_ROLL_STATE: GlobalSignal<(bool, Option<Stat>)> =
    GlobalSignal::new(|| (false, None));

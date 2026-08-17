//! The character sheet: identity header, import/export, and the three columns.

use dioxus::prelude::*;

use arrata_lib::{Character, CharacterStoreExt};

use crate::components::{
    combat::RenderCombat,
    io::{download_character, pick_character},
    quirks::RenderQuirks,
    shared::{Btn, BtnKind, Confirm, Field, Row, TextInput},
    stats::RenderStats,
};

/// Name and stock, then the three sheet columns.
#[component]
pub fn RenderCharacter(character: Store<Character>) -> Element {
    rsx! {
        Row { class: "fill",
            Field { label: "Name:",
                TextInput { value: character.name() }
            }
            Field { label: "Stock:",
                TextInput { value: character.stock() }
            }
        }

        div { class: "sheet",
            RenderStats { character }
            RenderQuirks { character }
            RenderCombat { character }
        }
    }
}

/// Export the active character, or overwrite it from a `.arrata` file.
#[component]
pub fn CharacterIO(mut character: Store<Character>) -> Element {
    let mut incoming = use_signal::<Option<Character>>(|| None);

    rsx! {
        div { class: "io",
            if let Some(replacement) = incoming() {
                Confirm {
                    prompt: "Overwrite \"{character.name()}\" with \"{replacement.name}\"?",
                    on_yes: move |_| {
                        character.set(replacement.clone());
                        incoming.set(None);
                    },
                    on_no: move |_| incoming.set(None),
                }
            } else {
                Btn {
                    kind: BtnKind::Large,
                    onclick: move |_| download_character(&character()),
                    "Export Character"
                }
                Btn {
                    kind: BtnKind::Large,
                    onclick: move |_| async move { incoming.set(pick_character().await) },
                    "Import Overwrite"
                }
            }
        }
    }
}

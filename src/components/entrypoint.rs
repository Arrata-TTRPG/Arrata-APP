//! Application root: installs state, loads stylesheets, lays out the shell.

use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;

use arrata_lib::Character;

use crate::components::{
    character::{CharacterIO, RenderCharacter},
    popup::PopupOverlay,
    premade_quirks::QuirkBrowser,
    rolls::RollDialog,
    sidebar::{Sidebar, SidebarToggle},
};
use crate::{Roster, RosterStoreExt, Ui, UiStoreExt, VERSION, load_premade_quirks};

const BASE_CSS: Asset = asset!("/assets/base.css");
const LAYOUT_CSS: Asset = asset!("/assets/layout.css");
const CONTROLS_CSS: Asset = asset!("/assets/controls.css");
const SHEET_CSS: Asset = asset!("/assets/sheet.css");
const RAT: Asset = asset!("/public/rat_release.svg");

/// The application.
///
/// Owns the two context stores and the local-storage mirror; everything below
/// reads them from context or from props.
#[component]
pub fn App() -> Element {
    let mut persisted = use_persistent("characters", || vec![Character::default()]);

    let roster = use_context_provider(|| {
        Store::new(Roster {
            characters: persisted(),
            active: 0,
        })
    });
    let ui = use_context_provider(|| Store::new(Ui::default()));

    // Mirror the roster back to local storage whenever any character changes.
    use_effect(move || persisted.set(roster.characters()()));
    use_future(move || load_premade_quirks(ui));

    rsx! {
        document::Stylesheet { href: BASE_CSS }
        document::Stylesheet { href: LAYOUT_CSS }
        document::Stylesheet { href: CONTROLS_CSS }
        document::Stylesheet { href: SHEET_CSS }

        Sheet {}
        PopupOverlay {}
    }
}

/// Sidebar plus the scrolling sheet for the active character.
#[component]
fn Sheet() -> Element {
    let ui = use_context::<Store<Ui>>();
    let Some(character) = crate::use_active_character() else {
        return rsx! {};
    };

    rsx! {
        div { class: "app",
            Sidebar {}
            SidebarToggle {}
            main { class: "app__main",
                Masthead {}
                CharacterIO { character }
                RenderCharacter { character }
            }
        }

        if ui.roll().is_some() {
            RollDialog {}
        }
        if ui.browser()() {
            QuirkBrowser {}
        }
    }
}

/// Logo, wordmark and version.
#[component]
fn Masthead() -> Element {
    rsx! {
        header { class: "masthead",
            object { class: "masthead__logo", r#type: "image/svg+xml",
                img { src: RAT }
            }
            div { class: "masthead__wordmark",
                h1 { "ARRATA" }
                span { class: "masthead__version", {VERSION} }
            }
        }
    }
}

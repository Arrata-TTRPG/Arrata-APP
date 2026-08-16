use dioxus::prelude::*;

use dioxus_sdk::storage::use_persistent;

use arrata_lib::Character;

use crate::{
    DICE_ROLL_STATE, Roster, RosterStoreExt, VERSION,
    components::{
        CharacterIO, CharacterSidebar, RenderCharacter, RenderRolls, SidebarToggle,
        popup::PopupOverlay,
    },
    load_initial_quirks,
};

const TAILWIND_CSS: Asset = asset!("public/tailwind.css");
const RAT_RELEASE: Asset = asset!("public/rat_release.svg");

/// The main application.
#[component]
pub fn App() -> Element {
    use_future(move || async {
        load_initial_quirks().await;
    });

    let mut persisted =
        use_persistent::<Vec<Character>>("characters", || vec![Character::default()]);

    let roster = use_store(|| Roster {
        characters: persisted(), // read once, store owns the value
        active: 0,
    });

    use_effect(move || {
        persisted.set(roster.characters()());
    });

    use_context_provider(|| roster);

    rsx! {
        Sheet {}
    }
}

#[component]
fn Sheet() -> Element {
    let char_store = use_context::<Store<Roster>>();
    let active = char_store.active()();
    let Some(character) = char_store.characters().get(active) else {
        return rsx! {};
    };

    rsx! {
        Stylesheet { href: TAILWIND_CSS }

        // Full-screen flex row: sidebar + main content
        div { class: "w-screen h-screen flex flex-row overflow-hidden",

            CharacterSidebar {}
            SidebarToggle {}

            // Main content — scrollable column
            div { class: "flex flex-col flex-grow overflow-y-auto",

                // Header
                div { class: "px-5 py-2 origin-center justify-center items-middle flex flex-wrap h-fit max-w-full gap-4",
                    object {
                        class: "object-fill med:w-[9rem] med:h-[9rem] sm:w-[6rem] sm:h-[6rem] w-[4.5rem] h-[4.5rem] pt-2",
                        r#type: "image/svg+xml",
                        img { class: "object-fit", src: RAT_RELEASE }
                    }
                    div { class: "flex flex-row items-baseline",
                        h1 { class: "text-center md:text-9xl sm:text-8xl text-7xl font-mono font-extrabold align-bottom",
                            "ARRATA"
                        }
                        span { class: "h-full font-bold align-bottom pl-2 lg:text-base md:text-sm text-xs",
                            {VERSION}
                        }
                    }
                }

                CharacterIO {}
                RenderCharacter { character: character }

                if DICE_ROLL_STATE().0 {
                    if let Some(_) = DICE_ROLL_STATE().1 {
                        RenderRolls {}
                    }
                }
            }
        }

        PopupOverlay {}
    }
}

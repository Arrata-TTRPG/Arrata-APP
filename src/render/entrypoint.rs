use dioxus::prelude::*;

use crate::{
    DICE_ROLL_STATE, VERSION, load_initial_quirks,
    render::{CharacterIO, CharacterSidebar, RenderCharacter, RenderRolls, SidebarToggle},
};

const TAILWIND_CSS: Asset = asset!("public/tailwind.css");
const RAT_RELEASE: Asset = asset!("public/rat_release.svg");

/// The main application.
#[component]
pub fn App() -> Element {
    use_future(move || async {
        load_initial_quirks().await;
    });

    #[cfg(any(feature = "web", feature = "desktop"))]
    {
        use crate::{
            ACTIVE_IDX, CHARACTER, CHARACTERS, PREMADE_QUIRKS,
            storage::{read_characters, read_quirks, write_characters, write_quirks},
        };

        use_future(|| async {
            // Load roster; fall back to a single default character
            if let Some(characters) = read_characters() {
                *CHARACTERS.write() = characters;
            }
            let first = CHARACTERS().into_iter().next().unwrap_or_default();
            *CHARACTER.write() = first;
            *ACTIVE_IDX.write() = 0;

            // Persist roster whenever CHARACTER changes (syncs active slot then saves all)
            use_effect(move || {
                let character = CHARACTER();
                let idx = ACTIVE_IDX();
                let mut chars = CHARACTERS.write();
                if let Some(slot) = chars.get_mut(idx) {
                    *slot = character;
                }
                write_characters(&chars);
            });

            // Pre-made quirks
            let quirks_key = format!("quirks-{}-{}", VERSION().major, VERSION().minor);
            if let Some(quirks) = read_quirks(&quirks_key) {
                PREMADE_QUIRKS.write().extend(quirks);
            }
            use_effect(move || {
                let quirks = PREMADE_QUIRKS();
                write_quirks(&quirks, &quirks_key);
            });
        });
    }

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
                            "v{VERSION()}"
                        }
                    }
                }

                CharacterIO {}
                RenderCharacter {}

                if DICE_ROLL_STATE().0 {
                    if let Some(_) = DICE_ROLL_STATE().1 {
                        RenderRolls {}
                    }
                }
            }
        }
    }
}

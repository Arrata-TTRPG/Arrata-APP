use dioxus::prelude::*;

use crate::{
    render::{CharacterIO, RenderCharacter, RenderRolls},
    DICE_ROLL_STATE, VERSION,
};

/// The main application.
#[component]
pub fn App() -> Element {
    let arrata_style = r"
    body { background-color: black; color: white; }
    input { background-color: black; color: white; }
    select { background-color: black; color: white; }
    option { background-color: black; color: white; }
    ";

    #[cfg(any(feature = "web", feature = "desktop"))]
    {
        use crate::{
            storage::{read_character, read_quirks, write_character, write_quirks},
            CHARACTER, PREMADE_QUIRKS,
        };

        use_future(|| async {
            // The key used for storage naming
            let key = format!("temp-{}-{}", VERSION().major, VERSION().minor);
            // Get the character from storage
            if let Some(character) = read_character(key.as_str()) {
                *CHARACTER.write() = character;
            }
            // Save the character on edit
            use_effect(move || {
                let character = CHARACTER();
                write_character(key.as_str(), &character);
            });

            let key = format!("quirks-{}-{}", VERSION().major, VERSION().minor);
            // Get pre-made Quirks
            if let Some(quirks) = read_quirks(key.as_str()) {
                PREMADE_QUIRKS.write().extend(quirks);
            }
            // Save the pre-made Quirks on edit
            use_effect(move || {
                let quirks = PREMADE_QUIRKS();
                write_quirks(quirks, key.as_str());
            });
        });
    }

    rsx! {
        style { "{arrata_style}" }
        link { rel: "stylesheet", href: "tailwind.css" }

        div { class: "px-5 py-2 origin-center justify-center items-middle flex flex-wrap h-fit max-w-full gap-4",
            // Arrata logo
            object {
                class: "object-fill med:w-[9rem] med:h-[9rem] sm:w-[6rem] sm:h-[6rem] w-[4.5rem] h-[4.5rem]",
                data: "rat_release.svg",
                r#type: "image/svg+xml",
                img { class: "object-fit", src: "rat_release.svg" }
            }

            // Title and version
            div { class: "flex flex-row items-baseline",
                h1 { class: "text-center md:text-9xl sm:text-8xl text-7xl font-mono font-extrabold align-bottom",
                    "ARRATA"
                }

                p { class: "h-full font-mono align-bottom ml-5 lg:text-base md:text-sm text-xs",
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

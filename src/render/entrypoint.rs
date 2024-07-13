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

    #[cfg(feature = "desktop")]
    {
        use crate::{
            storage::desktop::{read_character, write_character},
            CHARACTER,
        };

        use_future(|| async {
            if let Some(character) = read_character("temp") {
                *CHARACTER.write() = character;
                log::info!("Character loaded: {}", CHARACTER().name);
            }

            use_effect(move || {
                log::info!("Effect moment");
                let _ = CHARACTER();
                write_character("temp");
            });
        });
    }

    #[cfg(feature = "web")]
    {
        use crate::{
            storage::web::{read_character, write_character},
            CHARACTER,
        };

        use_future(|| async {
            log::info!("Future moment");
            if let Some(character) = read_character("temp").await {
                *CHARACTER.write() = character;
                log::info!("Character loaded: {}", CHARACTER().name);
            }

            use_effect(move || {
                log::info!("Effect moment");
                let _ = CHARACTER();
                spawn(async {
                    write_character("temp").await;
                });
            });
        });
    }

    rsx! {
        style { "{arrata_style}" }
        link { rel: "stylesheet", href: "tailwind.css"}

        div { class: "px-5 py-2 origin-center justify-center items-middle flex flex-wrap h-fit max-w-full space-x-4",
            // Arrata logo
            img {
                // Arrata logo
                class: "object-scale-down med:w-[8rem] med:h-[8rem] sm:w-[6rem] sm:h-[6rem] w-[4.5rem] h-[4.5rem]",
                src: "rat.png",
            }

            // Title and version
            div { class: "flex flex-row items-baseline",
                h1 { class: "text-center md:text-9xl sm:text-8xl text-7xl font-mono font-extrabold align-bottom",
                    "ARRATA"
                }

                p { class: "h-full font-mono align-bottom ml-5 lg:text-base md:text-sm text-xs", "{VERSION()}" }
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

use dioxus::prelude::*;

use crate::{
    render::{CharacterIO, RenderCharacter, RenderRolls},
    DICE_ROLL_STATE,
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

    rsx! {
        style { "{arrata_style}" }
        link { rel: "stylesheet", href: "tailwind.css" }

        div { class: "px-5 py-5 origin-center justify-center items-baseline flex",
            // Arrata logo
            img {
                // Arrata logo
                class: "h-fit mr-10",
                src: "rat.png",
                alt: "",
                width: 100,
                height: 100
            }

            h1 { class: "text-center text-9xl font-mono font-extrabold align-bottom",
                "ARRATA"
            }

            p { class: "h-full font-mono align-bottom ml-5", "VERSION" }
        }

        br {}

        CharacterIO {}

        br {}

        RenderCharacter {}

        if DICE_ROLL_STATE().0 {
            if let Some(_) = DICE_ROLL_STATE().1 {
                RenderRolls {}
            }
        }
    }
}

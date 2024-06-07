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

    let rat_path = if cfg!(target_family = "wasm") {
        "rat.png"
    } else {
        "public/rat.png"
    };

    rsx! {
        style { "{arrata_style}" }
        link { rel: "stylesheet", href: "tailwind.css" }

        div { class: "px-5 py-5 origin-center justify-center self-center items-center content-center flex",
            // Arrata logo
            img {
                // Arrata logo
                class: "w-24 h-24 md:w-28 md:h-auto md:rounded-none rounded-full mr-10",
                src: rat_path,
                alt: "",
                width: 300,
                height: 300
            }

            h1 { class: "text-center text-9xl font-mono font-extrabold", "ARRATA" }
        }

        br {}

        CharacterIO {}

        br {}

        RenderCharacter {}

        if DICE_ROLL_STATE().0 {
            match &DICE_ROLL_STATE().1 {
                Some(_) => rsx!(RenderRolls {}),
                None    => rsx!(""),
            }
        }
    }
}

mod character;
use character::*;

use dioxus::prelude::*;

/// The main application.
pub fn app(cx: Scope) -> Element {
    let character: &UseState<Character> = use_state(cx, Character::new);

    cx.render(rsx! {
        div {
            class: "focus:outline-none focus:ring  aspect auto content-center",

            // Arrata logo
            img {
                class: "w-24 h-24 md:w-48 md:h-auto md:rounded-none rounded-full mx-auto",
                src: "public/rat.png",
                alt: "",
                width: 315,
                height: 315,
            }
        }      

        h1 {
            class: "text-center",
            "ARRATA"
        }

        RenderCharacter {
            character: character
        }

        div {
            button {
                onclick: move |_| character.get().write_to_file().unwrap(),
                "Save Character"
            },
            button {
                onclick: move |_| {
                    character.set(Character::from_file().unwrap());
                },
                "Load Character"
            },
        }
    })
}

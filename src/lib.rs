mod character;
use character::*;

use dioxus::prelude::*;

/// The main application.
pub fn app(cx: Scope) -> Element {
    let character: &UseState<Character> = use_state(cx, Character::new);

    let arrata_style = r#"
    body { background-color: black; color: white; }
    input { background-color: black; color: white; }
    select { background-color: black; color: white; }
    "#;

    cx.render(rsx! {
        style { arrata_style }

        div {
            class: "top-5 bottom-5 origin-center justify-center self-center items-center content-center flex",
            // Arrata logo
            img {
                class: " slatew-24 h-24 md:w-28 md:h-auto md:rounded-none rounded-full mr-10",
                src: "public/rat.png",
                alt: "",
                width: 300,
                height: 300,
            }

            h1 {
                class: "text-center text-9xl font-mono font-extrabold",
                "ARRATA"
            }
        }

        br {}

        render_character {
            character: character
        }

        br {}

        div {
            button {
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| character.get().write_to_file().unwrap(),
                "Save Character"
            },
            button {
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| character.set(Character::from_file().unwrap()),
                "Load Character"
            },
        }
    })
}

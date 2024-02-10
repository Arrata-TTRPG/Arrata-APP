pub mod character;
pub mod dice;

use character::render::*;
use character::structs::*;
use dice::render::*;

use dioxus::prelude::*;

/// The main application.
pub fn app(cx: Scope) -> Element {
    let character: &UseRef<Character> = use_ref(cx, Character::new);

    let dice_roll_state: &UseState<(bool, Option<Stat>)> = use_state(cx, || (false, None));

    let arrata_style = r#"
    body { background-color: black; color: white; }
    input { background-color: black; color: white; }
    select { background-color: black; color: white; }
    "#;

    let rat_path = if cfg!(target_family = "wasm") {
        "rat.png"
    } else {
        "public/rat.png"
    };

    cx.render(rsx! {
        style { arrata_style }

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

        character_io { character: character }

        br {}

        render_character { character: character, dice_roll_state: dice_roll_state }

        if dice_roll_state.0 {
            match &dice_roll_state.1 {
                Some(_) => rsx!(render_rolls { state: dice_roll_state }),
                None    => rsx!(""),
            }
        }
    })
}

#[component(no_case_check)]
fn character_io<'a>(cx: Scope<'a>, character: &'a UseRef<Character>) -> Element<'a> {
    cx.render(rsx!{
        div { class: "px-5 py-5 font-mono origin-center justify-center text-center self-center items-center content-center flex space-x-3",
            if cfg!(not(target_family = "wasm")) {
                rsx! {
                    button {
                        class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| character.read().write_to_file().unwrap(),
                        "Save Character"
                    }
                    button {
                        class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| {
                            let new_character = Character::from_file();
                            match new_character {
                                Ok(c) => character.set(c),
                                Err(e) => match e.kind() {
                                    std::io::ErrorKind::Other => (),
                                    _ => panic!("{:?}", e),
                                }
                            }
                        },
                        "Load Character"
                    }
                }
            } else {
                rsx! {
                    "Character Saving/Loading is disabled for this platform."
                }
            }
        }
    })
}

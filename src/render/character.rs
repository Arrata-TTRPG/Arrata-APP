use dioxus::prelude::*;

use crate::{character::Character, render::{RenderInventory, RenderQuirks, RenderStats}, CHARACTER};

#[component]
pub(crate) fn RenderCharacter() -> Element {
    rsx! {
        div { class: "flex content-center items-center justify-center",
            div { class: "font-mono text-xl px-2 py-2", "Name:" }
            input {
                class: "border-spacing-1 border rounded-lg px-2 py-2",
                value: "{CHARACTER().name}",
                oninput: move |evt| {
                    CHARACTER.write().name = evt.value();
                }
            }
            div { class: "font-mono text-xl px-2 py-2", "Stock:" }
            input {
                class: "border-spacing-1 border rounded-lg px-2 py-2",
                value: "{CHARACTER().stock}",
                oninput: move |evt| {
                    CHARACTER.write().stock = evt.value();
                }
            }
        }

        div { class: "flex flex-wrap",
            RenderStats {}
            RenderQuirks {}
            RenderInventory {}
        }
    }
}

#[component]
pub(crate) fn CharacterIO() -> Element {
    rsx! {
        div { class: "px-5 py-5 font-mono origin-center justify-center text-center self-center items-center content-center flex space-x-3",
            if cfg!(feature = "desktop") {
                button {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| CHARACTER().write_to_file().unwrap(),
                    "Save Character"
                }
                button {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| {
                        let new_character = Character::from_file();
                        match new_character {
                            Ok(c) => *CHARACTER.write() = c,
                            Err(e) => match e.kind() {
                                std::io::ErrorKind::Other => (),
                                _ => panic!("{e:?}"),
                            }
                        }
                    },
                    "Load Character"
                }
            } else {
                "Character Saving/Loading is disabled for this platform."
            }
        }
    }
}

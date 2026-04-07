use arrata_lib::Character;
use dioxus::prelude::*;

use crate::{
    CHARACTER,
    render::{RenderCombat, RenderQuirks, RenderStats, download_character, pick_character_file},
};

#[component]
pub(crate) fn RenderCharacter() -> Element {
    rsx! {
        div { class: "flex flex-wrap sm:flex-row flex-col content-center items-center justify-center gap-2 px-2",
            div { class: "flex sm:w-fit w-full",
                h2 { class: "font-mono text-xl p-2", "Name:" }
                input {
                    class: "border-spacing-1 border rounded-lg p-2 text-center sm:flex-grow-0 flex-grow",
                    value: "{CHARACTER().name}",
                    oninput: move |evt| CHARACTER.write().name = evt.value(),
                }
            }
            div { class: "flex sm:w-fit w-full",
                h2 { class: "font-mono text-xl p-2", "Stock:" }
                input {
                    class: "border-spacing-1 border rounded-lg p-2 text-center sm:flex-grow-0 flex-grow",
                    value: "{CHARACTER().stock}",
                    oninput: move |evt| CHARACTER.write().stock = evt.value(),
                }
            }
        }

        div { class: "flex flex-grow flex-wrap pt-4 pb-4 items-start",
            RenderStats {}
            RenderQuirks {}
            RenderCombat {}
        }
    }
}

#[component]
pub(crate) fn CharacterIO() -> Element {
    // `Some(character)` while awaiting overwrite confirmation
    let mut pending_import: Signal<Option<Character>> = use_signal(|| None);

    rsx! {
        div { class: "w-full flex justify-center",
            div { class: "px-5 pb-5 font-mono origin-center w-fit max-w-[668px] flex flex-wrap gap-2",

                // ── Export ───
                button {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded flex-grow",
                    onclick: move |_| download_character(&CHARACTER()),
                    "Export Character"
                }

                // ── Import into Existing ──
                if let Some(incoming) = pending_import() {
                    div { class: "w-full flex flex-col items-center gap-2 border border-red-600 rounded p-3",
                        p { class: "font-mono text-sm text-center",
                            "Overwrite \"{CHARACTER().name}\" with \"{incoming.name}\"?"
                        }
                        div { class: "flex gap-2",
                            button {
                                class: "font-mono bg-red-700 hover:bg-red-600 text-white font-bold py-1 px-4 rounded",
                                onclick: move |_| {
                                    if let Some(character) = pending_import.take() {
                                        *CHARACTER.write() = character;
                                    }
                                },
                                "Confirm"
                            }
                            button {
                                class: "font-mono bg-slate-700 hover:bg-slate-600 text-white font-bold py-1 px-4 rounded",
                                onclick: move |_| { pending_import.set(None); },
                                "Cancel"
                            }
                        }
                    }
                } else {
                    button {
                        class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded flex-grow",
                        onclick: move |_| {
                            spawn(async move {
                                if let Some(character) = pick_character_file().await {
                                    pending_import.set(Some(character));
                                }
                            });
                        },
                        "Import Overwrite"
                    }
                }
            }
        }
    }
}

use arrata_lib::*;
use dioxus::prelude::*;

use crate::components::{
    RenderCombat, RenderQuirks, RenderStats, download_character, pick_character_file,
};
use crate::{Roster, RosterStoreExt};

#[component]
pub(crate) fn RenderCharacter(character: Store<Character>) -> Element {
    rsx! {
        RenderHeader { character }

        div { class: "flex flex-grow flex-wrap pt-4 pb-4 items-start",
            RenderStats { character }
            RenderQuirks { character }
            RenderCombat { character }
        }
    }
}

#[component]
fn RenderHeader(character: WriteStore<Character>) -> Element {
    let mut name = character.name();
    let mut stock = character.stock();
    rsx! {
        div { class: "flex-grid-md",
            div { class: "inline-field-sm",
                h2 { class: "label p-2", "Name:" }
                input {
                    class: "input-name",
                    value: name,
                    oninput: move |evt| name.set(evt.value()),
                }
            }
            div { class: "inline-field-sm",
                h2 { class: "label p-2", "Stock:" }
                input {
                    class: "input-name",
                    value: stock,
                    oninput: move |evt| stock.set(evt.value()),
                }
            }
        }
    }
}

#[component]
pub(crate) fn CharacterIO() -> Element {
    let roster = use_context::<Store<Roster>>();
    let mut character = roster.characters().get(roster.active()()).unwrap();
    let mut pending_import = use_signal::<Option<Character>>(|| None);
    rsx! {
        div { class: "w-full flex justify-center",
            div { class: "px-5 pb-5 font-mono origin-center w-fit max-w-[668px] flex flex-wrap gap-2",

                // ── Export ───
                button {
                    class: "btn-lg",
                    onclick: move |_| download_character(&character()),
                    "Export Character"
                }

                // ── Import into Existing ──
                if let Some(incoming) = pending_import() {
                    div { class: "w-full flex flex-col items-center gap-2 border border-red-600 rounded p-3",
                        p { class: "font-mono text-sm text-center",
                            "Overwrite \"{character.name()}\" with \"{incoming.name}\"?"
                        }
                        div { class: "flex gap-2",
                            button {
                                class: "btn-confirm-delete",
                                onclick: move |_| {
                                    character.set(incoming.clone());
                                },
                                "Confirm"
                            }
                            button {
                                class: "btn-cancel",
                                onclick: move |_| {
                                    pending_import.set(None);
                                },
                                "Cancel"
                            }
                        }
                    }
                } else {
                    button {
                        class: "btn-lg",
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

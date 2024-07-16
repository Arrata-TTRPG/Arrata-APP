use arrata_lib::Character;
use dioxus::prelude::*;
use rfd::AsyncFileDialog;

use crate::{
    render::{RenderInventory, RenderQuirks, RenderStats},
    CHARACTER,
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
                    oninput: move |evt| CHARACTER.write().name = evt.value()
                }
            }
            div { class: "flex sm:w-fit w-full",
                h2 { class: "font-mono text-xl p-2", "Stock:" }
                input {
                    class: "border-spacing-1 border rounded-lg p-2 text-center sm:flex-grow-0 flex-grow",
                    value: "{CHARACTER().stock}",
                    oninput: move |evt| CHARACTER.write().stock = evt.value()
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
        div { class: "w-full flex justify-center",
            div { class: "px-5 pb-5 font-mono origin-center w-fit max-w-[668px] flex flex-wrap gap-2",
                button {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded flex-grow",
                    onclick: move |_| {
                        use_future(|| async move {
                            let file = AsyncFileDialog::new()
                                .set_title("Save .arrata Character file")
                                .add_filter("type", &["arrata"])
                                .set_file_name(format!("{}.arrata", CHARACTER().name))
                                .save_file()
                                .await;
                            if let Some(f) = file {
                                let _ = f.write(&bitcode::encode(&CHARACTER())).await;
                            }
                        });
                    },
                    "Save Character"
                }
                button {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded flex-grow",
                    onclick: move |_| {
                        use_future(|| async move {
                            let file = AsyncFileDialog::new()
                                .set_title("Load .arrata Character file")
                                .add_filter("type", &["arrata"])
                                .pick_file()
                                .await;
                            if let Some(f) = file {
                                let character: Character = bitcode::decode(&f.read().await).unwrap();
                                CHARACTER.write().clone_from(&character);
                            }
                        });
                    },
                    "Load Character"
                }
                button {
                    class: "font-mono text-xl bg-red-900 hover:bg-red-950 text-white font-bold py-2 px-4 rounded flex-grow",
                    onclick: move |_| *CHARACTER.write() = Character::default(),
                    "Reset Character"
                }
            }
        }
    }
}

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
        div { class: "flex flex-wrap content-center items-center justify-center",
            div { class: "flex",
                h2 { class: "font-mono text-xl p-2", "Name:" }
                input {
                    class: "border-spacing-1 border rounded-lg p-2",
                    value: "{CHARACTER().name}",
                    oninput: move |evt| {
                        CHARACTER.write().name = evt.value();
                    }
                }
            }
            div { class: "flex",
                h2 { class: "font-mono text-xl p-2", "Stock:" }
                input {
                    class: "border-spacing-1 border rounded-lg p-2",
                    value: "{CHARACTER().stock}",
                    oninput: move |evt| {
                        CHARACTER.write().stock = evt.value();
                    }
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
            button {
                class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
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
                class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
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
        }
    }
}

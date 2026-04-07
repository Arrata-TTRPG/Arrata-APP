use arrata_lib::Character;
use base64::prelude::*;
use dioxus::prelude::*;

use crate::{
    CHARACTER,
    render::{RenderCombat, RenderQuirks, RenderStats},
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
    rsx! {
        div { class: "w-full flex justify-center",
            div { class: "px-5 pb-5 font-mono origin-center w-fit max-w-[668px] flex flex-wrap gap-2",
                button {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded flex-grow",
                    onclick: move |_| {
                        let bytes = bitcode::encode(&CHARACTER());
                        let b64 = BASE64_STANDARD.encode(&bytes);
                        let filename = format!("{}.arrata", CHARACTER().name);
                        let js = format!(
                            r#"var b=atob("{b64}");var u=new Uint8Array(b.length);for(var i=0;i<b.length;i++)u[i]=b.charCodeAt(i);var blob=new Blob([u],{{type:"application/octet-stream"}});var a=document.createElement("a");a.href=URL.createObjectURL(blob);a.download="{filename}";a.click();URL.revokeObjectURL(a.href);"#
                        );
                        let _ = document::eval(&js);
                    },
                    "Save Character"
                }
                button {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded flex-grow",
                    onclick: move |_| {
                        spawn(async move {
                            let js = r#"
                                var input = document.createElement("input");
                                input.type = "file";
                                input.accept = ".arrata";
                                input.onchange = async function(e) {
                                    var file = e.target.files[0];
                                    if (!file) return;
                                    var buf = await file.arrayBuffer();
                                    var bytes = new Uint8Array(buf);
                                    var b64 = "";
                                    for (var i = 0; i < bytes.length; i += 8192) {
                                        b64 += String.fromCharCode.apply(null, bytes.subarray(i, i + 8192));
                                    }
                                    dioxus.send(btoa(b64));
                                };
                                input.click();
                            "#;
                            let mut eval = document::eval(js);
                            if let Ok(val) = eval.recv::<String>().await {
                                let bytes = BASE64_STANDARD.decode(val.as_bytes()).unwrap_or_default();
                                if let Ok(character) = bitcode::decode::<Character>(&bytes) {
                                    CHARACTER.write().clone_from(&character);
                                } else {
                                    log::error!("Failed to decode character file");
                                }
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


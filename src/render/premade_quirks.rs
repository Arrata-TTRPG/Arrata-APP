use arrata_lib::{Quirk, QuirkCategory};
use base64::prelude::*;
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::bs_icons::{BsSave, BsTrash, BsX},
};
use thousands::Separable;

use crate::{CHARACTER, PREMADE_QUIRKS, PREMADE_QUIRKS_MENU};

#[component]
pub fn RenderPremadeQuirkList() -> Element {
    let shown_categories = (
        use_signal(|| true),
        use_signal(|| true),
        use_signal(|| true),
    );

    rsx! {
        div { class: "z-10 fixed flex flex-col w-full min-h-14 text-white border-white bg-black top-0 bottom-0 rounded-lg",
            // Close button
            div { class: "z-20 absolute right-0 top-0 p-2",
                div {
                    class: "bg-slate-950 hover:bg-slate-700 rounded cursor-pointer",
                    onclick: move |_| *PREMADE_QUIRKS_MENU.write() = false,
                    Icon {
                        width: 35,
                        height: 35,
                        fill: "red",
                        icon: BsX,
                    }
                }
            }

            div { class: "p-2 flex flex-wrap justify-center gap-x-4 gap-y-2",
                h1 { class: "text-center py-2 text-2xl font-bold font-mono", "Premade Quirks" }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 text-white font-mono font-bold flex px-2 h-12 items-center border rounded",
                    onclick: move |_| {
                        let bytes = bitcode::encode(&PREMADE_QUIRKS());
                        let b64 = BASE64_STANDARD.encode(&bytes);
                        let js = format!(
                            r#"var b=atob("{b64}");var u=new Uint8Array(b.length);for(var i=0;i<b.length;i++)u[i]=b.charCodeAt(i);var blob=new Blob([u],{{type:"application/octet-stream"}});var a=document.createElement("a");a.href=URL.createObjectURL(blob);a.download="quirks.quirks";a.click();URL.revokeObjectURL(a.href);"#,
                        );
                        let _ = document::eval(&js);
                    },
                    "Download Quirks"
                }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 text-white font-mono font-bold flex px-2 h-12 items-center border rounded",
                    onclick: move |_| {
                        spawn(async move {
                            // multiple=true mirrors the old pick_files behaviour
                            let js = r#"
                                                                var input = document.createElement("input");
                                                                input.type = "file";
                                                                input.accept = ".quirks";
                                                                input.multiple = true;
                                                                input.onchange = async function(e) {
                                                                    var results = [];
                                                                    for (var i = 0; i < e.target.files.length; i++) {
                                                                        var buf = await e.target.files[i].arrayBuffer();
                                                                        var bytes = new Uint8Array(buf);
                                                                        var b64 = "";
                                                                        for (var j = 0; j < bytes.length; j += 8192) {
                                                                            b64 += String.fromCharCode.apply(null, bytes.subarray(j, j + 8192));
                                                                        }
                                                                        results.push(btoa(b64));
                                                                    }
                                                                    dioxus.send(results);
                                                                };
                                                                input.click();
                                                            "#;
                            let mut eval = document::eval(js);
                            if let Ok(val) = eval.recv::<Vec<String>>().await {
                                for b64 in val {
                                    let bytes = BASE64_STANDARD
                                        .decode(b64.as_bytes())
                                        .unwrap_or_default();
                                    if let Ok(quirks) = bitcode::decode::<Vec<Quirk>>(&bytes) {
                                        PREMADE_QUIRKS.write().extend(quirks);
                                    }
                                }
                                PREMADE_QUIRKS.write().sort_by(|a, b| a.name.cmp(&b.name));
                                PREMADE_QUIRKS.write().dedup();
                            }
                        });
                    },
                    "Load .quirks File"
                }
            }

            // Quirks
            div { class: "h-full gap-1 justify-center p-2 overflow-scroll",
                if PREMADE_QUIRKS().is_empty() {
                    p { class: "flex font-mono text-lg gap-2 place-items-center",
                        "No premade quirks available. Save some here with the"
                        Icon {
                            width: 18,
                            height: 18,
                            fill: "white",
                            icon: BsSave,
                        }
                        "save"
                        "button."
                    }
                }

                div { class: "flex flex-col h-full lg:flex-row overflow-y-clip gap-2",
                    RenderPremadeQuirkCategory {
                        category: QuirkCategory::Ethos,
                        shown: shown_categories.0,
                    }
                    RenderPremadeQuirkCategory {
                        category: QuirkCategory::Pathos,
                        shown: shown_categories.1,
                    }
                    RenderPremadeQuirkCategory {
                        category: QuirkCategory::Logos,
                        shown: shown_categories.2,
                    }
                }
            }
        }
    }
}

#[component]
fn RenderPremadeQuirkCategory(category: QuirkCategory, shown: Signal<bool>) -> Element {
    let num_quirks = PREMADE_QUIRKS()
        .iter()
        .filter(|quirk| quirk.category == category)
        .count()
        .separate_with_commas();
    rsx! {
        div { class: format!("flex flex-col lg:h-full gap-2 border rounded-lg p-1 pt-2 w-full{}", if shown() { " flex-1 min-h-0" } else { "" }),
            div { class: "flex flex-wrap gap-2 justify-center items-center",
                h2 { class: "text-xl font-mono font-bold leading-none mb-0", "{category}" }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-2 border rounded",
                    onclick: move |_| shown.set(!shown()),
                    if shown() {
                        "Hide"
                    } else {
                        "Show"
                    }
                }
                span { class: "text-xl font-mono font-bold leading-none mb-0",
                    "{num_quirks} available"
                }
            }
            if shown() {
                div { class: "flex flex-col border-t w-full h-full gap-3 overflow-y-scroll pr-3 pt-1 min-h-0",
                    for (index, quirk) in PREMADE_QUIRKS()
                        .into_iter()
                        .enumerate()
                        .filter(|(_, quirk)| quirk.category == category)
                    {
                        RenderPremadeQuirk { index, quirk }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderPremadeQuirk(index: usize, quirk: Quirk) -> Element {
    let num_boons = quirk.boons.len();
    let num_flaws = quirk.flaws.len();
    let mut flashing = use_signal(|| false);
    rsx! {
        div {
            class: "flex flex-col bg-slate-950 w-full h-fit p-1 border rounded-lg gap-2",
            key: "{index}",
            div { class: "flex flex-wrap gap-2 justify-center place-items-center",
                button {
                    class: format!("flex font-extrabold font-xl py-1 px-3 border rounded-lg transition-colors duration-700{}",
                    if flashing() { " bg-green-550" } else { " bg-slate-750 hover:bg-slate-500" }),
                    disabled: flashing(),
                    onclick: move |_| {
                        if flashing() { return; }
                        spawn(async move {
                            flashing.set(true);
                            let _ = document::eval("await new Promise(r => setTimeout(r, 700))").await;
                            flashing.set(false);
                        });
                        CHARACTER
                            .with_mut(|character| {
                                character.quirks.push(quirk.clone());
                            });
                    },
                    "{quirk.name} +"
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-1 border rounded-lg",
                    onclick: move |_| std::mem::drop(PREMADE_QUIRKS.write().remove(index)),
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsTrash,
                    }
                }
            }

            if !quirk.description.is_empty() {
                p { class: "font-mono text-md text-center px-1", "{quirk.description}" }
            }

            if num_boons + num_flaws > 0 {
                div { class: "grid grid-cols-2 h-full border-t pt-2 [word-spacing:-4px]",
                    div { class: "flex flex-col gap-1 h-full border-r pt-1",
                        h4 { class: "font-mono text-lg text-center", "Boons" }
                        ul { class: "list-disc list-inside items-start px-2",
                            for (index, boon) in quirk.boons.iter().enumerate() {
                                li {
                                    key: "{index}",
                                    class: format!("text-sm font-mono text-wrap text-left{}",
                                        if num_boons > 1 && index < num_boons - 1 {
                                            " border-b pb-1"
                                        } else {
                                            ""
                                        }),
                                    "{boon}"
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col gap-1 h-full pt-1",
                        h4 { class: "font-mono text-lg text-center", "Flaws" }
                        ul { class: "list-disc list-inside items-start px-2",
                            for (index, flaw) in quirk.flaws.iter().enumerate() {
                                li {
                                    key: "{index}",
                                    class: format!("text-sm font-mono text-wrap text-left{}",
                                        if num_flaws > 1 && index < num_flaws - 1 {
                                            " border-b pb-1"
                                        } else {
                                            ""
                                        }),
                                    "{flaw}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

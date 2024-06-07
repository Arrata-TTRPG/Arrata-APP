use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsTrash, Icon};

use arrata_lib::{Quirk, QuirkCategory};

use crate::CHARACTER;

#[component]
pub(crate) fn RenderQuirks() -> Element {
    rsx! {
        div { class: "w-[1108px] flex-auto justify-center",
            div { class: "flex justify-center content-center items-center",
                h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                    "Argos"
                }
            }
            div { class: "flex justify-center content-center items-center py-2 px-2",
                textarea {
                    class: "rounded-lg w-2/3 py-2 px-2 bg-black text-white border border-white",
                    value: "{CHARACTER().argos}",
                    oninput: move |evt| CHARACTER.write().argos = evt.value().to_string()
                }
            }

            div { class: "flex justify-center content-center items-center",
                h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                    "Quirks"
                }
                button {
                    class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded",
                    onclick: move |_| CHARACTER.write().quirks.push(Quirk::default()),
                    "+ Add Quirk"
                }
            }

            div { class: "flex justify-center",
                div { class: "grid grid-cols-2 gap-4 justify-items-center max-w-5xl",
                    for (i , quirk) in CHARACTER().quirks.iter().enumerate() {
                        div { class: "w-[504px] border border-spacing-2 px-3 py-3 rounded-lg",
                            div { class: "flex justify-center content-center items-center justify-items-center text-2xl py-2 px-2 w-full",
                                div { class: "flex",
                                    input {
                                        class: "w-44 font-mono text-lg text-center border-spacing-1 border rounded-lg py-2 px-2",
                                        r#type: "text",
                                        value: "{quirk.name}",
                                        oninput: move |evt| {
                                            CHARACTER.write().quirks[i].name = evt.value().to_string();
                                        }
                                    }
                                }
                                div { class: "inline-flex justify-center content-center items-center justify-items-center px-2 py-2",
                                    select {
                                        class: "font-mono text-lg border rounded-lg py-2 px-2",
                                        onchange: move |evt| {
                                            CHARACTER
                                                .with_mut(|character| {
                                                    character.quirks[i].category = match evt
                                                        .value()
                                                        .parse::<usize>()
                                                        .unwrap()
                                                    {
                                                        0 => QuirkCategory::Ethos,
                                                        1 => QuirkCategory::Pathos,
                                                        _ => QuirkCategory::Logos,
                                                    }
                                                });
                                        },
                                        option { value: 0, "Ethos" }
                                        option { value: 1, "Pathos" }
                                        option { value: 2, "Logos" }
                                    }
                                }
                                div { class: "flex",
                                    button {
                                        class: "text-mono bg-slate-900 hover:bg-slate-600 text-white font-bold py-1 px-2 space-x-5 rounded",
                                        onclick: move |_| {
                                            let _ = CHARACTER.write().quirks.remove(i);
                                        },
                                        Icon { width: 20, height: 20, fill: "white", icon: BsTrash }
                                    }
                                }
                            }
                            div { class: "flex border justify-center content-center items-center justify-items-center",
                                textarea {
                                    class: "rounded-lg w-full py-2 px-2 bg-black text-white border-white",
                                    value: "{quirk.description}",
                                    oninput: move |evt| {
                                        CHARACTER.write().quirks[i].description = evt.value().to_string();
                                    }
                                }
                            }
                            div { class: "grid grid-cols-2 py-2 px-2",
                                div { class: "inline-flex font-mono text-xl justify-center content-center items-center",
                                    div { class: "font-mono text-xl px-4", "Boons" }
                                    button {
                                        class: "bg-slate-900 hover:bg-slate-500 text-lg text-white font-bold py-1 px-4 rounded",
                                        onclick: move |_| {
                                            CHARACTER
                                                .with_mut(|character| character.quirks[i].boons.push("New Boon!".into()));
                                        },
                                        "+ Boon"
                                    }
                                }
                                div { class: "inline-flex font-mono text-xl justify-center content-center items-center",
                                    div { class: "font-mono text-xl px-4", "Flaws" }
                                    button {
                                        class: "bg-slate-900 hover:bg-slate-500 text-lg text-white font-bold py-1 px-4 rounded",
                                        onclick: move |_| {
                                            CHARACTER
                                                .with_mut(|character| character.quirks[i].flaws.push("New Flaw!".into()));
                                        },
                                        "+ Flaw"
                                    }
                                }
                                div { class: "w-auto items-center justify-items-center",
                                    for (j , boon) in quirk.boons.iter().enumerate() {
                                        div { class: "inline-flex w-full justify-center items-start justify-items-center px-2 py-2",
                                            textarea {
                                                class: "text-mono w-full content-center justify-center border-spacing-1 border rounded-lg py-2 px-2 bg-black text-white",
                                                value: "{boon}",
                                                oninput: move |evt| CHARACTER.write().quirks[i].boons[j] = evt.value().to_string()
                                            }
                                            button {
                                                class: "text-mono bg-slate-900 hover:bg-slate-600 text-white font-bold py-1 px-2 space-x-5 rounded",
                                                onclick: move |_| {
                                                    let _ = CHARACTER.write().quirks[i].boons.remove(j);
                                                },
                                                Icon { width: 20, height: 20, fill: "white", icon: BsTrash }
                                            }
                                        }
                                    }
                                }
                                div { class: "w-auto items-center justify-items-center",
                                    for (j , flaw) in quirk.flaws.iter().enumerate() {
                                        div { class: "inline-flex w-full justify-center items-start justify-items-center px-2 py-2",
                                            textarea {
                                                class: "text-mono w-auto content-center justify-center border-spacing-1 border rounded-lg py-2 px-2 bg-black text-white",
                                                value: "{flaw}",
                                                oninput: move |evt| CHARACTER.write().quirks[i].flaws[j] = evt.value().to_string()
                                            }
                                            button {
                                                class: "text-mono bg-slate-900 hover:bg-slate-600 text-white font-bold py-1 px-2 space-x-5 rounded",
                                                onclick: move |_| {
                                                    let _ = CHARACTER.write().quirks[i].flaws.remove(j);
                                                },
                                                Icon { width: 20, height: 20, fill: "white", icon: BsTrash }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

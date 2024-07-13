use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsTrash, Icon};

use arrata_lib::{Quirk, QuirkCategory};

use crate::CHARACTER;

#[component]
pub(crate) fn RenderQuirks() -> Element {
    rsx! {
        div { class: "min-[1921px]:w-1/3 w-1/2 flex-auto justify-center px-2",
            div { class: "flex justify-center content-center items-center",
                h2 { class: "inline-flex py-2 px-4 text-center text-4xl font-bold font-mono",
                    "Argos"
                }
            }

            textarea {
                class: "rounded-lg w-full p-2 bg-black resize-none h-10 text-white border border-white text-center",
                value: "{CHARACTER().argos}",
                oninput: move |evt| CHARACTER.write().argos = evt.value().to_string()
            }

            div { class: "flex justify-center content-center items-center",
                h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                    "Quirks"
                }
                button {
                    class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 border rounded",
                    onclick: move |_| CHARACTER.write().quirks.push(Quirk::default()),
                    "+ Add Quirk"
                }
            }

            div { class: "flex justify-center",
                div { class: "w-full grid min-[1025px]:grid-cols-2 grid-cols-1 gap-4 justify-items-center",
                    for (i , quirk) in CHARACTER().quirks.iter().enumerate() {
                        div { class: "flex flex-col w-full border border-spacing-2 px-3 py-3 rounded-lg",
                            div { class: "flex justify-center content-center items-center justify-items-center text-2xl p-2 w-full space-x-2",
                                input {
                                    class: "flex-grow font-mono text-lg text-center border-spacing-1 border rounded-lg min-w-10 p-2",
                                    r#type: "text",
                                    value: "{quirk.name}",
                                    oninput: move |evt| {
                                        CHARACTER.write().quirks[i].name = evt.value().to_string();
                                    }
                                }
                                select {
                                    class: "hover:bg-slate-700 flex-grow font-mono text-center text-lg border rounded-lg p-2 appearance-none",
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
                                button {
                                    class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                                    onclick: move |_| {
                                        std::mem::drop(CHARACTER.write().quirks.remove(i));
                                    },
                                    Icon { width: 25, height: 25, fill: "white", icon: BsTrash }
                                }
                            }
                            div { class: "flex border justify-center content-center items-center justify-items-center",
                                textarea {
                                    class: "rounded-lg w-full p-2 bg-black text-white border-white",
                                    value: "{quirk.description}",
                                    oninput: move |evt| {
                                        CHARACTER.write().quirks[i].description = evt.value().to_string();
                                    }
                                }
                            }
                            div { class: "grid grid-cols-2 p-2",
                                div { class: "inline-flex font-mono text-xl space-x-3 justify-center items-center",
                                    h3 { class: "font-mono text-xl", "Boons" }
                                    button {
                                        class: "bg-slate-900 hover:bg-slate-500 text-lg text-white border font-bold rounded py-1 px-2",
                                        onclick: move |_| {
                                            CHARACTER
                                                .with_mut(|character| character.quirks[i].boons.push("New Boon!".into()));
                                        },
                                        "+ Boon"
                                    }
                                }
                                div { class: "inline-flex font-mono text-xl space-x-3 justify-center items-center",
                                    h3 { class: "font-mono text-xl", "Flaws" }
                                    button {
                                        class: "bg-slate-900 hover:bg-slate-500 text-lg text-white border font-bold rounded py-1 px-2",
                                        onclick: move |_| {
                                            CHARACTER
                                                .with_mut(|character| character.quirks[i].flaws.push("New Flaw!".into()));
                                        },
                                        "+ Flaw"
                                    }
                                }
                                div { class: "items-center justify-items-center",
                                    for (j , _) in quirk.boons.iter().enumerate() {
                                        RenderBF { boon: true, quirk: i, index: j }
                                    }
                                }
                                div { class: "items-center justify-items-center",
                                    for (j , _) in quirk.flaws.iter().enumerate() {
                                        RenderBF { boon: false, quirk: i, index: j }
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

#[component]
fn RenderBF(boon: bool, quirk: usize, index: usize) -> Element {
    rsx! {
        div { class: "flex w-full justify-center p-2",
            if boon {
                textarea {
                    class: "w-full text-mono flex-shrink border-spacing-1 border rounded-lg p-2 bg-black text-white",
                    value: "{CHARACTER().quirks[quirk].boons[index]}",
                    oninput: move |evt| CHARACTER.write().quirks[quirk].boons[index] = evt.value().to_string()
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                    onclick: move |_| {
                        std::mem::drop(CHARACTER.write().quirks[quirk].boons.remove(index));
                    },
                    Icon { width: 25, height: 25, fill: "white", icon: BsTrash }
                }
            } else {
                textarea {
                    class: "w-full text-mono flex-shrink border-spacing-1 border rounded-lg p-2 bg-black text-white",
                    value: "{CHARACTER().quirks[quirk].flaws[index]}",
                    oninput: move |evt| CHARACTER.write().quirks[quirk].flaws[index] = evt.value().to_string()
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                    onclick: move |_| {
                        std::mem::drop(CHARACTER.write().quirks[quirk].flaws.remove(index));
                    },
                    Icon { width: 25, height: 25, fill: "white", icon: BsTrash }
                }
            }
        }
    }
}

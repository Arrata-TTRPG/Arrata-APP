use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsSave, BsTrash, BsX},
    Icon,
};

use arrata_lib::{Quirk, QuirkCategory};

use crate::{CHARACTER, PREMADE_QUIRKS, PREMADE_QUIRKS_MENU};

#[component]
pub(crate) fn RenderQuirks() -> Element {
    rsx! {
        div { class: "min-[1921px]:w-1/3 w-full flex flex-col gap-4 justify-center px-2 h-full",
            h2 { class: "text-center text-4xl font-bold font-mono", "Argos" }

            textarea {
                class: "rounded-lg w-full p-2 bg-black resize-none h-fit text-slate-600 font-mono border border-white text-center",
                value: "{CHARACTER().argos}",
                oninput: move |evt| CHARACTER.write().argos = evt.value().to_string()
            }

            RenderInspiration {}

            div { class: "flex flex-wrap gap-3 justify-center content-center items-center",
                h2 { class: "inline-flex text-center text-4xl font-bold font-mono",
                    "Quirks"
                }
                button {
                    class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 border rounded",
                    onclick: move |_| CHARACTER.write().quirks.push(Quirk::default()),
                    "+ Add Quirk"
                }
                button {
                    class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 border rounded",
                    onclick: move |_| *PREMADE_QUIRKS_MENU.write() = true,
                    "+ Load Premade Quirk"
                }
            }

            div { class: "flex flex-col justify-center",
                div { class: "w-full grid min-[1025px]:grid-cols-2 grid-cols-1 gap-4 justify-items-center",
                    for (i , quirk) in CHARACTER().quirks.iter().enumerate() {
                        RenderQuirk { index: i, quirk: quirk.clone() }
                    }
                }
            }
        }

        if PREMADE_QUIRKS_MENU() {
            RenderPremadeQuirkList {}
        }
    }
}

#[component]
fn RenderQuirk(index: usize, quirk: Quirk) -> Element {
    let quirk: Signal<Quirk> = Signal::new(quirk);
    rsx! {
        div { class: "flex flex-col w-full border border-spacing-2 p-1 rounded-lg gap-y-1",
            div { class: "flex justify-center content-center items-center justify-items-center text-2xl w-full gap-x-2",
                input {
                    class: "flex-grow font-mono text-lg text-center border-spacing-1 border rounded-lg min-w-10 p-2",
                    r#type: "text",
                    value: "{quirk().name}",
                    oninput: move |evt| CHARACTER.write().quirks[index].name = evt.value().to_string()
                }
                select {
                    class: "hover:bg-slate-700 font-mono text-center text-lg border rounded-lg p-2 cursor-pointer appearance-none",
                    onchange: move |evt| {
                        CHARACTER
                            .with_mut(|character| {
                                character.quirks[index].category = match evt
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
                    option {
                        value: 0,
                        selected: CHARACTER().quirks[index].category == QuirkCategory::Ethos,
                        "Ethos"
                    }
                    option {
                        value: 1,
                        selected: CHARACTER().quirks[index].category == QuirkCategory::Pathos,
                        "Pathos"
                    }
                    option {
                        value: 2,
                        selected: CHARACTER().quirks[index].category == QuirkCategory::Logos,
                        "Logos"
                    }
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                    onclick: move |_| std::mem::drop(CHARACTER.write().quirks.remove(index)),
                    Icon { width: 25, height: 25, fill: "white", icon: BsTrash }
                }
                button {
                    class: "flex p-2 hover:bg-slate-700 border text-lg rounded-lg cursor-pointer",
                    onclick: move |_| {
                        PREMADE_QUIRKS.write().push(quirk());
                    },
                    Icon { width: 25, height: 25, fill: "white", icon: BsSave }
                }
            }
            div { class: "flex border justify-center content-center items-center justify-items-center",
                textarea {
                    class: "rounded-lg w-full p-2 bg-black text-white border-white",
                    value: "{quirk().description}",
                    oninput: move |evt| {
                        CHARACTER
                            .with_mut(|character| {
                                character.quirks[index].description = evt.value().to_string();
                            });
                    }
                }
            }
            div { class: "grid grid-cols-2 p-1 gap-1",
                div { class: "inline-flex font-mono text-xl gap-x-3 justify-center items-center",
                    h3 { class: "font-mono text-xl", "Boons" }
                    button {
                        class: "bg-slate-900 hover:bg-slate-500 text-lg text-white border font-bold rounded py-1 px-3",
                        onclick: move |_| {
                            CHARACTER
                                .with_mut(|character| {
                                    character.quirks[index].boons.push("New Boon!".into());
                                });
                        },
                        "+"
                    }
                }
                div { class: "inline-flex font-mono text-xl gap-x-3 justify-center items-center",
                    h3 { class: "font-mono text-xl", "Flaws" }
                    button {
                        class: "bg-slate-900 hover:bg-slate-500 text-lg text-white border font-bold rounded py-1 px-3",
                        onclick: move |_| {
                            CHARACTER
                                .with_mut(|character| {
                                    character.quirks[index].flaws.push("New Flaw!".into());
                                });
                        },
                        "+"
                    }
                }
                div { class: "flex flex-col gap-y-1 items-center justify-items-center",
                    for (j , _) in quirk().boons.iter().enumerate() {
                        RenderBF { boon: true, quirk: index, index: j }
                    }
                }
                div { class: "flex flex-col gap-y-1 items-center justify-items-center",
                    for (j , _) in quirk().flaws.iter().enumerate() {
                        RenderBF { boon: false, quirk: index, index: j }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderBF(boon: bool, quirk: usize, index: usize) -> Element {
    rsx! {
        div { class: "flex gap-x-1 w-full justify-center",
            if boon {
                textarea {
                    class: "w-full text-mono flex-shrink border-spacing-1 border p-2 bg-black text-white",
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
                    class: "w-full text-mono flex-shrink border-spacing-1 border p-2 bg-black text-white",
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

#[component]
fn RenderInspiration() -> Element {
    rsx! {
        h3 { class: "text-center p-2 text-3xl font-bold", "Inspiration" }

        div { class: "flex justify-center gap-4",
            div { class: "flex flex-row gap-2 p-2 border rounded-xl place-items-center",
                h4 { class: "text-center text-2xl font-bold", "Ethos" }
                input {
                    class: "rounded-lg w-16 p-2 bg-black text-white border border-white text-center",
                    r#type: "number",
                    min: 0,
                    max: i64::MAX
                }
            }
            div { class: "flex flex-row gap-2 p-2 border rounded-xl place-items-center",
                h4 { class: "text-center text-2xl font-bold", "Pathos" }
                input {
                    class: "rounded-lg w-16 p-2 bg-black text-white border border-white text-center",
                    r#type: "number",
                    min: 0,
                    max: i64::MAX
                }
            }
            div { class: "flex flex-row gap-2 p-2 border rounded-xl place-items-center",
                h4 { class: "text-center text-2xl font-bold", "Logos" }
                input {
                    class: "rounded-lg w-16 p-2 bg-black text-white border border-white text-center",
                    r#type: "number",
                    min: 0,
                    max: i64::MAX
                }
            }
        }
    }
}

#[component]
fn RenderPremadeQuirkList() -> Element {
    rsx! {
        div { class: "z-10 fixed flex flex-col justify-center content-center max-w-[80%] min-w-96 w-fit h-fit min-h-14 border text-white border-white bg-slate-800 m-auto left-0 right-0 top-0 bottom-0 rounded-lg",
            // Close button
            div { class: "z-20 absolute right-0 top-0 p-2",
                div {
                    class: "bg-slate-950 hover:bg-slate-700 rounded cursor-pointer",
                    onclick: move |_| *PREMADE_QUIRKS_MENU.write() = false,
                    Icon { width: 35, height: 35, fill: "red", icon: BsX }
                }
            }

            h1 { class: "text-center py-2 text-2xl font-bold font-mono", "Premade Quirks" }

            // Quirks
            div { class: "flex flex-wrap gap-1 justify-center p-2",
                if PREMADE_QUIRKS().is_empty() {
                    div { class: "flex font-mono text-lg gap-2 place-items-center",
                        p { "No premade quirks available. Save some here with the" }
                        Icon { width: 18, height: 18, fill: "white", icon: BsSave }
                        p { "button." }
                    }
                }

                for (index , quirk) in PREMADE_QUIRKS().into_iter().enumerate() {
                    div { class: "flex flex-col p-1 border rounded-lg gap-2",
                        // Name, add, and remove buttons
                        div { class: "flex flex-wrap gap-2 justify-center place-items-center",
                            h3 { class: "text-lg font-bold", "{quirk.name}" }
                            button {
                                class: "flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-2 border rounded",
                                onclick: move |_| {
                                    CHARACTER
                                        .with_mut(|character| {
                                            character.quirks.push(quirk.clone());
                                        });
                                },
                                "+ Add"
                            }
                            button {
                                class: "bg-red-950 hover:bg-red-600 p-1 border rounded-lg",
                                onclick: move |_| std::mem::drop(PREMADE_QUIRKS.write().remove(index)),
                                Icon { width: 25, height: 25, fill: "white", icon: BsTrash }
                            }
                        }

                        // Description
                        if !quirk.description.is_empty() {
                            p { class: "font-mono text-base text-center p-1 border",
                                "{quirk.description}"
                            }
                        }

                        // Boons and flaws
                        if !quirk.boons.is_empty() || !quirk.flaws.is_empty() {
                            div { class: "grid grid-cols-2 gap-1",
                                div { class: "flex flex-col gap-1",
                                    h4 { class: "font-mono text-lg text-center", "Boons" }
                                    for boon in quirk.boons.iter() {
                                        p { class: "text-sm font-mono", "{boon}" }
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

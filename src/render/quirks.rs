use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::bs_icons::{BsSave, BsTrash},
};
use thousands::Separable;

use arrata_lib::{Quirk, QuirkCategory};

use crate::{
    CHARACTER, PREMADE_QUIRKS, PREMADE_QUIRKS_MENU, SHOWN_CATEGORIES, render::auto_resize_js,
};

#[component]
pub(crate) fn RenderQuirks() -> Element {
    let mut show_argos = use_signal(|| false);
    rsx! {
        div { class: "flex w-full min-[1281px]:w-1/2 min-[1921px]:w-1/3 max-[1920px]:pt-10 flex-col gap-4 justify-center px-2",
            div { class: "flex flex-row justify-center gap-2",
                h2 { class: "text-center text-4xl font-bold font-mono", "Argos" }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 font-bold py-1 px-4 border rounded",
                    onclick: move |_| show_argos.set(!show_argos()),
                    if show_argos() {
                        "Hide"
                    } else {
                        "Show"
                    }
                }
            }

            if show_argos() {
                textarea {
                    id: "argos",
                    class: "rounded-lg w-full p-2 bg-black resize-none overflow-hidden h-fit font-mono border border-white text-center",
                    style: "min-height: 2.75rem; color: rgb(150 150 150);
                            background-image:   radial-gradient(circle, white 0.4px, transparent 1px),
                                                radial-gradient(circle, white 0.4px, transparent 1px);
                            background-size: 10px 10px;
                            background-position: 0 0, 5px 5px;",
                    value: "{CHARACTER().argos}",
                    placeholder: "TODO: Find purpose.",
                    onmounted: move |_| async move {
                        let _ = document::eval(&auto_resize_js("argos", true)).await;
                    },
                    oninput: move |evt| {
                        CHARACTER.write().argos.clone_from(&evt.value());
                        let _ = document::eval(&auto_resize_js("argos", false));
                    },
                }
            }

            RenderInspiration {}

            div { class: "flex flex-wrap gap-3 justify-center content-center items-center",
                h2 { class: "inline-flex text-center text-4xl font-bold font-mono",
                    "Quirks"
                }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 font-bold py-1 px-4 border rounded",
                    onclick: move |_| *PREMADE_QUIRKS_MENU.write() = true,
                    "+ Load Premade Quirk"
                }
            }

            div { class: "grid min-[1280px]:grid-cols-1 min-[1200px]:grid-cols-3 grid-cols-1 items-start gap-2",
                RenderQuirkCategory {
                    category: QuirkCategory::Ethos,
                    show: SHOWN_CATEGORIES().0,
                }
                RenderQuirkCategory {
                    category: QuirkCategory::Pathos,
                    show: SHOWN_CATEGORIES().1,
                }
                RenderQuirkCategory {
                    category: QuirkCategory::Logos,
                    show: SHOWN_CATEGORIES().2,
                }
            }
        }

        if PREMADE_QUIRKS_MENU() {
            crate::render::premade_quirks::RenderPremadeQuirkList {}
        }
    }
}

#[component]
fn RenderQuirkCategory(category: QuirkCategory, show: bool) -> Element {
    let num_quirks = CHARACTER()
        .quirks
        .iter()
        .filter(|quirk| quirk.category == category)
        .count();
    let category = Signal::new(category);
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-2",
            div { class: "flex justify-center items-center flex-row flex-wrap gap-4",
                h1 { class: "text-center text-3xl font-bold font-mono", "{category} {num_quirks.separate_with_commas()}" }
                button {
                    class: "font-bold py-1 px-2 border rounded bg-slate-900 hover:bg-slate-500",
                    onclick: move |_| {
                        CHARACTER
                            .write()
                            .quirks
                            .push(Quirk {
                                name: String::new(),
                                description: String::new(),
                                category: category(),
                                boons: vec![],
                                flaws: vec![],
                            });
                    },
                    "+"
                }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 font-bold py-1 px-4 border rounded",
                    onclick: move |_| {
                        match category() {
                            QuirkCategory::Ethos => SHOWN_CATEGORIES.write().0 = !SHOWN_CATEGORIES().0,
                            QuirkCategory::Pathos => SHOWN_CATEGORIES.write().1 = !SHOWN_CATEGORIES().1,
                            QuirkCategory::Logos => SHOWN_CATEGORIES.write().2 = !SHOWN_CATEGORIES().2,
                            QuirkCategory::Uncategorized => {}
                        }
                    },
                    if show {
                        "Hide"
                    } else {
                        "Show"
                    }
                }
            }
            if show {
                div { class: "flex flex-wrap p-2 border rounded-lg w-full max-h-96 overflow-y-scroll gap-3",
                    for (index, quirk) in CHARACTER()
                        .quirks
                        .iter()
                        .enumerate()
                        .filter(|(_, quirk)| quirk.category == category())
                    {
                        RenderQuirk { index, quirk: quirk.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderQuirk(index: usize, quirk: Quirk) -> Element {
    let quirk: Signal<Quirk> = Signal::new(quirk);
    rsx! {
        div { class: "flex flex-1 flex-col w-full md:w-1/2 border border-spacing-2 p-2 rounded-lg gap-1",
            div { class: "flex justify-center content-center items-center justify-items-center text-2xl w-full gap-x-2",
                input {
                    class: "flex-grow font-mono text-lg text-center border-spacing-1 border rounded-lg min-w-10 p-2",
                    r#type: "text",
                    value: "{quirk().name}",
                    placeholder: "Quirk Name",
                    oninput: move |evt| CHARACTER.write().quirks[index].name.clone_from(&evt.value()),
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                    onclick: move |_| std::mem::drop(CHARACTER.write().quirks.remove(index)),
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsTrash,
                    }
                }
                button {
                    class: "flex p-2 hover:bg-slate-700 border text-lg rounded-lg cursor-pointer",
                    onclick: move |_| {
                        PREMADE_QUIRKS.write().push(quirk());
                        PREMADE_QUIRKS.write().sort_by(|a, b| a.name.cmp(&b.name));
                        PREMADE_QUIRKS.write().dedup();
                    },
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsSave,
                    }
                }
            }
            div { class: "flex border justify-center content-center items-center justify-items-center",
                textarea {
                    id: "quirk-desc-{index}",
                    class: "rounded-lg w-full p-2 bg-black border-white resize-none overflow-hidden",
                    style: "min-height: 2.75rem",
                    value: "{quirk().description}",
                    placeholder: "Get quirky with it.",
                    onmounted: move |_| async move {
                        let _ = document::eval(&auto_resize_js(&format!("quirk-desc-{index}"), true))
                            .await;
                    },
                    oninput: move |evt| {
                        CHARACTER
                            .with_mut(|character| {
                                character.quirks[index].description.clone_from(&evt.value());
                            });
                        let _ = document::eval(&auto_resize_js(&format!("quirk-desc-{index}"), false));
                    },
                }
            }
            div { class: "grid grid-cols-2 p-1 gap-1",
                div { class: "inline-flex font-mono text-xl gap-x-3 justify-center items-center",
                    h3 { class: "font-mono text-xl", "Boons" }
                    button {
                        class: "bg-slate-900 hover:bg-slate-500 text-lg border font-bold rounded py-1 px-3",
                        onclick: move |_| {
                            CHARACTER
                                .with_mut(|character| {
                                    character.quirks[index].boons.push(String::new());
                                });
                        },
                        "+"
                    }
                }
                div { class: "inline-flex font-mono text-xl gap-x-3 justify-center items-center",
                    h3 { class: "font-mono text-xl", "Flaws" }
                    button {
                        class: "bg-slate-900 hover:bg-slate-500 text-lg border font-bold rounded py-1 px-3",
                        onclick: move |_| {
                            CHARACTER
                                .with_mut(|character| {
                                    character.quirks[index].flaws.push(String::new());
                                });
                        },
                        "+"
                    }
                }
                div { class: "flex flex-col gap-y-1 items-center justify-items-center",
                    for (j, _) in quirk().boons.iter().enumerate() {
                        RenderBF { boon: true, quirk: index, index: j }
                    }
                }
                div { class: "flex flex-col gap-y-1 items-center justify-items-center",
                    for (j, _) in quirk().flaws.iter().enumerate() {
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
                    id: "quirk-boon-{quirk}-{index}",
                    class: "w-full text-mono flex-shrink border-spacing-1 border p-2 bg-black resize-none overflow-hidden",
                    style: "min-height: 2.75rem",
                    value: "{CHARACTER().quirks[quirk].boons[index]}",
                    placeholder: "Boon",
                    onmounted: move |_| async move {
                        let _ = document::eval(
                                &auto_resize_js(&format!("quirk-boon-{quirk}-{index}"), true),
                            )
                            .await;
                    },
                    oninput: move |evt| {
                        CHARACTER.write().quirks[quirk].boons[index].clone_from(&evt.value());
                        let _ = document::eval(
                            &auto_resize_js(&format!("quirk-boon-{quirk}-{index}"), false),
                        );
                    },
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                    onclick: move |_| {
                        std::mem::drop(CHARACTER.write().quirks[quirk].boons.remove(index));
                    },
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsTrash,
                    }
                }
            } else {
                textarea {
                    id: "quirk-flaw-{quirk}-{index}",
                    class: "w-full text-mono flex-shrink border-spacing-1 border p-2 bg-black resize-none overflow-hidden",
                    style: "min-height: 2.75rem",
                    value: "{CHARACTER().quirks[quirk].flaws[index]}",
                    placeholder: "Flaw",
                    onmounted: move |_| async move {
                        let _ = document::eval(
                                &auto_resize_js(&format!("quirk-flaw-{quirk}-{index}"), true),
                            )
                            .await;
                    },
                    oninput: move |evt| {
                        CHARACTER.write().quirks[quirk].flaws[index].clone_from(&evt.value());
                        let _ = document::eval(
                            &auto_resize_js(&format!("quirk-flaw-{quirk}-{index}"), false),
                        );
                    },
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                    onclick: move |_| {
                        std::mem::drop(CHARACTER.write().quirks[quirk].flaws.remove(index));
                    },
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsTrash,
                    }
                }
            }
        }
    }
}

#[component]
fn RenderInspiration() -> Element {
    rsx! {
        div { class: "flex flex-col space-y-2 border rounded-xl",
            h3 { class: "text-center p-2 text-3xl font-bold", "Inspiration" }
            div { class: "flex justify-center gap-4",
                div { class: "flex flex-1 flex-wrap flex-col gap-2 p-2 rounded-xl justify-center place-items-center",
                    h4 { class: "text-center text-2xl font-bold", "Ethos" }
                    input {
                        class: "rounded-lg w-24 p-2 bg-black border border-white text-center",
                        r#type: "number",
                        min: 0,
                        max: i64::MAX,
                        value: "{CHARACTER().inspiration.ethos}",
                        onchange: move |evt| {
                            CHARACTER.write().inspiration.ethos = evt.value().parse::<usize>().unwrap_or(0);
                        },
                    }
                }
                div { class: "flex flex-1 flex-wrap flex-col gap-2 p-2 rounded-xl justify-center place-items-center",
                    h4 { class: "text-center text-2xl font-bold", "Pathos" }
                    input {
                        class: "rounded-lg w-24 p-2 bg-black border border-white text-center",
                        r#type: "number",
                        min: 0,
                        max: i64::MAX,
                        value: "{CHARACTER().inspiration.pathos}",
                        onchange: move |evt| {
                            CHARACTER.write().inspiration.pathos = evt.value().parse::<usize>().unwrap_or(0);
                        },
                    }
                }
                div { class: "flex flex-1 flex-wrap flex-col gap-2 p-2 rounded-xl justify-center place-items-center",
                    h4 { class: "text-center text-2xl font-bold", "Logos" }
                    input {
                        class: "rounded-lg w-24 p-2 bg-black border border-white text-center",
                        r#type: "number",
                        min: 0,
                        max: i64::MAX,
                        value: "{CHARACTER().inspiration.logos}",
                        onchange: move |evt| {
                            CHARACTER.write().inspiration.logos = evt.value().parse::<usize>().unwrap_or(0);
                        },
                    }
                }
            }
        }
    }
}

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
        div { class: "flex w-full min-[1281px]:w-1/2 min-[1921px]:w-1/3 max-[1280px]:pt-10 flex-col gap-4 justify-center px-2",
            div { class: "flex flex-row justify-center gap-2",
                h1 { "Argos" }
                button {
                    class: "btn",
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
                    class: "textarea-notes text-center",
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

            div { class: "flex-grid-big",
                h1 { "Quirks" }
                button {
                    class: "btn",
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
            div { class: "flex-grid-big w-full",
                h2 { "{category} {num_quirks.separate_with_commas()}" }
                button {
                    class: "btn",
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
                    class: "btn",
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
            div { class: "inline-field",
                input {
                    class: "input-stat",
                    r#type: "text",
                    value: "{quirk().name}",
                    placeholder: "Quirk Name",
                    oninput: move |evt| CHARACTER.write().quirks[index].name.clone_from(&evt.value()),
                }
                button {
                    class: "btn-danger",
                    onclick: move |_| std::mem::drop(CHARACTER.write().quirks.remove(index)),
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsTrash,
                    }
                }
                button {
                    class: "btn-add",
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
            textarea {
                id: "quirk-desc-{index}",
                class: "textarea-quirk",
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
            div { class: "grid grid-cols-2 p-1 gap-1",
                div { class: "inline-field",
                    h4 { "Boons" }
                    button {
                        class: "btn-sm text-lg",
                        onclick: move |_| {
                            CHARACTER
                                .with_mut(|character| {
                                    character.quirks[index].boons.push(String::new());
                                });
                        },
                        "+"
                    }
                }
                div { class: "inline-field",
                    h4 { "Flaws" }
                    button {
                        class: "btn-sm text-lg",
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
                    class: "textarea-quirk",
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
                    class: "btn-danger",
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
                    class: "textarea-quirk",
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
                    class: "btn-danger",
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
        div { class: "flex-col-md border rounded-xl",
            h1 { "Inspiration" }
            div { class: "inline-field gap-1 pt-4",
                div { class: "flex-col-md flex-1",
                    h3 { "Ethos" }
                    input {
                        class: "input-counter min-w-16 w-full max-w-24",
                        r#type: "number",
                        min: 0,
                        max: u64::MAX,
                        value: "{CHARACTER().inspiration.ethos}",
                        onchange: move |evt| {
                            CHARACTER.write().inspiration.ethos = evt.value().parse::<usize>().unwrap_or(0);
                        },
                    }
                }
                div { class: "flex-col-md flex-1",
                    h3 { "Pathos" }
                    input {
                        class: "input-counter min-w-16 w-full max-w-24",
                        r#type: "number",
                        min: 0,
                        max: u64::MAX,
                        value: "{CHARACTER().inspiration.pathos}",
                        onchange: move |evt| {
                            CHARACTER.write().inspiration.pathos = evt.value().parse::<usize>().unwrap_or(0);
                        },
                    }
                }
                div { class: "flex-col-md flex-1",
                    h3 { "Logos" }
                    input {
                        class: "input-counter min-w-16 w-full max-w-24",
                        r#type: "number",
                        min: 0,
                        max: u64::MAX,
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

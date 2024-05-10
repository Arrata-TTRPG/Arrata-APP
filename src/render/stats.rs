use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::{BsDice6, BsTrash}, Icon};

use crate::{character::{Quality, Stat}, CHARACTER, DICE_ROLL_STATE};

#[component]
pub(crate) fn RenderStats() -> Element {
    rsx! {
        div { class: "w-[748px] flex-auto justify-items-center justify-center",
            h2 { class: "py-4 text-center text-4xl font-bold font-mono",
                "Stats {CHARACTER().stats.iter().map(|stat| stat.quantity).sum::<usize>()}"
            }
            div { class: "flex justify-center justify-items-center content-center",
                div { class: "grid grid-cols-2 gap-4 justify-center justify-items-center content-center max-w-5xl",
                    for (i , stat) in CHARACTER().stats.iter().enumerate() {
                        div { class: "flex flex-col border border-spacing-2 items-center justify-center justify-items-center px-2 py-2 rounded-lg",
                            div { class: "inline-flex items-center justify-center py-2 px-2 w-auto",
                                div { class: "font-mono text-center text-2xl py-2 px-2", "{stat.name}" }
                                div { class: "py-2 px-2",
                                    button { class: "bg-slate-900 hover:bg-slate-600",
                                        onclick: move |_| {
                                            DICE_ROLL_STATE.with_mut(|state| {
                                                state.0 = true;
                                                state.1 = Some(CHARACTER().stats[i].clone());
                                            });
                                        },
                                        Icon {
                                            width: 30,
                                            height: 30,
                                            fill: "white",
                                            icon: BsDice6
                                        },
                                    }
                                }
                            },
                            div { class: "inline-flex w-full justify-center content-center items-center justify-items-center",
                                select { class: "font-mono border rounded-lg py-2 px-2",
                                    onchange: move |evt| {
                                        CHARACTER.with_mut(|character| {
                                            character.stats[i].quality = match evt.value().parse::<usize>().unwrap() {
                                                1 => Quality::Adept,
                                                2 => Quality::Superb,
                                                _ => Quality::Basic,
                                            }
                                        });
                                    },
                                    option {
                                        value: 0,
                                        "Basic"
                                    },
                                    option {
                                        value: 1,
                                        "Adept"
                                    },
                                    option {
                                        value: 2,
                                        "Superb"
                                    },
                                },
                                input { class: "w-16 border rounded-lg py-2 px-2",
                                    r#type:"number",
                                    value: i64::try_from(stat.quantity).unwrap_or_default(),
                                    oninput: move |evt| {
                                        CHARACTER.with_mut(|character| {
                                            character.stats[i].quantity = evt.value().parse::<usize>().unwrap_or(0);
                                        });
                                    }
                                },
                                div { class: "font-mono text-lg py-2 px-2", "Checks:" },
                                input { class: "w-16 border rounded-lg py-2 px-2",
                                    r#type:"number",
                                    value: i64::try_from(stat.checks.unwrap_or_default()).unwrap_or_default(),
                                    oninput: move |evt| {
                                        CHARACTER.with_mut(|character| {
                                            character.stats[i].checks = Some(evt.value().parse::<usize>().unwrap_or(0));
                                        });
                                    }
                                },
                            }
                        }
                    }
                }
            }
            div { class: "flex justify-center content-center items-center",
                h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                    "Skills"
                }
                button {
                    class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded",
                    onclick: move |_| CHARACTER.write().skills.push(Stat::new("New Skill!".into())),
                    "+ Add Skill"
                }
            }
            div { class: "flex justify-center",
                div { class: "grid grid-cols-2 gap-4 justify-items-center max-w-5xl",
                for (i , skill) in CHARACTER().skills.iter().enumerate() {
                        div { class: "flex flex-col border border-spacing-2 px-2 py-2 rounded-lg",
                            div { class: "flex justify-center content-center items-center justify-items-center text-2xl py-2 px-2",
                                div { class: "flex px-2 py-2",
                                    input { class: "w-40 font-mono text-lg text-center border-spacing-1 border rounded-lg py-2 px-2",
                                        r#type:"text",
                                        value: "{skill.name}",
                                        oninput: move |evt| {
                                            CHARACTER.write().skills[i].name = evt.value().to_string();
                                        }
                                    }
                                }
                                div { class: "px-2 py-2",
                                    button { class: "bg-slate-900 hover:bg-slate-600",
                                        onclick: move |_| {
                                            DICE_ROLL_STATE.with_mut(|state| {
                                                state.0 = true;
                                                state.1 = Some(CHARACTER().skills[i].clone());
                                            });
                                        },
                                        Icon {
                                            width: 30,
                                            height: 30,
                                            fill: "white",
                                            icon: BsDice6
                                        }
                                    }
                                }
                                div { class: "px-2 py-2 rounded-lg",
                                    button { class: "bg-slate-900 hover:bg-slate-600 py-2 px-2 space-x-5 rounded",
                                        onclick: move |_| { let _ = CHARACTER.write().skills.remove(i); },
                                        Icon {
                                            width: 20,
                                            height: 20,
                                            fill: "white",
                                            icon: BsTrash
                                        }
                                    }
                                }
                            }
                            div { class: "inline-flex justify-center content-center items-center justify-items-center",
                                select { class: "font-mono border rounded-lg py-2 px-2",
                                    onchange: move |evt| {
                                        CHARACTER.with_mut(|character| {
                                            character.skills[i].quality = match evt.value().parse::<usize>().unwrap() {
                                                1 => Quality::Adept,
                                                2 => Quality::Superb,
                                                _ => Quality::Basic,
                                            }
                                        });
                                    },
                                    option {
                                        value: 0,
                                        "Basic"
                                    },
                                    option {
                                        value: 1,
                                        "Adept"
                                    },
                                    option {
                                        value: 2,
                                        "Superb"
                                    },
                                },
                                input { class: "w-16 border rounded-lg py-2 px-2",
                                    r#type:"number",
                                    value: i64::try_from(skill.quantity).unwrap_or_default(),
                                    oninput: move |evt| {
                                        CHARACTER.with_mut(|character| {
                                            character.skills[i].quantity = evt.value().parse::<usize>().unwrap_or(0);
                                        });
                                    }
                                },
                                div { class: "font-mono text-lg py-2 px-2", "Checks:" },
                                input { class: "w-16 border rounded-lg py-2 px-2",
                                    r#type:"number",
                                    value: i64::try_from(skill.checks.unwrap_or(0)).unwrap_or_default(),
                                    oninput: move |evt| {
                                        CHARACTER.with_mut(|character| {
                                            character.skills[i].checks = Some(evt.value().parse::<usize>().unwrap_or(0));
                                        });
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
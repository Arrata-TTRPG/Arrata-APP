use super::structs::*;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsDice6, BsTrash};
use dioxus_free_icons::Icon;

#[component(no_case_check)]
pub fn render_character<'a>(
    cx: Scope,
    character: &'a UseRef<Character>,
    dice_roll_state: &'a UseState<(bool, Option<Stat>)>,
) -> Element<'a> {
    cx.render(rsx!{
        div { class: "flex content-center items-center justify-center",
            div { class: "font-mono text-xl px-2 py-2", "Name:" }
            input {
                class: "border-spacing-1 border rounded-lg px-2 py-2",
                value: "{character.read().name}",
                oninput: move |evt| {
                    character.write().name = evt.value.clone();
                }
            }
            div { class: "font-mono text-xl px-2 py-2", "Stock:" }
            input {
                class: "border-spacing-1 border rounded-lg px-2 py-2",
                value: "{character.read().stock}",
                oninput: move |evt| {
                    character.write().stock = evt.value.clone();
                }
            }
        }

        div { class: "flex flex-wrap",
            div { class: "w-[748px] flex-auto justify-items-center justify-center",
                h2 { class: "py-4 text-center text-4xl font-bold font-mono", "Stats" }
                div { class: "flex justify-center justify-items-center content-center",
                    div { class: "grid grid-cols-2 gap-4 justify-center justify-items-center content-center max-w-5xl",
                        for (i , stat) in character.read().stats.iter().enumerate() {
                            rsx!(
                                div { class: "flex flex-col border border-spacing-2 items-center justify-center justify-items-center px-2 py-2 rounded-lg",
                                    div { class: "inline-flex items-center justify-center py-2 px-2 w-auto",
                                        div { class: "font-mono text-center text-2xl py-2 space-x-5", "{stat.name}" }
                                        div { class: "bg-slate-900 hover:bg-slate-600 rounded",
                                            button {
                                                onclick: move |_| {
                                                    dice_roll_state.with_mut(|state| {
                                                        state.0 = true;
                                                        state.1 = Some(character.read().stats[i].clone());
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
                                                character.with_mut(|character| {
                                                    character.stats[i].quality = match evt.value.parse::<usize>().unwrap() {
                                                        0 => Quality::Basic,
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
                                        input { class: "w-12 border rounded-lg py-2 px-2",
                                            r#type:"number",
                                            value: stat.quantity as f64,
                                            oninput: move |evt| {
                                                character.with_mut(|character| {
                                                    character.stats[i].quantity = evt.value.parse::<usize>().unwrap_or(0);
                                                });
                                            }
                                        },
                                        div { class: "font-mono text-lg py-2 px-2", "Checks:" },
                                        input { class: "w-12 border rounded-lg py-2 px-2",
                                            r#type:"number",
                                            value: stat.checks.unwrap_or(0) as f64,
                                            oninput: move |evt| {
                                                character.with_mut(|character| {
                                                    character.stats[i].checks = Some(evt.value.parse::<usize>().unwrap_or(0));
                                                });
                                            }
                                        },
                                    }
                                }
                            )
                        }
                    }
                }
                div { class: "flex justify-center content-center items-center",
                    h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                        "Skills"
                    }
                    button { class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded",
                        onclick: move |_| character.write().skills.push(Stat::new("New Skill!".into())),
                        "+ Add Skill"
                    }
                }
                div { class: "flex justify-center",
                    div { class: "grid grid-cols-2 gap-4 justify-items-center max-w-5xl",
                        for (i , skill) in character.read().skills.iter().enumerate() {
                            rsx!(
                                div { class: "flex flex-col border border-spacing-2 px-2 py-2 rounded-lg",
                                    div { class: "flex justify-center content-center items-center justify-items-center text-2xl py-2 px-2",
                                        div { class: "flex px-2 py-2",
                                            input { class: "w-40 font-mono text-lg text-center border-spacing-1 border rounded-lg py-2 px-2",
                                                r#type:"text",
                                                value: "{skill.name}",
                                                oninput: move |evt| {
                                                    character.write().skills[i].name = evt.value.to_string();
                                                }
                                            }
                                        }
                                        div { class: "flex px-2 py-2 space-x-5",
                                            button { class: "bg-slate-900 hover:bg-slate-600 rounded",
                                                onclick: move |_| {
                                                    dice_roll_state.with_mut(|state| {
                                                        state.0 = true;
                                                        state.1 = Some(character.read().skills[i].clone());
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
                                        div { class: "flex px-2 py-2 rounded-lg",
                                            button { class: "bg-slate-900 hover:bg-slate-600 py-2 px-2 space-x-5 rounded",
                                                onclick: move |_| { let _ = character.write().skills.remove(i); },
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
                                                character.with_mut(|character| {
                                                    character.skills[i].quality = match evt.value.parse::<usize>().unwrap() {
                                                        0 => Quality::Basic,
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
                                        input { class: "w-12 border rounded-lg py-2 px-2",
                                            r#type:"number",
                                            value: skill.quantity as f64,
                                            oninput: move |evt| {
                                                character.with_mut(|character| {
                                                    character.skills[i].quantity = evt.value.parse::<usize>().unwrap_or(0);
                                                });
                                            }
                                        },
                                        div { class: "font-mono text-lg py-2 px-2", "Checks:" },
                                        input { class: "w-12 border rounded-lg py-2 px-2",
                                            r#type:"number",
                                            value: skill.checks.unwrap_or(0) as f64,
                                            oninput: move |evt| {
                                                character.with_mut(|character| {
                                                    character.skills[i].checks = Some(evt.value.parse::<usize>().unwrap_or(0));
                                                });
                                            }
                                        }
                                    }
                                }
                            )
                        }
                    }
                }
            }

            div { class: "w-[1108px] flex-auto justify-items-center justify-center content-center",
                div { class: "flex justify-center content-center items-center",
                    h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                        "Argos"
                    }
                }
                div { class: "flex justify-center content-center items-center py-2 px-2",
                    textarea { class: "rounded-lg w-2/3 py-2 px-2 bg-black text-white border border-white",
                        value: "{character.read().argos}",
                        oninput: move |evt| character.write().argos = evt.value.to_string()
                    }
                }

                div { class: "flex justify-center content-center items-center",
                    h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                        "Quirks"
                    }
                    button { class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded",
                        onclick: move |_| character.write().quirks.push(Quirk::default()),
                        "+ Add Quirk"
                    }
                }

                div { class: "flex justify-center",
                    div { class: "grid grid-cols-2 gap-4 justify-items-center max-w-5xl",
                        for (i , quirk) in character.read().quirks.iter().enumerate() {
                            rsx!(
                                div { class: "w-[504px] border border-spacing-2 px-3 py-3 rounded-lg",
                                    div { class: "flex justify-center content-center items-center justify-items-center text-2xl py-2 px-2 w-full",
                                        div { class: "flex",
                                            input { class: "w-44 font-mono text-lg text-center border-spacing-1 border rounded-lg py-2 px-2",
                                                r#type:"text",
                                                value: "{quirk.name}",
                                                oninput: move |evt| {
                                                    character.write().quirks[i].name = evt.value.to_string();
                                                }
                                            }
                                        }
                                        div { class: "inline-flex justify-center content-center items-center justify-items-center px-2 py-2",
                                            select { class: "font-mono text-lg border rounded-lg py-2 px-2",
                                                onchange: move |evt| {
                                                    character.with_mut(|character| {
                                                        character.quirks[i].category = match evt.value.parse::<usize>().unwrap() {
                                                            0 => QuirkCategory::Ethos,
                                                            1 => QuirkCategory::Pathos,
                                                            _ => QuirkCategory::Logos,
                                                        }
                                                    });
                                                },
                                                option {
                                                    value: 0,
                                                    "Ethos"
                                                },
                                                option {
                                                    value: 1,
                                                    "Pathos"
                                                },
                                                option {
                                                    value: 2,
                                                    "Logos"
                                                },
                                            },
                                        }
                                        div { class: "flex",
                                            button { class: "text-mono bg-slate-900 hover:bg-slate-600 text-white font-bold py-1 px-2 space-x-5 rounded",
                                                onclick: move |_| { let _ = character.write().quirks.remove(i); },
                                                Icon {
                                                    width: 20,
                                                    height: 20,
                                                    fill: "white",
                                                    icon: BsTrash
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex border justify-center content-center items-center justify-items-center",
                                        textarea { class: "rounded-lg w-full py-2 px-2 bg-black text-white border-white",
                                            value: "{quirk.description}",
                                            oninput: move |evt| {
                                                character.write().quirks[i].description = evt.value.to_string();
                                            }
                                        }
                                    }
                                    div { class: "grid grid-cols-2 py-2 px-2",
                                        div { class: "inline-flex font-mono text-xl justify-center content-center items-center",
                                            div { class: "font-mono text-xl px-4",
                                                "Boons",
                                            }
                                            button { class: "bg-slate-900 hover:bg-slate-500 text-lg text-white font-bold py-1 px-4 rounded",
                                                onclick: move |_| character.with_mut(|character| {
                                                    character.quirks[i].boons.push("New Boon!".into())
                                                }),

                                                "+ Boon"
                                            }
                                        }
                                        div { class: "inline-flex font-mono text-xl justify-center content-center items-center",
                                            div { class: "font-mono text-xl px-4",
                                                "Flaws",
                                            }
                                            button { class: "bg-slate-900 hover:bg-slate-500 text-lg text-white font-bold py-1 px-4 rounded",
                                                onclick: move |_| character.with_mut(|character| {
                                                    character.quirks[i].flaws.push("New Flaw!".into())
                                                }),
                                                "+ Flaw"
                                            }
                                        }
                                        div { class: "w-auto items-center justify-items-center",
                                            for (j, boon) in quirk.boons.iter().enumerate() {rsx!(
                                                div { class: "inline-flex w-full justify-center items-start justify-items-center px-2 py-2",
                                                    textarea { class: "text-mono w-full content-center justify-center border-spacing-1 border rounded-lg py-2 px-2 bg-black text-white",
                                                        value: "{boon}",
                                                        oninput: move |evt| character.write().quirks[i].boons[j] = evt.value.to_string()
                                                    }
                                                    button { class: "text-mono bg-slate-900 hover:bg-slate-600 text-white font-bold py-1 px-2 space-x-5 rounded",
                                                        onclick: move |_| { let _ = character.write().quirks[i].boons.remove(j); },
                                                        Icon {
                                                            width: 20,
                                                            height: 20,
                                                            fill: "white",
                                                            icon: BsTrash
                                                        }
                                                    }
                                                }
                                            )}
                                        }
                                        div { class: "w-auto items-center justify-items-center",
                                            for (j, flaw) in quirk.flaws.iter().enumerate() {rsx!(
                                                div { class: "inline-flex w-full justify-center items-start justify-items-center px-2 py-2",
                                                    textarea { class: "text-mono w-auto content-center justify-center border-spacing-1 border rounded-lg py-2 px-2 bg-black text-white",
                                                        value: "{flaw}",
                                                        oninput: move |evt| character.write().quirks[i].flaws[j] = evt.value.to_string()
                                                    }
                                                    button { class: "text-mono bg-slate-900 hover:bg-slate-600 text-white font-bold py-1 px-2 space-x-5 rounded",
                                                        onclick: move |_| { let _ = character.write().quirks[i].flaws.remove(j); },
                                                        Icon {
                                                            width: 20,
                                                            height: 20,
                                                            fill: "white",
                                                            icon: BsTrash
                                                        }
                                                    }
                                                }
                                            )}
                                        }
                                    }
                                }
                            )
                        }
                    }
                }
            }

            div { class: "w-[704px] flex-auto justify-center",
                div { class: "flex justify-center content-center items-center",
                    h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                        "Inventory"
                    }
                    button { class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded",
                        onclick: move |_| character.write().inventory.push(Item::default()),
                        "+ Add Item"
                    }
                }
                div { class: "flex justify-center",
                    div { class: "grid grid-cols-2 gap-4 justify-center justify-items-center max-w-2xl",
                        for (i , item) in character.read().inventory.iter().enumerate() {
                            rsx!(
                                div { class: "justify-center content-center items-center justify-items-center border border-spacing-2 px-3 py-3 top-2 bottom-2 left-2 right-2 rounded-lg",
                                    div { class: "inline-flex items-center content-center",
                                        div {
                                            input { class: "w-44 font-mono text-lg text-center border-spacing-1 border rounded-lg py-2 px-2",
                                                r#type:"text",
                                                value: "{item.name}",
                                                oninput: move |evt| {
                                                    character.write().inventory[i].name = evt.value.to_string();
                                                }
                                            }
                                        }
                                        div {
                                            input { class: "w-12 border rounded-lg py-2 px-2",
                                                r#type:"number",
                                                value: item.quantity as f64,
                                                oninput: move |evt| {
                                                    character.with_mut(|character| {
                                                        character.inventory[i].quantity = evt.value.parse::<usize>().unwrap_or(0);
                                                    });
                                                }
                                            },
                                        }
                                        div { class: "px-2 py-2",
                                            button { class: "text-mono bg-slate-900 hover:bg-slate-600 text-white font-bold py-1 px-2 space-x-5 rounded",
                                                onclick: move |_| { let _ = character.write().inventory.remove(i); },
                                                Icon {
                                                    width: 20,
                                                    height: 20,
                                                    fill: "white",
                                                    icon: BsTrash
                                                }
                                            }
                                        }
                                    }
                                }
                            )
                        }
                    }
                }
            }
        }
    })
}

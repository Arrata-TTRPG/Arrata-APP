use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsDice6, BsTrash},
    Icon,
};

use arrata_lib::{Quality, Resource, Stat};

use crate::{CHARACTER, DICE_ROLL_STATE};

#[component]
pub(crate) fn RenderStats() -> Element {
    rsx! {
        div { class: "min-[1920px]:w-1/3 min-[1280px]:w-1/2 w-full h-full flex flex-col justify-center px-2 gap-4",
            RenderCoreStats {}
            RenderSkills {}
            RenderResources {}
        }
    }
}

#[component]
fn RenderCoreStats() -> Element {
    let stats_total: usize = CHARACTER()
        .stats
        .iter()
        .map(|stat| {
            let quantity = stat.quantity;
            match stat.quality {
                Quality::Basic => quantity,
                Quality::Adept => quantity + 5,
                Quality::Superb => quantity + 10,
            }
        })
        .sum();

    rsx! {
        h2 { class: "text-center text-4xl font-bold font-mono", "Stats {stats_total}" }
        div { class: "flex justify-center justify-items-center content-center",
            div { class: "grid min-[1340px]:grid-cols-2 min-[670px]:grid-cols-2 grid-cols-1 gap-4 justify-center justify-items-center content-center w-full",
                for (i , stat) in CHARACTER().stats.iter().enumerate() {
                    div { class: "flex flex-col border gap-2 justify-center p-2 rounded-lg w-full",
                        div { class: "inline-flex items-center justify-center",
                            div { class: "flex-grow font-mono text-center text-2xl",
                                "{stat.name}"
                            }
                            button {
                                class: "bg-slate-900 hover:bg-slate-600",
                                onclick: move |_| {
                                    DICE_ROLL_STATE
                                        .with_mut(|state| {
                                            state.0 = true;
                                            state.1 = Some(CHARACTER().stats[i].clone());
                                        });
                                },
                                Icon {
                                    width: 45,
                                    height: 45,
                                    fill: "white",
                                    icon: BsDice6,
                                }
                            }
                        }
                        div { class: "inline-flex w-full h-full justify-center items-center space-x-2",
                            select {
                                class: "hover:bg-slate-700 flex-grow font-mono text-center border rounded-lg p-2 appearance-none cursor-pointer",
                                onchange: move |evt| {
                                    CHARACTER
                                        .with_mut(|character| {
                                            character.stats[i].quality = match evt.value().parse::<usize>().unwrap()
                                            {
                                                1 => Quality::Adept,
                                                2 => Quality::Superb,
                                                _ => Quality::Basic,
                                            }
                                        });
                                },
                                option {
                                    value: 0,
                                    selected: CHARACTER().stats[i].quality == Quality::Basic,
                                    "Basic"
                                }
                                option {
                                    value: 1,
                                    selected: CHARACTER().stats[i].quality == Quality::Adept,
                                    "Adept"
                                }
                                option {
                                    value: 2,
                                    selected: CHARACTER().stats[i].quality == Quality::Superb,
                                    "Superb"
                                }
                            }
                            input {
                                class: "w-16 border rounded-lg p-2 appearance-none",
                                r#type: "number",
                                value: "{stat.quantity}",
                                min: 0,
                                max: i64::MAX,
                                oninput: move |evt| {
                                    CHARACTER
                                        .with_mut(|character| {
                                            character.stats[i].quantity = evt.value().parse::<usize>().unwrap_or(0);
                                        });
                                },
                            }
                            div { class: "font-mono text-lg align-middle h-fit", "Checks:" }
                            input {
                                class: "w-16 border rounded-lg p-2",
                                r#type: "number",
                                value: "{stat.checks.unwrap_or_default()}",
                                min: 0,
                                max: i64::MAX,
                                oninput: move |evt| {
                                    CHARACTER
                                        .with_mut(|character| {
                                            character.stats[i].checks = Some(
                                                evt.value().parse::<usize>().unwrap_or(0),
                                            );
                                        });
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderSkills() -> Element {
    rsx! {
        div { class: "flex flex-row justify-center content-center items-center py-2",
            h2 { class: "px-4 text-center text-4xl font-bold font-mono",
                "Skills {CHARACTER().skills.iter().count()}"
            }
            button {
                class: "bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded h-full border",
                onclick: move |_| CHARACTER.write().skills.push(Stat::new("New Skill!".into())),
                "+ Add Skill"
            }
        }
        div { class: "flex justify-center",
            div { class: "grid min-[1340px]:grid-cols-2 min-[670px]:grid-cols-2 grid-cols-1 gap-4 justify-center justify-items-center content-center w-full",
                for (i , skill) in CHARACTER().skills.iter().enumerate() {
                    div { class: "flex flex-col border p-2 rounded-lg w-full space-y-2",
                        div { class: "flex w-full justify-center items-center text-2xl space-x-2",
                            input {
                                class: "flex flex-grow font-mono text-lg text-center border-spacing-1 border rounded-lg min-w-10 p-2",
                                r#type: "text",
                                value: "{skill.name}",
                                oninput: move |evt| {
                                    CHARACTER.write().skills[i].name = evt.value().to_string();
                                },
                            }
                            button {
                                class: "bg-slate-900 hover:bg-slate-600",
                                onclick: move |_| {
                                    DICE_ROLL_STATE
                                        .with_mut(|state| {
                                            state.0 = true;
                                            state.1 = Some(CHARACTER().skills[i].clone());
                                        });
                                },
                                Icon {
                                    width: 45,
                                    height: 45,
                                    fill: "white",
                                    icon: BsDice6,
                                }
                            }
                            button {
                                class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                                onclick: move |_| {
                                    std::mem::drop(CHARACTER.write().skills.remove(i));
                                },
                                Icon {
                                    width: 25,
                                    height: 25,
                                    fill: "white",
                                    icon: BsTrash,
                                }
                            }
                        }
                        div { class: "inline-flex justify-center content-center items-center justify-items-center space-x-2",
                            select {
                                class: "flex-grow hover:bg-slate-700 font-mono text-center text-lg border rounded-lg p-2 appearance-none cursor-pointer",
                                onchange: move |evt| {
                                    CHARACTER
                                        .with_mut(|character| {
                                            character.skills[i].quality = match evt.value().parse::<usize>().unwrap()
                                            {
                                                1 => Quality::Adept,
                                                2 => Quality::Superb,
                                                _ => Quality::Basic,
                                            }
                                        });
                                },
                                option {
                                    value: 0,
                                    selected: CHARACTER().skills[i].quality == Quality::Basic,
                                    "Basic"
                                }
                                option {
                                    value: 1,
                                    selected: CHARACTER().skills[i].quality == Quality::Adept,
                                    "Adept"
                                }
                                option {
                                    value: 2,
                                    selected: CHARACTER().skills[i].quality == Quality::Superb,
                                    "Superb"
                                }
                            }
                            input {
                                class: "w-16 border rounded-lg p-2",
                                r#type: "number",
                                value: "{skill.quantity}",
                                min: 0,
                                max: i64::MAX,
                                oninput: move |evt| {
                                    CHARACTER
                                        .with_mut(|character| {
                                            character.skills[i].quantity = evt.value().parse::<usize>().unwrap_or(0);
                                        });
                                },
                            }
                            div { class: "font-mono text-lg", "Checks:" }
                            input {
                                class: "w-16 border rounded-lg p-2",
                                r#type: "number",
                                value: "{skill.checks.unwrap_or(0)}",
                                min: 0,
                                max: i64::MAX,
                                oninput: move |evt| {
                                    CHARACTER
                                        .with_mut(|character| {
                                            character.skills[i].checks = Some(
                                                evt.value().parse::<usize>().unwrap_or(0),
                                            );
                                        });
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderResources() -> Element {
    let mut render_finite = use_signal(|| true);
    let mut render_infinite = use_signal(|| true);
    rsx! {
        div { class: "flex flex-wrap justify-center items-start",
            div { class: "flex flex-col min-[670px]:w-1/2 w-full p-1 gap-2",
                // Infinite resources
                div { class: "flex flex-row justify-center items-center gap-2",
                    h2 { class: "text-center text-3xl font-bold font-mono", "Infinite Resources" }
                    button {
                        class: "bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded h-full border",
                        onclick: move |_| {
                            CHARACTER
                                .write()
                                .resources
                                .push(Resource::new(Stat::new("New Infinite Resource!".into()), false));
                        },
                        "+ Add"
                    }
                    button {
                        class: "bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded h-full border",
                        onclick: move |_| {
                            render_infinite.set(!render_infinite());
                        },
                        if render_infinite() {
                            "Hide"
                        } else {
                            "Show"
                        }
                    }
                }
                div { class: "flex flex-col gap-1",
                    if render_infinite() {
                        for (i , _) in CHARACTER().resources.iter().enumerate().filter(|(_, r)| !r.finite) {
                            RenderResource { index: i }
                        }
                    }
                }
            }
            // Finite resources
            div { class: "flex flex-col min-[670px]:w-1/2 w-full p-1 gap-2",
                div { class: "flex flex-row justify-center items-center gap-2",
                    h2 { class: "text-center text-3xl font-bold font-mono", "Finite Resources" }
                    button {
                        class: "bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded h-full border",
                        onclick: move |_| {
                            CHARACTER
                                .write()
                                .resources
                                .push(Resource::new(Stat::new("New Finite Resource!".into()), true));
                        },
                        "+ Add"
                    }
                    button {
                        class: "bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded h-full border",
                        onclick: move |_| {
                            render_finite.set(!render_finite());
                        },
                        if render_finite() {
                            "Hide"
                        } else {
                            "Show"
                        }
                    }
                }
                div { class: "flex flex-col gap-1",
                    if render_finite() {
                        for (i , _) in CHARACTER().resources.iter().enumerate().filter(|(_, r)| r.finite) {
                            RenderResource { index: i }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderResource(index: usize) -> Element {
    rsx! {
        if let Some(r) = CHARACTER().resources.get(index) {
            div { class: "flex flex-col border p-2 rounded-lg w-full space-y-2",
                div { class: "flex w-full justify-center items-center text-2xl space-x-2",
                    input {
                        class: "flex flex-grow font-mono text-lg text-center border-spacing-1 border rounded-lg min-w-10 p-2",
                        r#type: "text",
                        value: "{r.stat.name}",
                        oninput: move |evt| {
                            CHARACTER.write().resources[index].stat.name = evt.value().to_string();
                        },
                    }
                    button {
                        class: "bg-slate-900 hover:bg-slate-600",
                        onclick: move |_| {
                            DICE_ROLL_STATE
                                .with_mut(|state| {
                                    state.0 = true;
                                    state.1 = Some(CHARACTER().resources[index].stat.clone());
                                });
                        },
                        Icon {
                            width: 45,
                            height: 45,
                            fill: "white",
                            icon: BsDice6,
                        }
                    }
                    button {
                        class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                        onclick: move |_| {
                            std::mem::drop(CHARACTER.write().resources.remove(index));
                        },
                        Icon {
                            width: 25,
                            height: 25,
                            fill: "white",
                            icon: BsTrash,
                        }
                    }
                }
                div { class: "inline-flex justify-center content-center items-center justify-items-center space-x-2",
                    select {
                        class: "flex-grow hover:bg-slate-700 font-mono text-center text-lg border rounded-lg p-2 appearance-none cursor-pointer",
                        onchange: move |evt| {
                            CHARACTER
                                .with_mut(|character| {
                                    character.resources[index].stat.quality = match evt
                                        .value()
                                        .parse::<usize>()
                                        .unwrap()
                                    {
                                        1 => Quality::Adept,
                                        2 => Quality::Superb,
                                        _ => Quality::Basic,
                                    }
                                });
                        },
                        option {
                            value: 0,
                            selected: CHARACTER().resources[index].stat.quality == Quality::Basic,
                            "Basic"
                        }
                        option {
                            value: 1,
                            selected: CHARACTER().resources[index].stat.quality == Quality::Adept,
                            "Adept"
                        }
                        option {
                            value: 2,
                            selected: CHARACTER().resources[index].stat.quality == Quality::Superb,
                            "Superb"
                        }
                    }
                    input {
                        class: "w-16 border rounded-lg p-2",
                        r#type: "number",
                        value: "{r.stat.quantity}",
                        min: 0,
                        max: i64::MAX,
                        oninput: move |evt| {
                            CHARACTER
                                .with_mut(|character| {
                                    character.resources[index].stat.quantity = evt
                                        .value()
                                        .parse::<usize>()
                                        .unwrap_or(0);
                                });
                        },
                    }
                    div { class: "font-mono text-lg", "Checks:" }
                    input {
                        class: "w-16 border rounded-lg p-2",
                        r#type: "number",
                        value: "{r.stat.checks.unwrap_or(0)}",
                        min: 0,
                        max: i64::MAX,
                        oninput: move |evt| {
                            CHARACTER
                                .with_mut(|character| {
                                    character.resources[index].stat.checks = Some(
                                        evt.value().parse::<usize>().unwrap_or(0),
                                    );
                                });
                        },
                    }
                }
            }
        }
    }
}

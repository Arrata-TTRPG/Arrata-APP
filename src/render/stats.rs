use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::bs_icons::{BsDice6, BsTrash},
};
use thousands::Separable;

use arrata_lib::{Item, Quality, Resource, Stat};

use crate::{CHARACTER, DICE_ROLL_STATE};

#[component]
pub(crate) fn RenderStats() -> Element {
    rsx! {
        div { class: "flex w-full min-[1281px]:w-1/2 min-[1921px]:w-1/3 flex-col justify-center px-2 gap-4",
            RenderCoreStats {}
            RenderSkills {}
            //RenderResources {} // TODO: Overhaul resources
            RenderInventory {}
        }
    }
}

#[component]
fn RenderCoreStats() -> Element {
    let stats_total = CHARACTER()
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
        .sum::<usize>()
        .separate_with_commas();

    rsx! {
        h1 { "Stats {stats_total}" }
        div { class: "flex-grid-none",
            div { class: "grid max-[650px]:grid-cols-1 grid-cols-2 gap-4 justify-center content-center w-full",
                for (i, stat) in CHARACTER().stats.iter().enumerate() {
                    div { class: "flex-col-md border rounded-lg p-2 flex-1 w-full",
                        div { class: "inline-field",
                            h3 { class: "flex-grow",
                                "{stat.name}"
                            }
                            button {
                                class: "btn-ghost",
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
                        div { class: "flex flex-wrap w-full h-full justify-center items-center gap-2",
                            div { class: "inline-field flex-1 gap-2",
                                select {
                                    class: "select-field min-w-12 flex-1",
                                    onchange: move |evt| {
                                        CHARACTER
                                            .with_mut(|character| {
                                                character.stats[i].quality = match evt.value().parse::<usize>().unwrap_or(1)
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
                                    class: "input-stat",
                                    r#type: "number",
                                    value: "{stat.quantity}",
                                    min: 0,
                                    max: u64::MAX,
                                    oninput: move |evt| {
                                        CHARACTER
                                            .with_mut(|character| {
                                                character.stats[i].quantity = evt.value().parse::<usize>().unwrap_or(0);
                                            });
                                    },
                                }
                            }
                            div { class: "inline-flex items-center justify-center space-x-2",
                                span { class: "label", "Checks:" }
                                input {
                                    class: "input-counter",
                                    r#type: "number",
                                    value: "{stat.checks.unwrap_or_default()}",
                                    min: 0,
                                    max: u64::MAX,
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
}

#[component]
fn RenderSkills() -> Element {
    let mut show = use_signal(|| false);
    rsx! {
        div { class: "inline-field",
            h1 { "Skills {CHARACTER().skills.iter().count().separate_with_commas()}" }
            button {
                class: "btn-add",
                onclick: move |_| CHARACTER.write().skills.push(Stat::new(String::new())),
                "+"
            }
            button {
                class: "btn",
                onclick: move |_| show.set(!show()),
                if show() { "Hide" } else { "Show" }
            }
        }
        if show() {
            div { class: "flex justify-center border rounded-lg p-2",
                div { class: "flex-grid-big",
                    for (i, skill) in CHARACTER().skills.iter().enumerate() {
                        div { class: "flex-col-md flex-1 border rounded-lg",
                            div { class: "inline-field w-full",
                                input {
                                    class: "input-stat",
                                    r#type: "text",
                                    value: "{skill.name}",
                                    placeholder: "Skill Name",
                                    oninput: move |evt| {
                                        CHARACTER.write().skills[i].name.clone_from(&evt.value());
                                    },
                                }
                                button {
                                    class: "btn-ghost",
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
                                    class: "btn-danger",
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
                            div { class: "flex-grid-md",
                                div { class: "inline-field flex-1",
                                    select {
                                        class: "select-field flex-1",
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
                                        class: "input-stat",
                                        r#type: "number",
                                        value: "{skill.quantity}",
                                        min: 0,
                                        max: u64::MAX,
                                        oninput: move |evt| {
                                            CHARACTER
                                                .with_mut(|character| {
                                                    character.skills[i].quantity = evt.value().parse::<usize>().unwrap_or(0);
                                                });
                                        },
                                    }
                                }
                                div { class: "inline-field",
                                    span { class: "label", "Checks:" }
                                    input {
                                        class: "input-counter",
                                        r#type: "number",
                                        value: "{skill.checks.unwrap_or(0)}",
                                        min: 0,
                                        max: u64::MAX,
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
                    h2 { class: "section-title text-3xl", "Infinite Resources" }
                    button {
                        class: "btn-add",
                        onclick: move |_| {
                            CHARACTER
                                .write()
                                .resources
                                .push(Resource::new(Stat::new("New Infinite Resource!".into()), false));
                        },
                        "+ Add"
                    }
                    button {
                        class: "btn-add",
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
                        for (i, _) in CHARACTER().resources.iter().enumerate().filter(|(_, r)| !r.finite) {
                            RenderResource { index: i }
                        }
                    }
                }
            }
            // Finite resources
            div { class: "flex flex-col min-[670px]:w-1/2 w-full p-1 gap-2",
                div { class: "flex flex-row justify-center items-center gap-2",
                    h2 { class: "section-title text-3xl", "Finite Resources" }
                    button {
                        class: "btn-add",
                        onclick: move |_| {
                            CHARACTER
                                .write()
                                .resources
                                .push(Resource::new(Stat::new("New Finite Resource!".into()), true));
                        },
                        "+ Add"
                    }
                    button {
                        class: "btn-add",
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
                        for (i, _) in CHARACTER().resources.iter().enumerate().filter(|(_, r)| r.finite) {
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
            div { class: "card w-full space-y-2",
                div { class: "flex w-full justify-center items-center text-2xl space-x-2",
                    input {
                        class: "input-stat",
                        r#type: "text",
                        value: "{r.stat.name}",
                        oninput: move |evt| {
                            CHARACTER.write().resources[index].stat.name.clone_from(&evt.value());
                        },
                    }
                    button {
                        class: "btn-ghost",
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
                        class: "btn-danger",
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
                div { class: "inline-field-sm space-x-2",
                    select {
                        class: "select-field flex-grow",
                        onchange: move |evt| {
                            CHARACTER
                                .with_mut(|character| {
                                    character.resources[index].stat.quality = match evt
                                        .value()
                                        .parse::<usize>()
                                        .unwrap_or(1)
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
                        class: "input-counter",
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
                        class: "input-counter",
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

#[component]
fn RenderInventory() -> Element {
    let mut show = use_signal(|| false);
    rsx! {
        div { class: "flex-col-md",
            div { class: "inline-field",
                h1 { "Inventory {CHARACTER().inventory.iter().count().separate_with_commas()}" }
                button {
                    class: "btn-add",
                    onclick: move |_| CHARACTER.write().inventory.push(Item::default()),
                    "+"
                }
                button {
                    class: "btn",
                    onclick: move |_| show.set(!show()),
                    if show() {
                        "Hide"
                    } else {
                        "Show"
                    }
                }
            }
            if show() {
                div { class: "flex-grid-md border rounded-lg p-2",
                    for (i, item) in CHARACTER().inventory.iter().enumerate() {
                        div { class: "inline-field border rounded-lg p-2 flex-1 min-w-[200px]",
                            input {
                                class: "input-stat flex-grow",
                                r#type: "text",
                                value: "{item.name}",
                                placeholder: "Item",
                                oninput: move |evt| CHARACTER.write().inventory[i].name.clone_from(&evt.value()),
                            }
                            input {
                                class: "input-counter",
                                r#type: "number",
                                value: i64::try_from(item.quantity).unwrap_or_default(),
                                oninput: move |evt| {
                                    CHARACTER
                                        .with_mut(|character| {
                                            character.inventory[i].quantity = evt
                                                .value()
                                                .parse::<usize>()
                                                .unwrap_or(0);
                                        });
                                },
                            }
                            button {
                                class: "btn-danger",
                                onclick: move |_| std::mem::drop(CHARACTER.write().inventory.remove(i)),
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
        }
    }
}

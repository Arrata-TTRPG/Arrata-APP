use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::bs_icons::BsTrash};
use thousands::Separable;

use arrata_lib::{Armor, Talent, Weapon, combat};

use crate::{CHARACTER, render::auto_resize_js};

const WILL_IDX: usize = 0;
const SPEED_IDX: usize = 4;
const FORTE_IDX: usize = 5;

const STAT_NAMES: [&str; 7] = [
    "Will",
    "Perception",
    "Conscious",
    "Power",
    "Speed",
    "Forte",
    "None",
];

/// Top-level combat section: derived stats, then weapon/armor/talent lists.
#[component]
pub(crate) fn RenderCombat() -> Element {
    rsx! {
        div { class: "flex flex-wrap w-full max-[1920px]:pt-10 min-[1921px]:w-1/3 min-[1921px]:pt-0 px-2 justify-center",
            RenderCombatStats {}
            RenderWeapons {}
            RenderArmor {}
            RenderTalents {}
        }
    }
}

// Combat stats

#[component]
fn RenderCombatStats() -> Element {
    let will_qty = CHARACTER().stats.get(WILL_IDX).map_or(1, |s| s.quantity);
    let forte_qty = CHARACTER().stats.get(FORTE_IDX).map_or(1, |s| s.quantity);
    let speed_qty = CHARACTER().stats.get(SPEED_IDX).map_or(1, |s| s.quantity);

    let max_hp = combat::max_health(will_qty, forte_qty);
    let max_ap = combat::ap_cap(speed_qty);
    let mut current_ap = use_signal(|| 0);
    let current_hp = CHARACTER().current_health;
    let injury = CHARACTER().injury;

    rsx! {
        div { class: "flex w-full flex-col gap-3 pb-4 gap-4",
            h1 { "Combat" }

            div { class: "flex-grid-big max-[1280px]:flex-col min-[1281px]:flex-row",
                // Health
                div { class: "flex-col-md border rounded-lg min-w-30 max-[1280px]:w-full",
                    span { "Health" }
                    div { class: "inline-field",
                        input {
                            class: "input-counter flex-1 min-[1281px]:max-w-30",
                            r#type: "number",
                            value: current_hp,
                            min: "0",
                            max: usize::MAX,
                            oninput: move |evt| {
                                CHARACTER
                                    .with_mut(|c| {
                                        c.current_health = evt.value().parse::<usize>().unwrap_or(0);
                                    });
                            },
                        }
                        p { class: "label whitespace-nowrap", "/ {max_hp.separate_with_commas()}" }
                    }
                }

                // Injury
                div { class: "flex-col-md border rounded-lg min-w-30 max-[1280px]:w-full min-[1281px]:h-full",
                    span { "Injury" }
                    div { class: "inline-field",
                        button {
                            class: "btn-add",
                            onclick: move |_| {
                                CHARACTER
                                    .with_mut(|c| {
                                        c.injury += 1;
                                    });
                            },
                            "+"
                        }
                        h2 { "{injury.separate_with_commas()}" }
                        button {
                            class: "btn-add",
                            onclick: move |_| {
                                CHARACTER
                                    .with_mut(|c| {
                                        c.injury = c.injury.saturating_sub(1);
                                    });
                            },
                            "-"
                        }
                    }
                }

                // Action Points
                div { class: "flex-col-md border rounded-lg min-w-30 max-[1280px]:w-full",
                    span { "Action Points" }
                    div { class: "inline-field",
                        input {
                            class: "input-counter flex-1 min-[1281px]:max-w-30",
                            r#type: "number",
                            value: "{current_ap()}",
                            oninput: move |evt| {
                                current_ap.set(evt.value().parse::<isize>().unwrap_or(0));
                            },
                        }
                        p { class: "label whitespace-nowrap", "/ {max_ap.separate_with_commas()}" }
                    }
                }
            }
        }
    }
}

// Weapons

#[component]
fn RenderWeapons() -> Element {
    let mut show = use_signal(|| false);
    rsx! {
        div { class: "flex min-[1281px]:max-[1920px]:w-1/2 min-[1281px]:max-[1920px]:pr-1 w-full flex-col gap-2",
            div { class: "inline-field",
                h2 { "Weapons {CHARACTER().weapons.iter().count().separate_with_commas()}" }
                button {
                    class: "btn-add",
                    onclick: move |_| CHARACTER.write().weapons.push(Weapon::default()),
                    "+"
                }
                button {
                    class: "btn text-lg",
                    onclick: move |_| show.set(!show()),
                    if show() {
                        "Hide"
                    } else {
                        "Show"
                    }
                }
            }
            if show() {
                div { class: "grid max-[650px]:grid-cols-1 grid-cols-2 gap-4 justify-center content-center w-full",
                    for (i, _) in CHARACTER().weapons.iter().enumerate() {
                        RenderWeapon { index: i }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderWeapon(index: usize) -> Element {
    let Some(w) = CHARACTER().weapons.get(index).cloned() else {
        return rsx! {};
    };

    rsx! {
        div { class: "flex-grid-md min-w-[310px] border rounded-lg p-2",
            // Name + delete
            div { class: "inline-field",
                input {
                    class: "input-stat",
                    r#type: "text",
                    placeholder: "Weapon Name",
                    value: "{w.name}",
                    oninput: move |evt| {
                        CHARACTER.write().weapons[index].name = evt.value();
                    },
                }
                button {
                    class: "btn-danger",
                    onclick: move |_| {
                        std::mem::drop(CHARACTER.write().weapons.remove(index));
                    },
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsTrash,
                    }
                }
            }

            // Skill (bounded) + Req (fixed)
            div { class: "flex-grid-big",
                div { class: "inline-field-sm flex-1",
                    p { class: "label", "Skill:" }
                    input {
                        class: "input-stat",
                        r#type: "text",
                        value: "{w.skill}",
                        placeholder: "None",
                        oninput: move |evt| {
                            CHARACTER.write().weapons[index].skill = evt.value();
                        },
                    }
                }
                div { class: "inline-field-sm",
                    p { class: "label", "Min:" }
                    input {
                        class: "input-counter",
                        r#type: "text",
                        value: "{w.skill_requirement.clone().unwrap_or_default()}",
                        placeholder: "B0",
                        oninput: move |evt| {
                            let v = evt.value();
                            CHARACTER.write().weapons[index].skill_requirement =
                                if v.is_empty() { None } else { Some(v) };
                        },
                    }
                }
            }

            // Base dmg + stat dropdown
            div { class: "flex-grid-md",
                div { class: "inline-field-sm",
                    p { class: "label", "Base dmg:" }
                    input {
                        class: "input-counter",
                        r#type: "number",
                        value: i64::from(w.base_damage),
                        oninput: move |evt| {
                            CHARACTER.write().weapons[index].base_damage =
                                evt.value().parse::<i32>().unwrap_or(0);
                        },
                    }
                }
                div { class: "inline-field-sm",
                    p { class: "label", "+" }
                    select {
                        class: "select-field",
                        onchange: move |evt| {
                            CHARACTER.write().weapons[index].stat_modifier = evt.value();
                        },
                        for name in STAT_NAMES {
                            option {
                                value: "{name}",
                                selected: w.stat_modifier == name,
                                "{name}"
                            }
                        }
                    }
                }
            }

            // Notes
            textarea {
                id: "weapon-notes-{index}",
                class: "textarea-notes",
                style: "min-height: 2.75rem",
                placeholder: "Notes",
                value: "{w.notes}",
                onmounted: move |_| async move {
                    let _ = document::eval(&auto_resize_js(&format!("weapon-notes-{index}"), true))
                        .await;
                },
                oninput: move |evt| {
                    CHARACTER.write().weapons[index].notes = evt.value();
                    let _ = document::eval(&auto_resize_js(&format!("weapon-notes-{index}"), false));
                },
            }
        }
    }
}

// Armor

#[component]
fn RenderArmor() -> Element {
    let mut show = use_signal(|| false);
    rsx! {
        div { class: "flex min-[1281px]:max-[1920px]:w-1/2 min-[1281px]:max-[1920px]:pl-1 min-[1281px]:max-[1920px]:py-0 w-full flex-col gap-2 py-4",
            div { class: "flex flex-row justify-center items-center py-2 gap-4",
                h2 { class: "text-center text-4xl font-bold font-mono",
                    "Armor {CHARACTER().armor.iter().count().separate_with_commas()}"
                }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 text-xl font-bold py-1 px-3 rounded h-full border",
                    onclick: move |_| CHARACTER.write().armor.push(Armor::default()),
                    "+"
                }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 font-bold py-1 px-4 rounded h-full border",
                    onclick: move |_| show.set(!show()),
                    if show() {
                        "Hide"
                    } else {
                        "Show"
                    }
                }
            }
            if show() {
                div { class: "flex flex-wrap border rounded-lg p-2 gap-4 justify-center content-center items-start",
                    for (i, _) in CHARACTER().armor.iter().enumerate() {
                        RenderArmorPiece { index: i }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderArmorPiece(index: usize) -> Element {
    let Some(a) = CHARACTER().armor.get(index).cloned() else {
        return rsx! {};
    };

    rsx! {
        div { class: "flex flex-1 flex-col border p-2 rounded-lg min-w-[310px] space-y-2",
            // Name + delete
            div { class: "flex w-full justify-center items-center text-2xl space-x-2",
                input {
                    class: "flex flex-grow font-mono text-lg text-center border-spacing-1 border rounded-lg min-w-10 p-2",
                    r#type: "text",
                    placeholder: "Armor Name",
                    value: "{a.name}",
                    oninput: move |evt| {
                        CHARACTER.write().armor[index].name = evt.value();
                    },
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                    onclick: move |_| {
                        std::mem::drop(CHARACTER.write().armor.remove(index));
                    },
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsTrash,
                    }
                }
            }

            // Reductions
            div { class: "flex flex-col border rounded-lg justify-center place-items-center space-y-2 py-2",
                span { class: "w-full font-mono text-xl text-center", "Reductions" }
                div { class: "flex flex-wrap space-x-2 ",
                    div { class: "flex flex-row gap-2 justify-center items-center",
                        span { class: "font-mono text-lg", "Flat:" }
                        input {
                            class: "w-16 border rounded-lg p-2 text-center",
                            r#type: "number",
                            value: i64::from(a.flat_reduction),
                            oninput: move |evt| {
                                CHARACTER.write().armor[index].flat_reduction =
                                    evt.value().parse::<i32>().unwrap_or(0);
                            },
                        }
                    }
                    div { class: "flex flex-row gap-2 justify-center items-center",
                        span { class: "font-mono text-lg", "Percent:" }
                        input {
                            class: "w-16 border rounded-lg p-2 text-center",
                            r#type: "number",
                            value: i64::from(a.pct_reduction),
                            min: 0,
                            max: 100,
                            oninput: move |evt| {
                                CHARACTER.write().armor[index].pct_reduction =
                                    evt.value().parse::<i32>().unwrap_or(0);
                            },
                        }
                        span { class: "font-mono text-lg", "%" }
                    }
                }
            }

            // Notes
            textarea {
                id: "armor-notes-{index}",
                class: "w-full max-w-full border rounded-lg p-2 font-mono text-lg resize-none overflow-hidden",
                style: "min-height: 2.75rem",
                placeholder: "Notes",
                value: "{a.notes}",
                onmounted: move |_| async move {
                    let _ = document::eval(&auto_resize_js(&format!("armor-notes-{index}"), true))
                        .await;
                },
                oninput: move |evt| {
                    CHARACTER.write().armor[index].notes = evt.value();
                    let _ = document::eval(&auto_resize_js(&format!("armor-notes-{index}"), false));
                },
            }
        }
    }
}

// Talents

#[component]
fn RenderTalents() -> Element {
    let mut show = use_signal(|| false);
    rsx! {
        div { class: "flex w-full flex-col gap-2",
            div { class: "flex flex-row justify-center items-center py-2 gap-4",
                h2 { class: "text-center text-4xl font-bold font-mono",
                    "Talents {CHARACTER().talents.iter().count().separate_with_commas()}"
                }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 text-xl font-bold px-3 rounded h-full border",
                    onclick: move |_| CHARACTER.write().talents.push(Talent::default()),
                    "+"
                }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 font-bold py-1 px-4 rounded h-full border",
                    onclick: move |_| show.set(!show()),
                    if show() {
                        "Hide"
                    } else {
                        "Show"
                    }
                }
            }
            if show() {
                div { class: "flex flex-wrap border rounded-lg p-2 gap-4 justify-center content-center items-start",
                    for (i, _) in CHARACTER().talents.iter().enumerate() {
                        RenderTalent { index: i }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderTalent(index: usize) -> Element {
    let Some(t) = CHARACTER().talents.get(index).cloned() else {
        return rsx! {};
    };

    rsx! {
        div { class: "flex flex-1 flex-col border p-2 rounded-lg min-w-[350px] space-y-2",
            // Name + AP cost + delete
            div { class: "flex w-full justify-center items-center text-2xl space-x-2",
                input {
                    class: "flex flex-grow font-mono text-lg text-center border-spacing-1 border rounded-lg min-w-10 p-2",
                    r#type: "text",
                    placeholder: "Talent Name",
                    value: "{t.name}",
                    oninput: move |evt| {
                        CHARACTER.write().talents[index].name = evt.value();
                    },
                }
                span { class: "font-mono text-lg", "AP:" }
                input {
                    class: "w-14 border rounded-lg p-2 text-center",
                    r#type: "number",
                    value: i64::try_from(t.ap_cost).unwrap_or_default(),
                    min: 0,
                    oninput: move |evt| {
                        CHARACTER.write().talents[index].ap_cost =
                            evt.value().parse::<usize>().unwrap_or(0);
                    },
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                    onclick: move |_| {
                        std::mem::drop(CHARACTER.write().talents.remove(index));
                    },
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsTrash,
                    }
                }
            }

            // Required skill
            div { class: "inline-flex justify-center content-center items-center justify-items-center space-x-1",
                span { class: "font-mono text-lg", "Req. skill:" }
                input {
                    class: "flex flex-grow font-mono text-lg text-center border-spacing-1 border rounded-lg min-w-10 p-2",
                    r#type: "text",
                    placeholder: "None",
                    value: "{t.required_skill}",
                    oninput: move |evt| {
                        CHARACTER.write().talents[index].required_skill = evt.value();
                    },
                }
            }

            // Effect
            textarea {
                id: "talent-effect-{index}",
                class: "w-full max-w-full border rounded-lg p-2 font-mono text-lg resize-none overflow-hidden",
                style: "min-height: 2.75rem",
                placeholder: "Effects",
                value: "{t.description}",
                onmounted: move |_| async move {
                    let _ = document::eval(&auto_resize_js(&format!("talent-effect-{index}"), true))
                        .await;
                },
                oninput: move |evt| {
                    CHARACTER.write().talents[index].description = evt.value();
                    let _ = document::eval(
                        &auto_resize_js(&format!("talent-effect-{index}"), false),
                    );
                },
            }
        }
    }
}

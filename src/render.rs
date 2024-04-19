use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::{BsDice6, BsTrash, BsX}, Icon};

use crate::{
    character::{Stat, Character, Item, Quality, Quirk, QuirkCategory},
    dice::{roll_stat, RollResult}
};

#[component(no_case_check)]
pub fn render_rolls(state: Signal<(bool, Option<Stat>)>) -> Element {
    // Stat being passed must be given as a Some(Stat) otherwise the app will crash.
    let stat = state().1.clone().unwrap();
    // Create a state for the dice results
    let mut dice_results: Signal<Option<RollResult>> = use_signal(|| None);
    // Create a state for advantage and disadvantage
    let mut advantage = use_signal(|| 0);
    let mut disadvantage = use_signal(|| 0);
    rsx! {
        div { class: "z-10 fixed flex justify-center content-center max-w-[80%] w-96 h-fit border text-white border-white bg-slate-800 m-auto left-0 right-0 top-0 bottom-0 rounded-lg",
            // Close button
            div { class: "z-20 absolute right-0 px-2 py-2",
                div {
                    class: "bg-slate-950 hover:bg-slate-700 rounded",
                    onclick: move |_| {
                        state
                            .with_mut(|state| {
                                state.0 = false;
                                state.1 = None;
                            });
                    },
                    Icon { width: 35, height: 35, fill: "red", icon: BsX }
                }
            }
            div { class: "content-center justify-items-center",
                // Stat
                div { class: "content-center",
                    // Stat Name
                    div { class: "flex justify-center px-2 py-2",
                        h2 { class: "w-fit text-2xl text-center font-mono px-2 py-2 bg-slate-900 rounded border",
                            "{stat.name}"
                        }
                    }
                    // Quality + Quantity
                    div { class: "inline-flex w-full justify-center justify-items-center content-center",
                        div { class: "text-xl justify-center font-mono px-2 py-2", "{stat.quality}" }
                        div { class: "text-xl justify-center font-mono px-2 py-2", "{stat.quantity}" }
                        // Rolling
                        div { class: "justify-center",
                            button {
                                class: "max-w-fit font-mono place-self-center bg-slate-900 hover:bg-slate-600 py-2 px-2 rounded",
                                onclick: move |_| {
                                    dice_results
                                        .with_mut(|results| {
                                            *results = Some(
                                                roll_stat(&stat, advantage(), disadvantage()),
                                            );
                                        });
                                },
                                "Roll!"
                            }
                        }
                    }
                    // Advantage + Disadvantage
                    div { class: "grid grid-cols-2 justify-center content-even",
                        div { class: "grid grid-cols-1 px-2 py-2",
                            div { class: "px-1 py-1 items-center justify-center bg-green-950 rounded border",
                                div { class: "font-mono text-center", "Advantage" }
                                div { class: "flex justify-center",
                                    input {
                                        class: "w-12 border rounded-lg py-1 px-1",
                                        r#type: "number",
                                        value: i64::try_from(advantage()).unwrap_or_default(),
                                        oninput: move |evt| {
                                            advantage.set(evt.value().parse::<usize>().unwrap_or(0));
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "grid grid-cols-1 px-2 py-2",
                            div { class: "px-1 py-1 items-center justify-center bg-red-950 rounded border",
                                div { class: "font-mono text-center", "Disadvantage" }
                                div { class: "flex justify-center",
                                    input {
                                        class: "w-12 border rounded-lg py-1 px-1",
                                        r#type: "number",
                                        value: i64::try_from(disadvantage()).unwrap_or_default(),
                                        oninput: move |evt| {
                                            disadvantage.set(evt.value().parse::<usize>().unwrap_or(0));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(results) = dice_results() {
                    div { class: "font-mono justify-center",
                        div { class: "flex justify-center",
                            // Successes
                            div { class: "text-center text-green-600 px-2 py-2",
                                "Successes: {results.successes}"
                            }
                            // Failures
                            div { class: "text-center text-red-600 px-2 py-2",
                                "Failures: {results.failures}"
                            }
                        }
                        div { class: "px-2 py-2 text-lg text-center", "Results" },
                        // Results
                        div { class: "px-2 py-2",
                            div { class: "px-1 py-1 flex flex-wrap content-around justify-center text-center border bg-slate-900",
                                for r in results.results.iter() {
                                    div { class: "px-1 py-1",
                                        if *r >= stat.quality as u8 {
                                            div { class: "px-1 text-green-500 bg-slate-800 rounded", "{r}" }
                                        } else {
                                            div { class: "px-1 text-red-600 bg-slate-950 rounded", "{r}" }
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

#[component(no_case_check)]
pub fn render_character(
    character: Signal<Character>,
    dice_roll_state: Signal<(bool, Option<Stat>)>,
) -> Element {
    rsx!{
        div { class: "flex content-center items-center justify-center",
            div { class: "font-mono text-xl px-2 py-2", "Name:" }
            input {
                class: "border-spacing-1 border rounded-lg px-2 py-2",
                value: "{character.read().name}",
                oninput: move |evt| {
                    character.write().name = evt.value().clone();
                }
            }
            div { class: "font-mono text-xl px-2 py-2", "Stock:" }
            input {
                class: "border-spacing-1 border rounded-lg px-2 py-2",
                value: "{character.read().stock}",
                oninput: move |evt| {
                    character.write().stock = evt.value().clone();
                }
            }
        }

        div { class: "flex flex-wrap",
            div { class: "w-[748px] flex-auto justify-items-center justify-center",
                h2 { class: "py-4 text-center text-4xl font-bold font-mono",
                    "Stats {character.read().stats.iter().map(|stat| stat.quantity).sum::<usize>()}"
                }
                div { class: "flex justify-center justify-items-center content-center",
                    div { class: "grid grid-cols-2 gap-4 justify-center justify-items-center content-center max-w-5xl",
                        for (i , stat) in character.read().stats.iter().enumerate() {
                            div { class: "flex flex-col border border-spacing-2 items-center justify-center justify-items-center px-2 py-2 rounded-lg",
                                div { class: "inline-flex items-center justify-center py-2 px-2 w-auto",
                                    div { class: "font-mono text-center text-2xl py-2 px-2", "{stat.name}" }
                                    div { class: "py-2 px-2",
                                        button { class: "bg-slate-900 hover:bg-slate-600",
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
                                            character.with_mut(|character| {
                                                character.stats[i].quantity = evt.value().parse::<usize>().unwrap_or(0);
                                            });
                                        }
                                    },
                                    div { class: "font-mono text-lg py-2 px-2", "Checks:" },
                                    input { class: "w-16 border rounded-lg py-2 px-2",
                                        r#type:"number",
                                        value: i64::try_from(stat.checks.unwrap_or_default()).unwrap_or_default(),
                                        oninput: move |evt| {
                                            character.with_mut(|character| {
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
                        onclick: move |_| character.write().skills.push(Stat::new("New Skill!".into())),
                        "+ Add Skill"
                    }
                }
                div { class: "flex justify-center",
                    div { class: "grid grid-cols-2 gap-4 justify-items-center max-w-5xl",
                    for (i , skill) in character.read().skills.iter().enumerate() {
                            div { class: "flex flex-col border border-spacing-2 px-2 py-2 rounded-lg",
                                div { class: "flex justify-center content-center items-center justify-items-center text-2xl py-2 px-2",
                                    div { class: "flex px-2 py-2",
                                        input { class: "w-40 font-mono text-lg text-center border-spacing-1 border rounded-lg py-2 px-2",
                                            r#type:"text",
                                            value: "{skill.name}",
                                            oninput: move |evt| {
                                                character.write().skills[i].name = evt.value().to_string();
                                            }
                                        }
                                    }
                                    div { class: "px-2 py-2",
                                        button { class: "bg-slate-900 hover:bg-slate-600",
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
                                    div { class: "px-2 py-2 rounded-lg",
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
                                            character.with_mut(|character| {
                                                character.skills[i].quantity = evt.value().parse::<usize>().unwrap_or(0);
                                            });
                                        }
                                    },
                                    div { class: "font-mono text-lg py-2 px-2", "Checks:" },
                                    input { class: "w-16 border rounded-lg py-2 px-2",
                                        r#type:"number",
                                        value: i64::try_from(skill.checks.unwrap_or(0)).unwrap_or_default(),
                                        oninput: move |evt| {
                                            character.with_mut(|character| {
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

            div { class: "w-[1108px] flex-auto justify-items-center justify-center content-center",
                div { class: "flex justify-center content-center items-center",
                    h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                        "Argos"
                    }
                }
                div { class: "flex justify-center content-center items-center py-2 px-2",
                    textarea {
                        class: "rounded-lg w-2/3 py-2 px-2 bg-black text-white border border-white",
                        value: "{character.read().argos}",
                        oninput: move |evt| character.write().argos = evt.value().to_string()
                    }
                }

                div { class: "flex justify-center content-center items-center",
                    h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                        "Quirks"
                    }
                    button {
                        class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded",
                        onclick: move |_| character.write().quirks.push(Quirk::default()),
                        "+ Add Quirk"
                    }
                }

                div { class: "flex justify-center",
                    div { class: "grid grid-cols-2 gap-4 justify-items-center max-w-5xl",
                        for (i , quirk) in character.read().quirks.iter().enumerate() {
                            div { class: "w-[504px] border border-spacing-2 px-3 py-3 rounded-lg",
                                div { class: "flex justify-center content-center items-center justify-items-center text-2xl py-2 px-2 w-full",
                                    div { class: "flex",
                                        input { class: "w-44 font-mono text-lg text-center border-spacing-1 border rounded-lg py-2 px-2",
                                            r#type:"text",
                                            value: "{quirk.name}",
                                            oninput: move |evt| {
                                                character.write().quirks[i].name = evt.value().to_string();
                                            }
                                        }
                                    }
                                    div { class: "inline-flex justify-center content-center items-center justify-items-center px-2 py-2",
                                        select { class: "font-mono text-lg border rounded-lg py-2 px-2",
                                            onchange: move |evt| {
                                                character.with_mut(|character| {
                                                    character.quirks[i].category = match evt.value().parse::<usize>().unwrap() {
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
                                            character.write().quirks[i].description = evt.value().to_string();
                                        }
                                    }
                                }
                                div { class: "grid grid-cols-2 py-2 px-2",
                                    div { class: "inline-flex font-mono text-xl justify-center content-center items-center",
                                        div { class: "font-mono text-xl px-4",
                                            "Boons",
                                        }
                                        button { class: "bg-slate-900 hover:bg-slate-500 text-lg text-white font-bold py-1 px-4 rounded",
                                            onclick: move |_| character.with_mut(|character| 
                                                character.quirks[i].boons.push("New Boon!".into())
                                            ),
                                            "+ Boon"
                                        }
                                    }
                                    div { class: "inline-flex font-mono text-xl justify-center content-center items-center",
                                        div { class: "font-mono text-xl px-4",
                                            "Flaws",
                                        }
                                        button { class: "bg-slate-900 hover:bg-slate-500 text-lg text-white font-bold py-1 px-4 rounded",
                                            onclick: move |_| character.with_mut(|character|
                                                character.quirks[i].flaws.push("New Flaw!".into())
                                            ),
                                            "+ Flaw"
                                        }
                                    }
                                    div { class: "w-auto items-center justify-items-center",
                                        for (j, boon) in quirk.boons.iter().enumerate() {
                                            div { class: "inline-flex w-full justify-center items-start justify-items-center px-2 py-2",
                                                textarea { class: "text-mono w-full content-center justify-center border-spacing-1 border rounded-lg py-2 px-2 bg-black text-white",
                                                    value: "{boon}",
                                                    oninput: move |evt| character.write().quirks[i].boons[j] = evt.value().to_string()
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
                                        }
                                    }
                                    div { class: "w-auto items-center justify-items-center",
                                        for (j, flaw) in quirk.flaws.iter().enumerate() {
                                            div { class: "inline-flex w-full justify-center items-start justify-items-center px-2 py-2",
                                                textarea { class: "text-mono w-auto content-center justify-center border-spacing-1 border rounded-lg py-2 px-2 bg-black text-white",
                                                    value: "{flaw}",
                                                    oninput: move |evt| character.write().quirks[i].flaws[j] = evt.value().to_string()
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "w-[704px] flex-auto justify-center",
                div { class: "flex justify-center content-center items-center",
                    h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                        "Inventory"
                    }
                    button {
                        class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded",
                        onclick: move |_| character.write().inventory.push(Item::default()),
                        "+ Add Item"
                    }
                }
                div { class: "flex justify-center",
                    div { class: "grid grid-cols-2 gap-4 justify-center justify-items-center max-w-2xl",
                        for (i , item) in character.read().inventory.iter().enumerate() {
                            div { class: "justify-center content-center items-center justify-items-center border border-spacing-2 px-3 py-3 top-2 bottom-2 left-2 right-2 rounded-lg",
                                div { class: "inline-flex items-center content-center",
                                    div {
                                        input { class: "w-44 font-mono text-lg text-center border-spacing-1 border rounded-lg py-2 px-2",
                                            r#type:"text",
                                            value: "{item.name}",
                                            oninput: move |evt| {
                                                character.write().inventory[i].name = evt.value().to_string();
                                            }
                                        }
                                    }
                                    div {
                                        input { class: "w-16 border rounded-lg py-2 px-2",
                                            r#type:"number",
                                            value: i64::try_from(item.quantity).unwrap_or_default(),
                                            oninput: move |evt| {
                                                character.with_mut(|character| {
                                                    character.inventory[i].quantity = evt.value().parse::<usize>().unwrap_or(0);
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
                        }
                    }
                }
            }
        }
    }
}

/// The main application.
pub fn app() -> Element {
    let character = use_signal(Character::new);

    let dice_roll_state: Signal<(bool, Option<Stat>)> = use_signal(|| (false, None));

    let arrata_style = r"
    body { background-color: black; color: white; }
    input { background-color: black; color: white; }
    select { background-color: black; color: white; }
    ";

    let rat_path = if cfg!(target_family = "wasm") {
        "rat.png"
    } else {
        "public/rat.png"
    };

    rsx! {
        style { "{arrata_style}" }

        div { class: "px-5 py-5 origin-center justify-center self-center items-center content-center flex",
            // Arrata logo
            img {
                // Arrata logo
                class: "w-24 h-24 md:w-28 md:h-auto md:rounded-none rounded-full mr-10",
                src: rat_path,
                alt: "",
                width: 300,
                height: 300
            }

            h1 { class: "text-center text-9xl font-mono font-extrabold", "ARRATA" }
        }

        br {}

        character_io { character: character }

        br {}

        render_character { character: character, dice_roll_state: dice_roll_state }

        if dice_roll_state().0 {
            match &dice_roll_state().1 {
                Some(_) => rsx!(render_rolls { state: dice_roll_state }),
                None    => rsx!(""),
            }
        }
    }
}

#[component(no_case_check)]
fn character_io(character: Signal<Character>) -> Element {
    rsx!{
        div { class: "px-5 py-5 font-mono origin-center justify-center text-center self-center items-center content-center flex space-x-3",
            if cfg!(feature = "desktop") {
                button {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| character().write_to_file().unwrap(),
                    "Save Character"
                }
                button {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| {
                        let new_character = Character::from_file();
                        match new_character {
                            Ok(c) => character.set(c),
                            Err(e) => match e.kind() {
                                std::io::ErrorKind::Other => (),
                                _ => panic!("{e:?}"),
                            }
                        }
                    },
                    "Load Character"
                }
            } else {
                "Character Saving/Loading is disabled for this platform."
            }
        }
    }
}

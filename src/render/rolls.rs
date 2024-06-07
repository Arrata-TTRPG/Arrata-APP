use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsX, Icon};

use arrata_lib::{roll_stat, RollResult};

use crate::DICE_ROLL_STATE;

#[component]
pub(crate) fn RenderRolls() -> Element {
    // Stat being passed must be given as a Some(Stat) otherwise the app will crash.
    let stat = DICE_ROLL_STATE().1.clone().unwrap();
    // Create a state for the dice results
    let mut dice_results: Signal<Option<RollResult>> = use_signal(|| None);
    // Create a state for advantage and disadvantage
    let mut advantage = use_signal(|| 0);
    let mut disadvantage = use_signal(|| 0);
    rsx! {
        div { class: "z-10 fixed flex justify-center content-center max-w-[80%] w-96 h-fit border text-white border-white bg-slate-800 m-auto left-0 right-0 top-0 bottom-0 rounded-lg",
            // Close button
            div { class: "z-20 absolute right-0 p-2",
                div {
                    class: "bg-slate-950 hover:bg-slate-700 rounded",
                    onclick: move |_| {
                        DICE_ROLL_STATE
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
                    div { class: "flex justify-center p-2",
                        h2 { class: "w-fit text-2xl text-center font-mono p-2 bg-slate-900 rounded border",
                            "{stat.name}"
                        }
                    }
                    // Quality + Quantity
                    div { class: "inline-flex w-full justify-center justify-items-center content-center",
                        div { class: "text-xl justify-center font-mono p-2", "{stat.quality}" }
                        div { class: "text-xl justify-center font-mono p-2", "{stat.quantity}" }
                        // Rolling
                        div { class: "justify-center",
                            button {
                                class: "max-w-fit font-mono place-self-center bg-slate-900 hover:bg-slate-600 p-2 rounded",
                                onclick: move |_| {
                                    dice_results
                                        .with_mut(|results| {
                                            *results = Some(roll_stat(&stat, advantage(), disadvantage()));
                                        });
                                },
                                "Roll!"
                            }
                        }
                    }
                    // Advantage + Disadvantage
                    div { class: "grid grid-cols-2 justify-center content-even",
                        div { class: "grid grid-cols-1 p-2",
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
                        div { class: "grid grid-cols-1 p-2",
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
                            div { class: "text-center text-green-600 p-2",
                                "Successes: {results.successes}"
                            }
                            // Failures
                            div { class: "text-center text-red-600 p-2", "Failures: {results.failures}" }
                        }
                        div { class: "p-2 text-lg text-center", "Results" }
                        // Results
                        div { class: "p-2",
                            div { class: "px-1 py-1 flex flex-wrap content-around justify-center text-center border bg-slate-900",
                                for r in results.results.iter() {
                                    div { class: "px-1 py-1",
                                        if *r >= stat.quality as u8 {
                                            div { class: "px-1 text-green-500 bg-slate-800 rounded",
                                                "{r}"
                                            }
                                        } else {
                                            div { class: "px-1 text-red-600 bg-slate-950 rounded",
                                                "{r}"
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

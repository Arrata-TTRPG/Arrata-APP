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
        div { class: "z-10 fixed flex flex-col items-center max-w-[80%] max-h-[80%] h-fit w-96 overflow-hidden border text-white border-white bg-slate-800 m-auto left-0 right-0 top-0 bottom-0 rounded-lg gap-2 pt-2",
            // Close button
            div { class: "z-20 absolute right-0 top-0 p-2",
                div {
                    class: "bg-slate-950 hover:bg-slate-700 rounded cursor-pointer",
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
            // Stat Name
            h2 { class: "text-2xl text-center font-mono p-2 bg-slate-900 rounded border",
                "{stat.name}"
            }
            // Quality + Quantity
            div { class: "inline-flex w-full justify-center place-items-center content-center gap-2",
                p { class: "text-xl font-mono", "{stat.quality}" }
                p { class: "text-xl font-mono", "{stat.quantity}" }
                // Rolling
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
            // Advantage + Disadvantage
            div { class: "grid grid-cols-2 justify-center content-even gap-4",
                div { class: "p-1 items-center justify-center bg-green-950 rounded border",
                    div { class: "font-mono text-center", "Advantage" }
                    div { class: "flex justify-center",
                        input {
                            class: "w-12 border rounded-lg p-1 appearance-none",
                            r#type: "number",
                            value: "{advantage()}",
                            min: 0,
                            max: i64::MAX,
                            oninput: move |evt| advantage.set(evt.value().parse::<usize>().unwrap_or(0))
                        }
                    }
                }
                div { class: "p-1 items-center justify-center bg-red-950 rounded border",
                    div { class: "font-mono text-center", "Disadvantage" }
                    div { class: "flex justify-center",
                        input {
                            class: "w-12 border rounded-lg p-1",
                            r#type: "number",
                            value: "{disadvantage()}",
                            min: 0,
                            max: i64::MAX,
                            oninput: move |evt| disadvantage.set(evt.value().parse::<usize>().unwrap_or(0))
                        }
                    }
                }
            }

            if let Some(results) = dice_results() {
                div { class: "flex flex-col font-mono justify-center max-h-full overflow-y-hidden pt-2",
                    div { class: "flex justify-center",
                        // Successes
                        div { class: "text-center text-green-600 p-2",
                            "Successes: {results.successes}"
                        }
                        // Failures
                        div { class: "text-center text-red-600 p-2", "Failures: {results.failures}" }
                    }
                    h2 { class: "p-2 text-lg text-center", "Results" }
                    // Results
                    div { class: "p-1 overflow-scroll flex flex-wrap justify-center max-h-full w-full gap-1 border rounded bg-slate-900",
                        for r in results.results.iter() {
                            div {
                                class: format!(
                                    "px-1 bg-slate-950 rounded {}",
                                    if *r >= stat.quality as u8 { "text-green-500" } else { "text-red-600" },
                                ),
                                "{r}"
                            }
                        }
                    }
                }
            }
        }
    }
}

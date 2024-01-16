use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsX, Icon};

use crate::{
    character::structs::*,
    dice::{roll::roll_stat, structs::*},
};

#[component(no_case_check)]
pub fn render_rolls<'a>(cx: Scope, state: &'a UseState<(bool, Option<Stat>)>) -> Element {
    // Stat being passed must be given as a Some(Stat) otherwise the app will crash.
    let stat = state.get().1.clone().unwrap();
    // Create a state for the dice results
    let dice_results: &UseState<Option<DiceResult>> = use_state(cx, || None);
    // Create a state for advantage and disadvantage
    //let advantage = use_state(cx, || 0);
    //let disadvantage = use_state(cx, || 0);
    cx.render(rsx! {
        div { class: "z-10 fixed flex justify-center content-center max-w-[80%] w-96 h-fit border text-white border-white bg-slate-800 m-auto left-0 right-0 top-0 bottom-0 rounded-lg",
            // Close button
            div { class: "z-20 absolute right-0 px-2 py-2",
                div {
                    class: "bg-slate-900 hover:bg-slate-600 rounded",
                    onclick: move |_| {
                        state
                            .with_mut(|state| {
                                state.0 = false;
                                state.1 = None;
                            });
                    },
                    Icon { width: 50, height: 50, fill: "red", icon: BsX }
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
                        div { class: "text-xl justify-center font-mono px-2 py-2",
                            "{stat.quality}"
                        }
                        div { class: "text-xl justify-center font-mono px-2 py-2",
                            "{stat.quantity}"
                        }
                        // Rolling
                        div { class: "justify-center",
                            button {
                                class: "max-w-fit font-mono place-self-center bg-slate-900 hover:bg-slate-600 py-2 px-2 rounded",
                                onclick: move |_| {
                                    dice_results
                                        .with_mut(|results| {
                                            *results = Some(roll_stat(stat.clone()));
                                        });
                                },
                                "Roll!"
                            }
                        }
                    }
                }

                if let Some(results) = dice_results.get() {
                    rsx!(
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
                                        rsx! (
                                            div { class: "px-1 py-1",
                                                if *r >= stat.quality as u8 {
                                                    rsx!( div { class: "px-1 text-green-500 bg-slate-800 rounded", "{r}" } )
                                                } else {
                                                    rsx!( div { class: "px-1 text-red-600 bg-slate-950 rounded", "{r}" } )
                                                }
                                            }
                                        )
                                    }
                                }
                            }
                        }
                    )
                }
            }
        }
    })
}

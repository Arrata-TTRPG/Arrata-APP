//! The dice roller dialog.

use dioxus::prelude::*;
use thousands::Separable;

use arrata_lib::{RollResult, roll_stat};

use crate::components::shared::{Btn, CloseBtn, Col, NumberInput, Row};
use crate::{Ui, UiStoreExt};

/// Rolls the stat held in [`Ui::roll`], showing successes, failures and dice.
#[component]
pub fn RollDialog() -> Element {
    let mut roll = use_context::<Store<Ui>>().roll();
    let mut results = use_signal::<Option<RollResult>>(|| None);
    let advantage = use_signal(|| 0usize);
    let disadvantage = use_signal(|| 0usize);

    let Some(stat) = roll.cloned() else {
        return rsx! {};
    };
    let threshold = stat.quality as u8;

    rsx! {
        div { class: "dialog",
            CloseBtn { onclick: move |_| roll.set(None) }

            h2 { class: "roll__stat", "{stat.name}" }

            Row {
                p { "{stat.quality}" }
                p { "{stat.quantity.separate_with_commas()}" }
                Btn {
                    onclick: move |_| {
                        results.set(Some(roll_stat(&stat, advantage(), disadvantage())));
                    },
                    "Roll!"
                }
            }

            div { class: "roll__odds",
                div { class: "advantage",
                    div { "Advantage" }
                    NumberInput { value: advantage }
                }
                div { class: "disadvantage",
                    div { "Disadvantage" }
                    NumberInput { value: disadvantage }
                }
            }

            if let Some(result) = results() {
                Col { class: "scroll-y",
                    div { class: "roll__tally",
                        div { class: "success",
                            "Successes: {result.successes.separate_with_commas()}"
                        }
                        div { class: "failure",
                            "Failures: {result.failures.separate_with_commas()}"
                        }
                    }
                    if !result.results.is_empty() {
                        h2 { "Results" }
                        div { class: "roll__dice",
                            for (index , die) in result.results.iter().enumerate() {
                                div {
                                    key: "{index}",
                                    class: if *die >= threshold { "roll__die roll__die--hit" } else { "roll__die" },
                                    "{die}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

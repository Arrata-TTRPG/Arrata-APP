mod character;

use character::*;
use dioxus::prelude::*;

/// The main application.
pub fn app(cx: Scope) -> Element {
    let character = use_state(cx, Character::new);

    cx.render(rsx! {
        figure {
            class: "md:flex bg-slate-100 rounded-xl p-8 md:p-0 dark:bg-slate-800",
            blockquote {
                p {
                    class: "text-lg font-medium",
                    "Wow very Arrata!"
                }
            }
        }

        div {
            "ARRATA"
        }

        div {
            input {
                value: "{character.name}",
                oninput: move |evt| {
                    character.make_mut().name = evt.value.clone();
                },
            }
            input {
                value: "{character.stock}",
                oninput: move |evt| {
                    character.make_mut().stock = evt.value.clone();
                },
            }
        },

        b { "Stats " },

        button {
            onclick: move |_| character.make_mut().stats.push(Stat::new()),
            "Add Stat",
        },

        for (i,stat) in character.get().stats.iter().enumerate() {
            rsx!(
                div {
                    input {
                        value: "{stat.name.clone()}",
                        oninput: move |evt| {
                        character.with_mut(|character| {
                            character.stats[i].name = evt.value.to_string();
                        });
                        }
                    },
                    ":", 
                    input {
                        r#type:"number",
                        value: stat.quantity as f64,
                        oninput: move |evt| {
                            character.with_mut(|character| {
                            character.stats[i].quantity = evt.value.parse::<u64>().unwrap_or(0);
                            });
                        }
                    },
                    select {
                        onchange: move |evt| {
                            character.with_mut(|character| {
                                character.stats[i].quality = match evt.value.parse::<u64>().unwrap() {
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
                    " Checks:", 
                    input {
                        r#type:"number",
                        value: stat.checks.unwrap_or(0) as f64,
                        oninput: move |evt| {
                            character.with_mut(|character| {
                                character.stats[i].checks = Some(evt.value.parse::<u64>().unwrap_or(0));
                            });
                        }
                    },
                 }
             )
        }

        div {
            button {
                onclick: move |_| write_to_file(character.get().clone()),
                "Save Character"
            },
            button {
                onclick: move |_| {
                    character.set(read_from_file());
                },
                "Load Character"
            },
        }
    })
}

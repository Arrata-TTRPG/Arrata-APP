use std::{
    fs::File,
    io::{BufWriter, Write},
};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct Character {
    pub name: String,
    pub stock: String,
    pub stats: Vec<Stat>,
    pub skills: Vec<Stat>,
}

impl Character {
    pub fn new() -> Character {
        Character {
            name: "John Arrata".to_string(),
            stock: "Human".to_string(),
            stats: Vec::new(),
            skills: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Props, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Stat {
    name: Option<String>,
    quality: Option<Quality>,
    quantity: Option<u64>,
    checks: Option<u64>,
}

impl Stat {
    pub fn new() -> Stat {
        Stat {
            name: Some("Stat".to_string()),
            quality: Some(Quality::Basic),
            quantity: Some(0),
            checks: Some(0),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
enum Quality {
    Basic,
    Adept,
    Superb,
}

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Basic => write!(f, "Basic"),
            Quality::Adept => write!(f, "Adept"),
            Quality::Superb => write!(f, "Superb"),
        }
    }
}

fn write_to_file(character: Character) {
    let f = File::create("char.arrata").unwrap();

    let mut writer = BufWriter::new(f);

    let character_serde = serde_json::to_string_pretty(&character).unwrap();

    writer.write_all(character_serde.as_bytes()).unwrap();
}

pub fn app(cx: Scope) -> Element {
    let character = use_state(cx, Character::new);
    
    cx.render(rsx! {
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

        b { "Stats" },

        button {
            onclick: move |_| character.make_mut().stats.push(Stat::new()),
            "Add Stat",
        },
        
        for (i,stat) in character.get().stats.iter().enumerate() {
            rsx!(
               div {
                    input {
                        value: "{stat.name.clone().unwrap()}",
                        oninput: move |evt| {
                        character.with_mut(|character| {
                            character.stats[i].name = Some(evt.value.to_string());
                        });
                        }
                    },
                    ":", 
                    input {
                        r#type:"number",
                        value: stat.quantity.clone().unwrap_or(0) as f64,
                        oninput: move |evt| {
                            character.with_mut(|character| {
                            character.stats[i].quantity = Some(evt.value.parse::<u64>().unwrap_or(0));
                            });
                        }
                    },
                    select {
                        onchange: move |evt| {
                            character.with_mut(|character| {
                                character.stats[i].quality = Some(match evt.value.parse::<u64>().unwrap() {
                                    0 => Quality::Basic,
                                    1 => Quality::Adept,
                                    2 => Quality::Superb,
                                    _ => Quality::Basic,
                                })
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
                        value: stat.checks.clone().unwrap_or(0) as f64,
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
            }
        }
    })
}

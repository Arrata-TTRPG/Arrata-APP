// character.rs
// All structs and enums relating to characters.

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use dioxus_free_icons::icons::bs_icons::BsDice6;
use dioxus_free_icons::Icon;

use dioxus::prelude::*;
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};

/* Structs and Enums */

/// A struct containing all info about a character.
#[derive(Serialize, Deserialize, Clone, Props, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
    pub name: String,
    pub stock: String,
    pub stats: Vec<Stat>,
    pub skills: Vec<Stat>,
    pub quirks: Vec<Quirk>,
    pub argos: String,
    pub inventory: Vec<Item>,
}

impl Character {
    pub fn new() -> Character {
        Character {
            name: "John Arrata".to_string(),
            stock: "Human".to_string(),
            stats: vec![
                Stat::new("Will".into()),
                Stat::new("Perception".into()),
                Stat::new("Conscious".into()),
                Stat::new("Power".into()),
                Stat::new("Speed".into()),
                Stat::new("Forte".into()),
            ],
            skills: Vec::new(),
            quirks: Vec::new(),
            argos: String::new(),
            inventory: Vec::new(),
        }
    }

    /// Write a character to their relevant `.arrata` file.
    ///
    /// # Inputs
    /// `character` - The character to write to the file.
    ///
    /// # Outputs
    ///
    /// `Result<(), std::io::Error>` - None if successful or
    /// the given IO error if one is encountered.
    ///
    /// Characters written will be written as
    /// "`{character.name}.arrata`"
    ///
    /// This method only writes if we have relevant permissions.
    pub fn write_to_file(&self) -> Result<(), std::io::Error> {
        // Grab the current file path; should never throw unless we don't have file permissions
        let path: Option<std::path::PathBuf> = FileDialog::new().show_open_single_dir().unwrap();
        let path: std::path::PathBuf = match path {
            Some(path) => path,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid path given",
                ))
            }
        };

        // {character.name}.arrata
        let suffix = self.name.clone() + ".arrata";

        let f = File::create(path.to_str().unwrap().to_owned() + "/" + &suffix).unwrap();

        let mut writer = BufWriter::new(f);

        // Serialize the character with serde and write to file
        let character_serde = serde_json::to_string_pretty(self)?;
        writer.write_all(character_serde.as_bytes())?;

        Ok(())
    }

    pub fn from_file() -> Result<Self, std::io::Error> {
        // Grab the current file path; should never throw unless we don't have file permissions
        let path: Option<std::path::PathBuf> = FileDialog::new().show_open_single_file().unwrap();
        let path: std::path::PathBuf = match path {
            Some(path) => path,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid path given",
                ))
            }
        };

        let file = File::open(path)?;
        let module: Character = serde_json::from_reader(file)?;
        Ok(module)
    }
}

/// A struct for Stats.
///
/// `checks` is optional as some stats don't
/// require checks to function.
#[derive(Serialize, Deserialize, Clone, Props, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Stat {
    pub name: String,
    pub quality: Quality,
    pub quantity: u64,
    pub checks: Option<u64>,
}

impl Stat {
    pub fn new(name: String) -> Stat {
        Stat {
            name,
            quality: Quality::Basic,
            quantity: 1,
            checks: Some(0),
        }
    }
}

/// A struct for Quality. Determines the
/// lower bound for rolls.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Quality {
    Basic = 4,
    Adept = 3,
    Superb = 2,
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

/// A struct for Quirks. Boons
/// and flaws are optional as some
/// Quirks are purely cosmetic/neutral.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Quirk {
    pub name: String,
    pub category: QuirkCategory,
    pub boons: Option<Vec<String>>,
    pub flaws: Option<Vec<String>>,
}

/// The Quirk category.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum QuirkCategory {
    Ethos,
    Pathos,
    Logos,
    Uncategorized,
}

/// A struct for items.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub name: String,
    pub quantity: u64,
    pub description: Option<String>,
}

#[component(no_case_check)]
pub fn render_character<'a>(cx: Scope, character: &'a UseState<Character>) -> Element {
    cx.render(rsx!{
        div { class: "flex content-center items-center justify-center text-lg",
            div { class: "px-2 py-2", "Name:" }
            input {
                class: "border-spacing-1 border rounded-lg",
                value: "{character.name}",
                oninput: move |evt| {
                    character.make_mut().name = evt.value.clone();
                }
            }
            div { class: "px-2 py-2", "Stock:" }
            input {
                class: "border-spacing-1 border rounded-lg",
                value: "{character.stock}",
                oninput: move |evt| {
                    character.make_mut().stock = evt.value.clone();
                }
            }
        }

        h2 { class: "py-4 text-center text-4xl font-bold font-mono", "Stats" }

        div {
            class: "flex justify-center",
            div { class: "grid grid-cols-2 gap-4 justify-items-center content-stretch max-w-5xl",
                for (i , stat) in character.get().stats.iter().enumerate() {
                    rsx!(
                        div {
                            class: "border border-spacing-2 px-3 py-3 rounded-lg",
                            div { class: "font-mono text-center text-2xl py-2 px-2 w-full", "{stat.name.clone()}" },
                            div {
                                class: "inline-flex w-full justify-center content-center items-center justify-items-center",
                                select {
                                    class: "font-mono py-2 px-2",
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
                                input {
                                    class: "w-12 border rounded-lg py-2 px-2",
                                    r#type:"number",
                                    value: stat.quantity as f64,
                                    oninput: move |evt| {
                                        character.with_mut(|character| {
                                            character.stats[i].quantity = evt.value.parse::<u64>().unwrap_or(0);
                                        });
                                    }
                                },
                                div { class: "py-2 px-2", "Checks:" },
                                input {
                                    class: "w-12 border rounded-lg py-2 px-2",
                                    r#type:"number",
                                    value: stat.checks.unwrap_or(0) as f64,
                                    oninput: move |evt| {
                                        character.with_mut(|character| {
                                            character.stats[i].checks = Some(evt.value.parse::<u64>().unwrap_or(0));
                                        });
                                    }
                                },
                                div { class: "py-2 px-2", "Roll:" },
                                button {
                                    class: "py-2 px-2",
                                    Icon {
                                        width: 20,
                                        height: 20,
                                        fill: "white",
                                        icon: BsDice6
                                    }
                                    // TODO: onclick event!
                                }
                            }
                        }
                    )
                }
            }
        }

        div { class: "flex justify-center content-center items-center",
            h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                "Skills"
            }
            button {
                onclick: move |_| character.make_mut().skills.push(Stat::new("New Skill!".into())),
                class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded",
                "+ Add Skill"
            }
        }

        div {
            class: "flex justify-center",
            div { class: "grid grid-cols-2 gap-4 justify-items-center content-stretch max-w-5xl",
                for (i , skill) in character.get().skills.iter().enumerate() {
                    rsx!(
                        div {
                            class: "border border-spacing-2 px-3 py-3 rounded-lg",
                            div {
                                class: "justify-center content-center text-2xl py-2 px-1 w-full",
                                input {
                                    class: "text-mono text-center content-center justify-center w-auto border-spacing-1 border rounded-lg py-2 px-2",
                                    r#type:"text",
                                    value: "{skill.name.clone()}",
                                    oninput: move |evt| {
                                        character.make_mut().skills[i].name = evt.value.to_string();
                                    }
                                }
                                button {
                                    onclick: move |_| { let _ = character.make_mut().skills.remove(i); },
                                    class: "text-mono bg-slate-900 hover:bg-slate-600 text-white font-bold py-1 px-2 space-x-5 rounded",
                                    "Delete"
                                }
                            }
                            div {
                                class: "inline-flex w-full justify-center content-center items-center justify-items-center",
                                select {
                                    class: "font-mono py-2 px-1",
                                    onchange: move |evt| {
                                        character.with_mut(|character| {
                                            character.skills[i].quality = match evt.value.parse::<u64>().unwrap() {
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
                                input {
                                    class: "w-12 border rounded-lg py-2 px-1",
                                    r#type:"number",
                                    value: skill.quantity as f64,
                                    oninput: move |evt| {
                                        character.with_mut(|character| {
                                            character.skills[i].quantity = evt.value.parse::<u64>().unwrap_or(0);
                                        });
                                    }
                                },
                                div { class: "py-2 px-2", "Checks:" },
                                input {
                                    class: "w-12 border rounded-lg py-2 px-1",
                                    r#type:"number",
                                    value: skill.checks.unwrap_or(0) as f64,
                                    oninput: move |evt| {
                                        character.with_mut(|character| {
                                            character.skills[i].checks = Some(evt.value.parse::<u64>().unwrap_or(0));
                                        });
                                    }
                                },
                                div { class: "py-2 px-1", "Roll:" },
                                button {
                                    class: "py-2 px-1",
                                    Icon {
                                        width: 20,
                                        height: 20,
                                        fill: "white",
                                        icon: BsDice6
                                    }
                                    // TODO: onclick event!
                                }
                            }
                        }
                    )
                }
            }
        }
    })
}

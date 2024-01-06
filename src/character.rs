// character.rs
// All structs and enums relating to characters.

use std::{
    fs::File,
    io::{BufWriter, Write},
};

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
            stats: Vec::new(),
            skills: Vec::new(),
            quirks: Vec::new(),
            argos: String::new(),
            inventory: Vec::new(),
        }
    }

    
    /// Write a character to their relevant `.arrata` file.
    ///
    /// `character` - The character to write to the file.
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
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path given")),
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
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path given")),
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
    pub fn new() -> Stat {
        Stat {
            name: "Stat".to_string(),
            quality: Quality::Basic,
            quantity: 0,
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

#[component]
pub fn RenderCharacter<'a>(cx: Scope, character: &'a UseState<Character>) -> Element {
    cx.render(rsx!{
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

        h2 {
            class: "text-center",
            "Stats" 
        },

        br {}

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
    })
}
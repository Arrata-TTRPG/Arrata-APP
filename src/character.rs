// character.rs
// All structs and enums relating to characters.

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use dioxus::prelude::Props;
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};

/* Structs and Enums */

/// A struct containing all info about a character.
#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Quirk {
    pub name: String,
    pub category: QuirkCategory,
    pub boons: Option<Vec<String>>,
    pub flaws: Option<Vec<String>>,
}

/// The Quirk category.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum QuirkCategory {
    Ethos,
    Pathos,
    Logos,
    Uncategorized,
}

/// A struct for items.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub name: String,
    pub quantity: u64,
    pub description: Option<String>,
}

/* Utility Functions */

/// Write a character to their relevant `.arrata` file.
///
/// `character` - The character to write to the file.
///
/// Characters written will be written as
/// "`{character.name}.arrata`"
///
/// This method only writes if we have relevant permissions.
pub fn write_to_file(character: Character) {
    // Grab the current file path; should never throw unless we don't have file permissions
    let path: Option<std::path::PathBuf> = FileDialog::new().show_open_single_dir().unwrap();
    let path: std::path::PathBuf = match path {
        Some(path) => path,
        None => return,
    };

    // {character.name}.arrata
    let suffix = character.name.clone() + ".arrata";

    let f = File::create(path.to_str().unwrap().to_owned() + "/" + &suffix).unwrap();

    let mut writer = BufWriter::new(f);

    // Serialize the character with serde and write to file
    let character_serde = serde_json::to_string_pretty(&character).unwrap();
    writer.write_all(character_serde.as_bytes()).unwrap();
}

pub fn read_from_file() -> Character {
    todo!()
}

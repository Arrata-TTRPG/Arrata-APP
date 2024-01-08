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

impl Default for Character {
    fn default() -> Self {
        Self::new()
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
    pub quantity: usize,
    pub checks: Option<usize>,
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

impl Quirk {
    pub fn new(name: String) -> Self {
        Self {
            name,
            category: QuirkCategory::Ethos,
            boons: Some(vec![]),
            flaws: Some(vec![]),
        }
    }
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
    pub quantity: usize,
    pub description: Option<String>,
}

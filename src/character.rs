// character.rs
// All structs and enums relating to characters.

use serde::{Deserialize, Serialize};

/* Structs and Enums */

/// A struct containing all info about a character.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
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
    #[must_use]
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

    #[cfg(feature = "desktop")]
    /// Read a character from a file.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if there is an error reading the file.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the read character if successful.
    pub fn from_file() -> Result<Self, std::io::Error> {
        use native_dialog::FileDialog;
        use std::fs::File;

        // Grab the current file path; should never throw unless we don't have file permissions
        let path: std::path::PathBuf = match FileDialog::new().show_open_single_file() {
            Ok(p) => match p {
                Some(p) => p,
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No file selected",
                    ))
                }
            },
            Err(_) => {
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

    #[cfg(not(feature = "desktop"))]
    /// Read a character from a file.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if there is an error reading the file.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the read character if successful.
    pub fn from_file() -> Result<Self, std::io::Error> {
        todo!("This is not yet implemented!")
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
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Stat {
    pub name: String,
    pub quality: Quality,
    pub quantity: usize,
    pub checks: Option<usize>,
}

impl Stat {
    #[must_use]
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
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
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
    pub description: String,
    pub boons: Vec<String>,
    pub flaws: Vec<String>,
}

impl Quirk {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            category: QuirkCategory::Ethos,
            boons: vec![],
            flaws: vec![],
        }
    }
}

impl Default for Quirk {
    fn default() -> Self {
        Self::new("New Quirk!".into())
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
    pub description: String,
}

impl Item {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            quantity: 0,
            description: String::new(),
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new("New Item!".into())
    }
}

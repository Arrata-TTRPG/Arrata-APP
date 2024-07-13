use std::sync::OnceLock;

use arrata_lib::Character;

use crate::{CHARACTER, VERSION};

pub static LOCATION: OnceLock<std::path::PathBuf> = OnceLock::new();

/// Sets the directory path.
///
/// # Panics
///
/// This function will panic if the provided path is invalid.
pub fn set_directory(path: std::path::PathBuf) {
    LOCATION.set(path).unwrap();
}

/// Writes the character data to a file.
///
/// # Panics
///
/// This function will panic if it fails to write the character data to the file.
pub fn write_character(name: &str) {
    if let Some(path) = LOCATION.get() {
        let version = format!("{}.{}", VERSION().major, VERSION().minor);
        let character_file = format!("{name}-{version}.arrata");
        let file_path = path.join(character_file);
        if let Ok(file) = std::fs::write(file_path, serde_json::to_string(&CHARACTER()).unwrap()) {
            println!("Character saved: {file:?}");
        }
    }
}

pub fn read_character(name: &str) -> Option<Character> {
    if let Some(path) = LOCATION.get() {
        let character_file = format!("{name}-{VERSION}.arrata");
        let file_path = path.join(character_file);
        if let Ok(file) = std::fs::read(file_path) {
            if let Ok(character) = serde_json::from_slice(&file) {
                return Some(character);
            }
        }
    }

    None
}

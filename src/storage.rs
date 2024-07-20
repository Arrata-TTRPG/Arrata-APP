use arrata_lib::Character;

use arrata_lib::Quirk;

#[cfg(feature = "desktop")]
pub static LOCATION: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();

/// Sets the directory path.
///
/// # Panics
///
/// This function will panic if the provided path is invalid.
#[cfg(feature = "desktop")]
pub fn set_directory(path: std::path::PathBuf) {
    LOCATION.set(path).unwrap();
}

/// Writes the character data to a file.
///
/// # Panics
///
/// This function will panic if it fails to write the character data to the file.
#[cfg(feature = "desktop")]
pub fn write_character(name: &str, character: &Character) {
    if let Some(path) = LOCATION.get() {
        let data = bitcode::encode(character);
        let character_file = format!("{name}.arrata");
        let file_path = path.join(character_file);
        if let Ok(file) = std::fs::write(file_path, data) {
            println!("Character saved: {file:?}");
        }
    }
}

/// Reads the character data from a file.
#[cfg(feature = "desktop")]
pub fn read_character(name: &str) -> Option<Character> {
    if let Some(path) = LOCATION.get() {
        let character_file = format!("{name}.arrata");
        let file_path = path.join(character_file);
        if let Ok(file) = std::fs::read(file_path.clone()) {
            if let Ok(character) = bitcode::decode(&file) {
                return Some(character);
            }
        } else {
            println!("Failed to read file {}", file_path.clone().display());
        }
    }

    None
}

#[cfg(feature = "desktop")]
pub fn write_quirks(quirks: Vec<Quirk>, key: &str) {
    if let Some(path) = LOCATION.get() {
        let data = bitcode::encode(&quirks);
        let quirk_file = format!("{key}.arrata");
        let file_path = path.join(quirk_file);
        if let Ok(file) = std::fs::write(file_path, data) {
            println!("Quirks saved: {file:?}");
        }
    }
}

#[cfg(feature = "desktop")]
pub fn read_quirks(key: &str) -> Option<Vec<Quirk>> {
    if let Some(path) = LOCATION.get() {
        let quirk_file = format!("{key}.quirks");
        let file_path = path.join(quirk_file);
        if let Ok(file) = std::fs::read(file_path.clone()) {
            if let Ok(quirks) = bitcode::decode(&file) {
                return Some(quirks);
            }
        } else {
            println!("Failed to read file {}", file_path.clone().display());
        }
    }

    None
}

#[cfg(feature = "web")]
use gloo_storage::{LocalStorage, Storage};

/// Writes a key-value pair to the persistent storage.
///
/// # Arguments
///
/// * `key` - The key of the pair.
/// * `value` - The value of the pair.
///
/// # Errors
///
/// Returns an error if the key-value pair could not be written to the persistent storage.
///
/// This error is the stringified form of the one given by `gloo_storage`.
///
/// # Panics
///
/// Panics if the character cannot be written to a `String` by `serde_json`.
#[cfg(feature = "web")]
pub fn write_character(key: &str, character: &Character) {
    let character = serde_json::to_string(character).unwrap();
    LocalStorage::set(key, character).unwrap();
}

/// Reads the value associated with the given key from the persistent storage.
///
/// # Arguments
///
/// * `key` - The key of the pair.
///
/// # Returns
///
/// * `Option<String>` - The value associated with the key. `None` if an error occured.
///
/// # Panics
///
/// This function will panic if the value associated with the key could not be read from the persistent storage.
#[cfg(feature = "web")]
#[must_use]
pub fn read_character(key: &str) -> Option<Character> {
    if let Ok(data) = &LocalStorage::get::<String>(key) {
        Some(serde_json::from_str(data).unwrap())
    } else {
        None
    }
}

#[cfg(feature = "web")]
pub fn read_quirks(key: &str) -> Option<Vec<Quirk>> {
    if let Ok(data) = &LocalStorage::get::<String>(key) {
        Some(serde_json::from_str(data).unwrap())
    } else {
        None
    }
}

#[cfg(feature = "web")]
pub fn write_quirks(quirks: Vec<Quirk>, key: &str) {
    let quirks = serde_json::to_string(&quirks).unwrap();
    LocalStorage::set(key, quirks).unwrap();
}

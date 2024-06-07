#![warn(clippy::all, clippy::pedantic)]

pub mod render;

use arrata_lib::character::{Character, Stat};

use dioxus::prelude::GlobalSignal;

/// The `GlobalSignal` for the `Character`.
pub(crate) static CHARACTER: GlobalSignal<Character> = GlobalSignal::new(Character::new);
/// The `GlobalSignal` for rolling dice.
pub(crate) static DICE_ROLL_STATE: GlobalSignal<(bool, Option<Stat>)> =
    GlobalSignal::new(|| (false, None));

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
pub fn character_from_file() -> Result<Character, std::io::Error> {
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
pub fn character_from_file() -> Result<Character, std::io::Error> {
    todo!("This is not yet implemented!")
}
use arrata_lib::{Character, Quirk};

// ── Desktop ───────────────────────────────────────────────────────────────────

#[cfg(feature = "desktop")]
pub static LOCATION: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();

/// Sets the app data directory used for all reads and writes.
///
/// # Panics
///
/// Panics if called more than once.
#[cfg(feature = "desktop")]
pub fn set_directory(path: std::path::PathBuf) {
    LOCATION.set(path).unwrap();
}

/// Writes every character in the roster to its own `character-N.arrata` file,
/// plus a `character-count` file so reads know how many to expect.
///
/// # Panics
///
/// Panics if any character cannot be encoded.
#[cfg(feature = "desktop")]
pub fn write_characters(characters: &[Character]) {
    let Some(path) = LOCATION.get() else { return };
    for (i, character) in characters.iter().enumerate() {
        let data = bitcode::encode(character);
        let _ = std::fs::write(path.join(format!("character-{i}.arrata")), data);
    }
    let _ = std::fs::write(path.join("character-count"), characters.len().to_string());
}

/// Reads all characters from disk. Returns `None` if no roster has been saved yet.
#[cfg(feature = "desktop")]
#[must_use]
pub fn read_characters() -> Option<Vec<Character>> {
    let path = LOCATION.get()?;
    let count: usize = std::fs::read_to_string(path.join("character-count"))
        .ok()?
        .trim()
        .parse()
        .ok()?;
    let characters: Vec<Character> = (0..count)
        .filter_map(|i| {
            let bytes = std::fs::read(path.join(format!("character-{i}.arrata"))).ok()?;
            bitcode::decode(&bytes).ok()
        })
        .collect();
    if characters.is_empty() {
        None
    } else {
        Some(characters)
    }
}

/// Writes pre-made quirks to `<key>.quirks`.
///
/// # Panics
///
/// Panics if encoding fails.
#[cfg(feature = "desktop")]
pub fn write_quirks(quirks: &[Quirk], key: &str) {
    let Some(path) = LOCATION.get() else { return };
    let data = bitcode::encode(quirks);
    let _ = std::fs::write(path.join(format!("{key}.quirks")), data);
}

/// Reads pre-made quirks from `<key>.quirks`. Returns `None` if not found.
#[cfg(feature = "desktop")]
#[must_use]
pub fn read_quirks(key: &str) -> Option<Vec<Quirk>> {
    let path = LOCATION.get()?;
    let bytes = std::fs::read(path.join(format!("{key}.quirks"))).ok()?;
    bitcode::decode(&bytes).ok()
}

// ── Web ───────────────────────────────────────────────────────────────────────

#[cfg(feature = "web")]
use gloo_storage::{LocalStorage, Storage};

/// Writes the full character roster to `localStorage` as JSON.
///
/// # Panics
///
/// Panics if serialization fails.
#[cfg(feature = "web")]
pub fn write_characters(characters: &[Character]) {
    LocalStorage::set("characters", serde_json::to_string(characters).unwrap()).unwrap();
}

/// Reads the character roster from `localStorage`. Returns `None` if not found.
#[cfg(feature = "web")]
#[must_use]
pub fn read_characters() -> Option<Vec<Character>> {
    serde_json::from_str(&LocalStorage::get::<String>("characters").ok()?).ok()
}

/// Writes pre-made quirks to `localStorage` under `key`.
///
/// # Panics
///
/// Panics if serialization fails.
#[cfg(feature = "web")]
pub fn write_quirks(quirks: &[Quirk], key: &str) {
    LocalStorage::set(key, serde_json::to_string(quirks).unwrap()).unwrap();
}

/// Reads pre-made quirks from `localStorage` under `key`. Returns `None` if not found.
#[cfg(feature = "web")]
#[must_use]
pub fn read_quirks(key: &str) -> Option<Vec<Quirk>> {
    serde_json::from_str(&LocalStorage::get::<String>(key).ok()?).ok()
}


use arrata_lib::Character;
use base64::prelude::*;
use dioxus::prelude::eval;

use crate::{CHARACTER, VERSION};

/// Writes the character data to a file.
///
/// # Panics
///
/// This function will panic if it fails to write the character data to the file.
pub async fn write_character(name: &str) {
    println!("Writing character: {name}");

    let character = CHARACTER();
    let encoded = BASE64_STANDARD.encode(serde_json::to_string(&character).unwrap());

    let eval = eval(
        "
        const fileName = await dioxus.recv();
        const fileData = await dioxus.recv();
        localStorage.setItem(fileName, fileData);
    ",
    );

    eval.send(serde_json::Value::String(format!("{}-{}", name, VERSION())))
        .unwrap();
    eval.send(serde_json::Value::String(encoded)).unwrap();

    eval.await.unwrap();
}

/// Reads the character data from a file.
///
/// # Panics
///
/// This function will panic if it fails to read the character data from the file.
pub async fn read_character(name: &str) -> Option<Character> {
    let mut eval = eval(
        "
        const fileName = await dioxus.recv();
        const fileData = localStorage.getItem(fileName);
        dioxus.send(fileData);
    ",
    );

    let version = format!("{}-{}", VERSION().major, VERSION().minor);
    eval.send(serde_json::Value::String(format!("{name}-{version}")))
        .unwrap();

    let data = eval.recv().await.unwrap();

    log::info!("Got value: {data:?}");

    if let Some(data) = data.as_str() {
        if let Ok(decoded) = BASE64_STANDARD.decode(data.as_bytes()) {
            if let Ok(character) = serde_json::from_slice(&decoded) {
                return Some(character);
            }
        }
    }

    None
}

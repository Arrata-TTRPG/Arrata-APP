use std::{
    fs::File,
    io::{BufWriter, Write},
};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Character {
    pub name: String,
    pub stock: String,
}

impl Character {
    pub fn new() -> Character {
        Character {
            name: "John Arrata".to_string(),
            stock: "Human".to_string(),
        }
    }
}

fn write_to_file(character: Character) {
    let f = File::create("char.arrata").unwrap();

    let mut writer = BufWriter::new(f);

    let character_serde = serde_json::to_string(&character).unwrap();

    writer.write(character_serde.as_bytes()).unwrap();
}

pub fn app(cx: Scope) -> Element {

    let character = use_state(cx, || Character::new());

    cx.render(rsx! {
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
        button {
            onclick: move |_event| write_to_file(character.get().clone()),
            "Save Character"
        }
    })
}

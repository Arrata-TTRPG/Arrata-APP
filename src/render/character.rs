use dioxus::prelude::*;

use crate::{
    character::Character,
    render::{RenderInventory, RenderQuirks, RenderStats},
    CHARACTER,
};

#[component]
pub(crate) fn RenderCharacter() -> Element {
    rsx! {
        div { class: "flex content-center items-center justify-center",
            div { class: "font-mono text-xl px-2 py-2", "Name:" }
            input {
                class: "border-spacing-1 border rounded-lg px-2 py-2",
                value: "{CHARACTER().name}",
                oninput: move |evt| {
                    CHARACTER.write().name = evt.value();
                }
            }
            div { class: "font-mono text-xl px-2 py-2", "Stock:" }
            input {
                class: "border-spacing-1 border rounded-lg px-2 py-2",
                value: "{CHARACTER().stock}",
                oninput: move |evt| {
                    CHARACTER.write().stock = evt.value();
                }
            }
        }

        div { class: "flex flex-wrap",
            RenderStats {}
            RenderQuirks {}
            RenderInventory {}
        }
    }
}

#[cfg(feature = "desktop")]
#[component]
pub(crate) fn CharacterIO() -> Element {
    rsx! {
        div { class: "px-5 py-5 font-mono origin-center justify-center text-center self-center items-center content-center flex space-x-3",
            button {
                class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| {
                    use native_dialog::FileDialog;
                    use std::{fs::File, io::{BufWriter, Write}};
                    let path: std::path::PathBuf = match FileDialog::new().show_open_single_dir() {
                        Ok(p) => {
                            match p {
                                Some(p) => p,
                                None => return,
                            }
                        }
                        Err(_) => return,
                    };
                    let suffix = CHARACTER().name.clone() + ".arrata";
                    let f = File::create(path.to_str().unwrap().to_owned() + "/" + &suffix).unwrap();
                    let mut writer = BufWriter::new(f);
                    let character_serde = serde_json::to_string_pretty(self).unwrap();
                    writer.write_all(character_serde.as_bytes()).unwrap();
                },
                "Save Character"
            }
            button {
                class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| {
                    let new_character = Character::from_file();
                    match new_character {
                        Ok(c) => *CHARACTER.write() = c,
                        Err(e) => {
                            match e.kind() {
                                std::io::ErrorKind::Other => {}
                                _ => panic!("{e:?}"),
                            }
                        }
                    }
                },
                "Load Character"
            }
        }
    }
}

#[cfg(feature = "web")]
pub(crate) fn CharacterIO() -> Element {
    rsx! {
        div { class: "px-5 py-5 font-mono origin-center justify-center text-center self-center items-center content-center flex space-x-3",
            a {
                class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                href: "data:text/json;charset=utf-8,{urlencoding::encode(&serde_json::to_string(&CHARACTER()).unwrap())}",
                download: "{CHARACTER().name}.arrata",
                "Save Character"
            }
            button {
                label {
                    class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                    r#for: "character-file-input",
                    "Load Character"
                }
            }
            input {
                r#type: "file",
                class: "hidden",
                id: "character-file-input",
                accept: ".arrata",
                onchange: |evt| {
                    let mut character = CHARACTER.write();
                    async move {
                        if let Some(file_engine) = &evt.files() {
                            let files = file_engine.files();
                            for file_name in files {
                                if let Some(file) = file_engine.read_file_to_string(&file_name).await
                                {
                                    if let Ok(new_character) = serde_json::from_str::<
                                        Character,
                                    >(&file) {
                                        *character = new_character;
                                    }
                                }
                            }
                        }
                    }
                },
                "Load Character"
            }
        }
    }
}

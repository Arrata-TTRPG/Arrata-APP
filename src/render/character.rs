use dioxus::prelude::*;

use crate::{
    render::{RenderInventory, RenderQuirks, RenderStats},
    CHARACTER,
};

#[component]
pub(crate) fn RenderCharacter() -> Element {
    rsx! {
        div { class: "flex flex-wrap content-center items-center justify-center",
            div { class: "flex",
                h2 { class: "font-mono text-xl p-2", "Name:" }
                input {
                    class: "border-spacing-1 border rounded-lg p-2",
                    value: "{CHARACTER().name}",
                    oninput: move |evt| {
                        CHARACTER.write().name = evt.value();
                    }
                }
            }
            div { class: "flex",
                h2 { class: "font-mono text-xl p-2", "Stock:" }
                input {
                    class: "border-spacing-1 border rounded-lg p-2",
                    value: "{CHARACTER().stock}",
                    oninput: move |evt| {
                        CHARACTER.write().stock = evt.value();
                    }
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

#[cfg(not(any(feature = "desktop", feature = "web")))]
#[component]
pub(crate) fn CharacterIO() -> Element {
    rsx! {
        p { "How did you do this? You built the app without the Desktop or Web feature. Fool." }
    }
}

#[cfg(feature = "desktop")]
#[component]
pub(crate) fn CharacterIO() -> Element {
    use crate::character_from_file;
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
                    let character_serde = serde_json::to_string_pretty(&CHARACTER()).unwrap();
                    writer.write_all(character_serde.as_bytes()).unwrap();
                },
                "Save Character"
            }
            button {
                class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| {
                    let new_character = character_from_file();
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
#[component]
pub(crate) fn CharacterIO() -> Element {
    use arrata_lib::Character;
    rsx! {
        div { class: "px-5 py-5 font-mono origin-center justify-center text-center self-center items-center content-center flex space-x-3",
            a {
                class: "font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                href: "data:text/json;charset=utf-8,{urlencoding::encode(&serde_json::to_string(&CHARACTER()).unwrap())}",
                download: "{CHARACTER().name}.arrata",
                "Save Character"
            }
            label {
                class: "flex cursor-pointer font-mono text-xl bg-slate-900 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded",
                r#for: "character-file-input",
                "Load Character"
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

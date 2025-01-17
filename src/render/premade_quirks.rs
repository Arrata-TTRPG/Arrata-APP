use arrata_lib::{Quirk, QuirkCategory};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsSave, BsTrash, BsX},
    Icon,
};
use rfd::AsyncFileDialog;

use crate::{CHARACTER, PREMADE_QUIRKS, PREMADE_QUIRKS_MENU};

#[component]
pub fn RenderPremadeQuirkList() -> Element {
    let shown_categories = (
        use_signal(|| true),
        use_signal(|| true),
        use_signal(|| true),
    );

    rsx! {
        div { class: "z-10 fixed flex flex-col max-w-[90%] w-full min-h-14 h-[90%] border text-white border-white bg-slate-950 m-auto left-0 right-0 top-0 bottom-0 rounded-lg",
            // Close button
            div { class: "z-20 absolute right-0 top-0 p-2",
                div {
                    class: "bg-slate-950 hover:bg-slate-700 rounded cursor-pointer",
                    onclick: move |_| *PREMADE_QUIRKS_MENU.write() = false,
                    Icon {
                        width: 35,
                        height: 35,
                        fill: "red",
                        icon: BsX,
                    }
                }
            }

            div { class: "p-2 flex flex-wrap justify-center gap-x-4 gap-y-2",
                h1 { class: "text-center py-2 text-2xl font-bold font-mono", "Premade Quirks" }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 text-white font-mono font-bold flex px-2 h-12 items-center border rounded",
                    onclick: move |_| {
                        use_future(|| async move {
                            let file = AsyncFileDialog::new()
                                .set_title("Save .quirks Quirks file")
                                .set_file_name("quirks.quirks")
                                .save_file()
                                .await;
                            if let Some(f) = file {
                                let _ = f.write(&bitcode::encode(&PREMADE_QUIRKS())).await;
                            }
                        });
                    },
                    "Download Quirks"
                }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 text-white font-mono font-bold flex px-2 h-12 items-center border rounded",
                    onclick: move |_| {
                        spawn(async move {
                            let files = AsyncFileDialog::new()
                                .set_title("Load .quirks Quirks file")
                                .add_filter("type", &["quirks"])
                                .pick_files()
                                .await;
                            if let Some(files) = files {
                                for f in files {
                                    let quirks: Vec<Quirk> = bitcode::decode(&f.read().await).unwrap();
                                    PREMADE_QUIRKS.write().extend(quirks);
                                }
                                PREMADE_QUIRKS.write().sort_by(|a, b| a.name.cmp(&b.name));
                                PREMADE_QUIRKS.write().dedup();
                            }
                        });
                    },
                    "Load .quirks File"
                }
            }

            // Quirks
            div { class: "h-full gap-1 justify-center p-2 overflow-scroll",
                if PREMADE_QUIRKS().is_empty() {
                    p { class: "flex font-mono text-lg gap-2 place-items-center",
                        "No premade quirks available. Save some here with the"
                        Icon {
                            width: 18,
                            height: 18,
                            fill: "white",
                            icon: BsSave,
                        }
                        "save"
                        "button."
                    }
                }

                // Split quirks into categories
                div { class: "flex flex-col h-full lg:flex-row gap-2 overflow-y-scroll lg:pr-0 pr-4",
                    RenderPremadeQuirkCategory {
                        category: QuirkCategory::Ethos,
                        shown: shown_categories.0,
                    }
                    RenderPremadeQuirkCategory {
                        category: QuirkCategory::Pathos,
                        shown: shown_categories.1,
                    }
                    RenderPremadeQuirkCategory {
                        category: QuirkCategory::Logos,
                        shown: shown_categories.2,
                    }
                }
            }
        }
    }
}

#[component]
fn RenderPremadeQuirkCategory(category: QuirkCategory, shown: Signal<bool>) -> Element {
    rsx! {
        // Logos
        div { class: "flex flex-col lg:h-full h-2/3 gap-2 border rounded-lg p-1 w-full",
            div { class: "flex flex-wrap gap-2 justify-center items-center",
                h2 { class: "text-xl font-mono font-bold text-center", "{category}" }
                button {
                    class: "bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-2 border rounded",
                    onclick: move |_| {
                        shown.set(!shown());
                    },
                    if shown() {
                        "Hide"
                    } else {
                        "Show"
                    }
                }
            }
            if shown() {
                div { class: "flex flex-col w-full max-h-full gap-3 overflow-y-scroll pr-3",
                    for (index , quirk) in PREMADE_QUIRKS()
                        .into_iter()
                        .enumerate()
                        .filter(|(_, quirk)| quirk.category == category)
                    {
                        RenderPremadeQuirk { index, quirk }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderPremadeQuirk(index: usize, quirk: Quirk) -> Element {
    rsx! {
        div {
            class: "flex flex-col bg-slate-900 w-full h-fit p-1 border gap-2",
            key: index,
            // Name, add, and remove buttons
            div { class: "flex flex-wrap gap-2 justify-center place-items-center",
                h3 { class: "text-xl font-extrabold", "{quirk.name}" }
                button {
                    class: "flex bg-slate-900 hover:bg-slate-700 text-white font-bold py-1 px-2 border rounded",
                    onclick: move |_| {
                        CHARACTER
                            .with_mut(|character| {
                                character.quirks.push(quirk.clone());
                            });
                    },
                    "+ Add"
                }
                button {
                    class: "bg-red-950 hover:bg-red-600 p-1 border rounded-lg",
                    onclick: move |_| std::mem::drop(PREMADE_QUIRKS.write().remove(index)),
                    Icon {
                        width: 25,
                        height: 25,
                        fill: "white",
                        icon: BsTrash,
                    }
                }
            }

            // Description
            if !quirk.description.is_empty() {
                p { class: "font-mono text-base text-center px-1", "{quirk.description}" }
            }

            // Boons and flaws
            if !quirk.boons.is_empty() || !quirk.flaws.is_empty() {
                div { class: "grid grid-cols-2 h-full",
                    div { class: "flex flex-col gap-1 h-full",
                        h4 { class: "font-mono text-lg text-center", "Boons" }
                        ul { class: "list-disc list-inside items-start px-2",
                            for (index , boon) in quirk.boons.iter().enumerate() {
                                li {
                                    key: "{index}",
                                    class: "text-sm font-mono text-wrap",
                                    "{boon}"
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col gap-1 h-full",
                        h4 { class: "font-mono text-lg text-center", "Flaws" }
                        ul { class: "list-disc list-inside items-start px-2",
                            for (index , flaw) in quirk.flaws.iter().enumerate() {
                                li {
                                    key: "{index}",
                                    class: "text-sm font-mono text-wrap",
                                    "{flaw}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

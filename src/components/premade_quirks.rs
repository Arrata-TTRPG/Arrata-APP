//! Full-screen browser for saved and community quirks.

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use thousands::Separable;

use arrata_lib::{CharacterStoreExt, QuirkCategory};

use crate::components::{
    io::{download_quirks, pick_quirks},
    shared::{Btn, BtnKind, CloseBtn, DeleteBtn, Row},
};
use crate::{Ui, UiStoreExt, use_active_character};

/// How long the "added!" highlight stays lit.
const FLASH_MS: u32 = 700;

/// The three categories, in browser order.
const CATEGORIES: [QuirkCategory; 3] = [
    QuirkCategory::Ethos,
    QuirkCategory::Pathos,
    QuirkCategory::Logos,
];

/// The browser overlay: import/export controls above three category columns.
#[component]
pub fn QuirkBrowser() -> Element {
    let ui = use_context::<Store<Ui>>();
    let mut browser = ui.browser();
    let mut premade = ui.premade();

    rsx! {
        div { class: "sheet-overlay",
            CloseBtn { onclick: move |_| browser.set(false) }

            Row { class: "row--loose",
                h1 { "Premade Quirks" }
                Btn {
                    onclick: move |_| download_quirks(premade.read().as_slice()),
                    "Download Quirks"
                }
                Btn {
                    onclick: move |_| async move {
                        let loaded = pick_quirks().await;
                        premade.write().extend(loaded);
                        premade.write().sort_by(|a, b| a.name.cmp(&b.name));
                        premade.write().dedup();
                    },
                    "Load .quirks File"
                }
            }

            if premade.is_empty() {
                p { class: "label",
                    "No premade quirks available — save one from the sheet, or load a .quirks file."
                }
            }

            div { class: "browser__columns",
                for category in CATEGORIES {
                    BrowserColumn { key: "{category}", category }
                }
            }
        }
    }
}

/// One collapsible category column.
#[component]
fn BrowserColumn(category: QuirkCategory) -> Element {
    let premade = use_context::<Store<Ui>>().premade();
    let mut shown = use_signal(|| true);

    let indices: Vec<usize> = premade
        .read()
        .iter()
        .enumerate()
        .filter(|(_, quirk)| quirk.category == category)
        .map(|(index, _)| index)
        .collect();
    let count = indices.len().separate_with_commas();

    rsx! {
        div { class: "browser__column",
            Row {
                h4 { "{category}" }
                Btn {
                    kind: BtnKind::Small,
                    onclick: move |_| shown.toggle(),
                    if shown() { "Hide" } else { "Show" }
                }
                span { "{count} available" }
            }
            if shown() {
                div { class: "browser__entries",
                    for index in indices {
                        PremadeQuirk { key: "{index}", index }
                    }
                }
            }
        }
    }
}

/// A read-only preview of one saved quirk, with add-to-sheet and delete.
#[component]
fn PremadeQuirk(index: usize) -> Element {
    let mut premade = use_context::<Store<Ui>>().premade();
    let character = use_active_character();
    let mut flashing = use_signal(|| false);

    let Some(quirk) = premade.read().get(index).cloned() else {
        return rsx! {};
    };
    let added = quirk.clone();

    rsx! {
        div { class: "quirk-card",
            Row {
                Btn {
                    class: if flashing() { "btn--flash" } else { "" },
                    onclick: move |_| {
                        if flashing() {
                            return;
                        }
                        if let Some(character) = character {
                            character.quirks().push(added.clone());
                        }
                        spawn(async move {
                            flashing.set(true);
                            TimeoutFuture::new(FLASH_MS).await;
                            flashing.set(false);
                        });
                    },
                    "+ {quirk.name}"
                }
                DeleteBtn {
                    small: true,
                    onclick: move |_| {
                        premade.remove(index);
                    },
                }
            }

            if !quirk.description.is_empty() {
                p { class: "quirk-card__description", "{quirk.description}" }
            }

            if !quirk.boons.is_empty() || !quirk.flaws.is_empty() {
                div { class: "quirk-card__effects",
                    EffectColumn { title: "Boons", entries: quirk.boons.clone() }
                    EffectColumn { title: "Flaws", entries: quirk.flaws }
                }
            }
        }
    }
}

/// A titled, bulleted list of boons or flaws.
#[component]
fn EffectColumn(title: String, entries: Vec<String>) -> Element {
    rsx! {
        div {
            h4 { {title} }
            ul {
                for (index , entry) in entries.into_iter().enumerate() {
                    li { key: "{index}", {entry} }
                }
            }
        }
    }
}

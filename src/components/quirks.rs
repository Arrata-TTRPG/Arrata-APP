//! Middle column: argos, inspiration and the three quirk categories.

use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsSave;

use arrata_lib::{Character, CharacterStoreExt, Quirk, QuirkCategory};

use crate::components::shared::{
    Btn, BtnKind, Card, Col, DeleteBtn, Grid, IconBtn, NotesArea, NumberInput, Row, Section,
    TextInput, field,
};
use crate::{Ui, UiStoreExt};

/// The three categories, in sheet order.
const CATEGORIES: [QuirkCategory; 3] = [
    QuirkCategory::Ethos,
    QuirkCategory::Pathos,
    QuirkCategory::Logos,
];

/// Middle column of the sheet.
#[component]
pub fn RenderQuirks(character: Store<Character>) -> Element {
    let mut browser = use_context::<Store<Ui>>().browser();

    rsx! {
        div { class: "sheet__column",
            Section { title: "Argos",
                NotesArea {
                    value: character.argos(),
                    class: "textarea--argos",
                    placeholder: "TODO: Find purpose.",
                }
            }

            Inspiration { character }

            Row { class: "row--loose fill",
                h1 { "Quirks" }
                Btn { onclick: move |_| browser.set(true), "+ Load Premade Quirk" }
            }

            for category in CATEGORIES {
                QuirkCategorySection { key: "{category}", character, category }
            }
        }
    }
}

/// The three inspiration pools.
#[component]
fn Inspiration(character: Store<Character>) -> Element {
    let inspiration = character.inspiration();

    rsx! {
        Card { class: "fill",
            h1 { "Inspiration" }
            Row { class: "row--loose fill",
                Col { class: "grow",
                    h3 { "Ethos" }
                    NumberInput { value: field!(inspiration, ethos) }
                }
                Col { class: "grow",
                    h3 { "Pathos" }
                    NumberInput { value: field!(inspiration, pathos) }
                }
                Col { class: "grow",
                    h3 { "Logos" }
                    NumberInput { value: field!(inspiration, logos) }
                }
            }
        }
    }
}

/// Every quirk in one category. Quirks live in a single flat list on the
/// character, so this filters by category and edits through the real indices.
#[component]
fn QuirkCategorySection(character: Store<Character>, category: QuirkCategory) -> Element {
    let mut quirks = character.quirks();

    let indices: Vec<usize> = quirks
        .read()
        .iter()
        .enumerate()
        .filter(|(_, quirk)| quirk.category == category)
        .map(|(index, _)| index)
        .collect();

    let blank = Quirk {
        category: category.clone(),
        ..Quirk::default()
    };

    rsx! {
        Section {
            title: "{category}",
            small: true,
            count: indices.len(),
            onadd: move |_| quirks.push(blank.clone()),
            div { class: "card-grid quirk-list",
                for index in indices {
                    QuirkCard {
                        key: "{index}",
                        quirk: quirks.get(index).unwrap(),
                        ondelete: move |_| {
                            quirks.remove(index);
                        },
                    }
                }
            }
        }
    }
}

/// One quirk: name, description and its boon/flaw columns.
#[component]
fn QuirkCard(quirk: Store<Quirk>, ondelete: EventHandler<MouseEvent>) -> Element {
    let mut premade = use_context::<Store<Ui>>().premade();

    rsx! {
        Card {
            Row { class: "fill",
                TextInput {
                    value: field!(quirk, name),
                    class: "input--stat",
                    placeholder: "Quirk Name",
                }
                DeleteBtn { onclick: ondelete }
                IconBtn {
                    icon: BsSave,
                    onclick: move |_| premade.push(quirk()),
                }
            }
            NotesArea {
                value: field!(quirk, description),
                class: "textarea--flat",
                placeholder: "Get quirky with it.",
            }
            Grid { class: "grid--pair",
                Effects { quirk, boon: true }
                Effects { quirk, boon: false }
            }
        }
    }
}

/// The boon or flaw column of a quirk card.
#[component]
fn Effects(mut quirk: Store<Quirk>, boon: bool) -> Element {
    let count = if boon {
        quirk.read().boons.len()
    } else {
        quirk.read().flaws.len()
    };

    rsx! {
        Col { class: "col--tight col--top fill",
            Row {
                h4 { if boon { "Boons" } else { "Flaws" } }
                Btn {
                    kind: BtnKind::Small,
                    onclick: move |_| {
                        let mut quirk = quirk.write();
                        let list = if boon { &mut quirk.boons } else { &mut quirk.flaws };
                        list.push(String::new());
                    },
                    "+"
                }
            }
            for index in 0..count {
                Effect { key: "{index}", quirk, boon, index }
            }
        }
    }
}

/// A single boon or flaw entry.
#[component]
fn Effect(mut quirk: Store<Quirk>, boon: bool, index: usize) -> Element {
    // `Quirk` doesn't derive `Store`, and the entry is behind a runtime index,
    // so map straight through to the string the textarea edits.
    let value = quirk.map_mut(
        move |quirk| &(if boon { &quirk.boons } else { &quirk.flaws })[index],
        move |quirk| {
            &mut (if boon {
                &mut quirk.boons
            } else {
                &mut quirk.flaws
            })[index]
        },
    );

    rsx! {
        Row { class: "row--tight row--nowrap fill",
            NotesArea {
                value,
                class: "textarea--flat textarea--inline",
                placeholder: if boon { "Boon" } else { "Flaw" },
            }
            DeleteBtn {
                small: true,
                onclick: move |_| {
                    let mut quirk = quirk.write();
                    let list = if boon { &mut quirk.boons } else { &mut quirk.flaws };
                    list.remove(index);
                },
            }
        }
    }
}

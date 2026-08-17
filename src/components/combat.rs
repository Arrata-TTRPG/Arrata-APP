//! Right column: derived combat stats, then weapons, armor and talents.
//!
//! `Weapon`, `Armor` and `Talent` don't derive `Store`, so their fields are
//! reached with [`field!`](crate::components::shared::field) rather than
//! generated accessors.

use dioxus::prelude::*;
use thousands::Separable;

use arrata_lib::{
    Armor, Character, CharacterStoreExt, CoreStatsStoreExt, StatStoreExt, Talent, Weapon, combat,
};

use crate::components::shared::{
    Btn, BtnKind, Card, CardGrid, DeleteBtn, Dropdown, Field, NotesArea, NumberInput, Row, Section,
    SignedInput, TextInput, field,
};

/// The core stats a weapon can add to its damage.
const STAT_NAMES: [&str; 7] = [
    "Will",
    "Perception",
    "Conscious",
    "Power",
    "Speed",
    "Forte",
    "None",
];

/// Right column of the sheet.
#[component]
pub fn RenderCombat(character: Store<Character>) -> Element {
    rsx! {
        div { class: "sheet__column",
            CombatStats { character }
            Weapons { character }
            ArmorList { character }
            Talents { character }
        }
    }
}

/// Health, injury and action points. Maxima are derived from core stats.
#[component]
fn CombatStats(character: Store<Character>) -> Element {
    let stats = character.stats();
    let max_health = combat::max_health(stats.will().quantity()(), stats.forte().quantity()());
    let max_ap = combat::ap_cap(stats.speed().quantity()());

    let mut injury = character.injury();
    let action_points = use_signal(|| 0usize);

    rsx! {
        h1 { "Combat" }
        Row { class: "row--loose fill",
            Card { class: "grow",
                span { "Health" }
                div { class: "field",
                    NumberInput { value: character.current_health() }
                    p { class: "label", "/ {max_health.separate_with_commas()}" }
                }
            }
            Card { class: "grow",
                span { "Injury" }
                div { class: "field",
                    Btn {
                        kind: BtnKind::Add,
                        onclick: move |_| injury.set(injury().saturating_add(1)),
                        "+"
                    }
                    h2 { "{injury}" }
                    Btn {
                        kind: BtnKind::Add,
                        onclick: move |_| injury.set(injury().saturating_sub(1)),
                        "-"
                    }
                }
            }
            Card { class: "grow",
                span { "Action Points" }
                div { class: "field",
                    NumberInput { value: action_points, class: "grow" }
                    p { class: "label", "/ {max_ap.separate_with_commas()}" }
                }
            }
        }
    }
}

#[component]
fn Weapons(character: Store<Character>) -> Element {
    let mut weapons = character.weapons();

    rsx! {
        Section {
            title: "Weapons",
            small: true,
            count: weapons.len(),
            onadd: move |_| weapons.push(Weapon::default()),
            CardGrid {
                for (index , weapon) in weapons.iter().enumerate() {
                    WeaponCard {
                        key: "{index}",
                        weapon,
                        ondelete: move |_| {
                            weapons.remove(index);
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn WeaponCard(mut weapon: Store<Weapon>, ondelete: EventHandler<MouseEvent>) -> Element {
    rsx! {
        Card {
            Row { class: "fill",
                TextInput {
                    value: field!(weapon, name),
                    class: "input--stat",
                    placeholder: "Weapon Name",
                }
                DeleteBtn { onclick: ondelete }
            }
            Row { class: "row--loose fill",
                Field { label: "Skill:", class: "grow",
                    TextInput {
                        value: field!(weapon, skill),
                        class: "input--stat",
                        placeholder: "None",
                    }
                }
                Field { label: "Min:",
                    // Stored as `Option<String>`; an empty box means "no minimum".
                    input {
                        class: "input input--count",
                        r#type: "text",
                        placeholder: "B0",
                        value: "{weapon().skill_requirement.unwrap_or_default()}",
                        oninput: move |event| {
                            let requirement = event.value();
                            weapon.write().skill_requirement = (!requirement.is_empty())
                                .then_some(requirement);
                        },
                    }
                }
            }
            Row { class: "fill",
                Field { label: "Base dmg:",
                    SignedInput { value: field!(weapon, base_damage) }
                }
                Field { label: "+",
                    Dropdown {
                        value: field!(weapon, stat_modifier),
                        options: STAT_NAMES.map(String::from).to_vec(),
                    }
                }
            }
            NotesArea { value: field!(weapon, notes), placeholder: "Notes" }
        }
    }
}

#[component]
fn ArmorList(character: Store<Character>) -> Element {
    let mut armor = character.armor();

    rsx! {
        Section {
            title: "Armor",
            small: true,
            count: armor.len(),
            onadd: move |_| armor.push(Armor::default()),
            CardGrid {
                for (index , piece) in armor.iter().enumerate() {
                    ArmorCard {
                        key: "{index}",
                        armor: piece,
                        ondelete: move |_| {
                            armor.remove(index);
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ArmorCard(mut armor: Store<Armor>, ondelete: EventHandler<MouseEvent>) -> Element {
    rsx! {
        Card {
            Row { class: "fill",
                TextInput {
                    value: field!(armor, name),
                    class: "input--stat",
                    placeholder: "Armor Name",
                }
                DeleteBtn { onclick: ondelete }
            }
            h4 { "Reductions" }
            Row { class: "fill",
                Field { label: "Flat:",
                    SignedInput { value: field!(armor, flat_reduction) }
                }
                Field { label: "Percent:",
                    SignedInput { value: field!(armor, pct_reduction), min: 0, max: 100 }
                    p { class: "label", "%" }
                }
            }
            NotesArea { value: field!(armor, notes), placeholder: "Notes" }
        }
    }
}

#[component]
fn Talents(character: Store<Character>) -> Element {
    let mut talents = character.talents();

    rsx! {
        Section {
            title: "Talents",
            small: true,
            count: talents.len(),
            onadd: move |_| talents.push(Talent::default()),
            CardGrid {
                for (index , talent) in talents.iter().enumerate() {
                    TalentCard {
                        key: "{index}",
                        talent,
                        ondelete: move |_| {
                            talents.remove(index);
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn TalentCard(mut talent: Store<Talent>, ondelete: EventHandler<MouseEvent>) -> Element {
    rsx! {
        Card {
            Row { class: "fill",
                TextInput {
                    value: field!(talent, name),
                    class: "input--stat",
                    placeholder: "Talent Name",
                }
                Field { label: "AP:",
                    NumberInput { value: field!(talent, ap_cost) }
                }
                DeleteBtn { onclick: ondelete }
            }
            Field { label: "Req. skill:", class: "fill",
                TextInput {
                    value: field!(talent, required_skill),
                    class: "input--stat",
                    placeholder: "None",
                }
            }
            NotesArea { value: field!(talent, description), placeholder: "Effects" }
        }
    }
}

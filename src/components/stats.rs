//! Left column: core stats, skills and inventory.

use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsDice6;
use thousands::Separable;

use arrata_lib::{
    Character, CharacterStoreExt, CoreStatsStoreExt, Item, ItemStoreExt, Quality, Stat,
    StatStoreExt,
};

use crate::components::shared::{
    BtnKind, Card, CardGrid, DeleteBtn, Field, Grid, IconBtn, NumberInput, QualityInput, Row,
    Section, TextInput,
};
use crate::{Ui, UiStoreExt};

/// Left column of the sheet.
#[component]
pub fn RenderStats(character: Store<Character>) -> Element {
    rsx! {
        div { class: "sheet__column",
            CoreStats { character }
            Skills { character }
            Inventory { character }
        }
    }
}

/// The six fixed stats, with their combined effective total in the heading.
#[component]
fn CoreStats(character: Store<Character>) -> Element {
    let stats = character.stats();
    let core = [
        stats.will(),
        stats.perception(),
        stats.conscious(),
        stats.power(),
        stats.speed(),
        stats.forte(),
    ];

    let total: usize = core
        .iter()
        .map(|stat| {
            let bonus = match stat.quality()() {
                Quality::Basic => 0,
                Quality::Adept => 5,
                Quality::Superb => 10,
            };
            stat.quantity()() + bonus
        })
        .sum();

    rsx! {
        h1 { "Stats {total.separate_with_commas()}" }
        Grid { class: "grid--pair",
            for stat in core {
                StatCard { stat }
            }
        }
    }
}

/// User-defined skills — the same card as a core stat, plus rename and delete.
#[component]
fn Skills(character: Store<Character>) -> Element {
    let mut skills = character.skills();

    rsx! {
        Section {
            title: "Skills",
            count: skills.len(),
            onadd: move |_| skills.push(Stat::new(String::new())),
            CardGrid {
                for (index , skill) in skills.iter().enumerate() {
                    StatCard {
                        key: "{index}",
                        stat: skill,
                        renameable: true,
                        ondelete: move |_| {
                            skills.remove(index);
                        },
                    }
                }
            }
        }
    }
}

/// One stat block: name, roll trigger, quality, quantity and checks.
#[component]
fn StatCard(
    stat: Store<Stat>,
    /// Skills can be renamed; core stats can't.
    #[props(default)]
    renameable: bool,
    /// Present only for stats the user can remove.
    ondelete: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        Card { class: "fill",
            Row { class: "fill",
                if renameable {
                    TextInput {
                        value: stat.name(),
                        class: "input--stat",
                        placeholder: "Skill Name",
                    }
                } else {
                    h3 { class: "grow", {stat.name()} }
                }
                RollBtn { stat }
                if let Some(ondelete) = ondelete {
                    DeleteBtn { onclick: ondelete }
                }
            }
            Row { class: "fill",
                div { class: "field grow",
                    QualityInput { quality: stat.quality(), class: "grow" }
                    NumberInput { value: stat.quantity(), class: "grow" }
                }
                Field { label: "Checks:",
                    NumberInput { value: stat.checks() }
                }
            }
        }
    }
}

/// Opens the dice roller for `stat`.
#[component]
fn RollBtn(stat: Store<Stat>) -> Element {
    let mut roll = use_context::<Store<Ui>>().roll();

    rsx! {
        IconBtn {
            icon: BsDice6,
            size: 45,
            kind: BtnKind::Ghost,
            onclick: move |_| roll.set(Some(stat())),
        }
    }
}

/// Carried items, as name/quantity pairs.
#[component]
fn Inventory(character: Store<Character>) -> Element {
    let mut inventory = character.inventory();

    rsx! {
        Section {
            title: "Inventory",
            count: inventory.len(),
            onadd: move |_| inventory.push(Item::default()),
            CardGrid {
                for (index , item) in inventory.iter().enumerate() {
                    Row { key: "{index}", class: "bordered",
                        TextInput {
                            value: item.name(),
                            class: "input--stat",
                            placeholder: "Item",
                        }
                        NumberInput { value: item.quantity() }
                        DeleteBtn {
                            onclick: move |_| {
                                inventory.remove(index);
                            },
                        }
                    }
                }
            }
        }
    }
}

// NOTE: `Character::resources` still round-trips through save/load but has no
// UI — it needs a rules overhaul before it's worth rendering again.

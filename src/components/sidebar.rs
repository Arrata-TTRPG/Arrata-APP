//! Roster sidebar: switch between characters, create, import and delete them.

use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::bs_icons::{BsBoxArrowInDown, BsList, BsPersonPlusFill},
};

use arrata_lib::{Character, CharacterStoreExt, CoreStatsStoreExt, StatStoreExt, combat};

use crate::components::{
    io::pick_character,
    shared::{Btn, BtnKind, DeleteBtn, IconBtn},
};
use crate::{Roster, RosterStoreExt, Ui, UiStoreExt};

/// Reopen handle, pinned to the left edge while the sidebar is collapsed.
#[component]
pub fn SidebarToggle() -> Element {
    let mut open = use_context::<Store<Ui>>().sidebar();

    if open() {
        return rsx! {};
    }

    rsx! {
        IconBtn {
            icon: BsList,
            size: 28,
            kind: BtnKind::Ghost,
            class: "sidebar__toggle",
            onclick: move |_| open.set(true),
        }
    }
}

/// The roster list itself. Collapses to zero width rather than unmounting, so
/// the open/close transition has something to animate.
#[component]
pub fn Sidebar() -> Element {
    let roster = use_context::<Store<Roster>>();
    let mut open = use_context::<Store<Ui>>().sidebar();
    let characters = roster.characters();

    rsx! {
        aside { class: if open() { "sidebar sidebar--open" } else { "sidebar" },
            div { class: "sidebar__header",
                span { "Characters" }
                button {
                    class: "sidebar__close",
                    title: "Close sidebar",
                    onclick: move |_| open.set(false),
                    "✕"
                }
            }

            div { class: "sidebar__actions",
                button {
                    class: "sidebar__action",
                    title: "New Character",
                    onclick: move |_| add_character(roster, Character::default()),
                    Icon { width: 20, height: 20, icon: BsPersonPlusFill }
                    "New"
                }
                button {
                    class: "sidebar__action",
                    title: "Import as New Character",
                    onclick: move |_| async move {
                        if let Some(character) = pick_character().await {
                            add_character(roster, character);
                        }
                    },
                    Icon { width: 20, height: 20, icon: BsBoxArrowInDown }
                    "Import"
                }
            }

            div { class: "sidebar__list",
                for (index , character) in characters.iter().enumerate() {
                    RosterEntry { key: "{index}", index, character }
                }
            }
        }
    }
}

/// One row of the roster: name, current/max health, and a guarded delete.
#[component]
fn RosterEntry(index: usize, character: Store<Character>) -> Element {
    let roster = use_context::<Store<Roster>>();
    let mut confirming = use_signal(|| false);

    let stats = character.stats();
    let max_health = combat::max_health(stats.will().quantity()(), stats.forte().quantity()());
    let name = character.name()();
    let name = if name.is_empty() { "Unnamed" } else { &name };
    let active = roster.active()() == index;

    rsx! {
        div { class: if active { "sidebar__entry sidebar__entry--active" } else { "sidebar__entry" },
            div { class: "row fill",
                span {
                    class: "sidebar__name",
                    onclick: move |_| roster.active().set(index),
                    {name}
                }
                if confirming() {
                    div { class: "row row--tight",
                        Btn {
                            kind: BtnKind::Confirm,
                            class: "btn--sm",
                            onclick: move |event: MouseEvent| {
                                event.stop_propagation();
                                delete_character(roster, index);
                                confirming.set(false);
                            },
                            "Yes"
                        }
                        Btn {
                            kind: BtnKind::Cancel,
                            class: "btn--sm",
                            onclick: move |event: MouseEvent| {
                                event.stop_propagation();
                                confirming.set(false);
                            },
                            "No"
                        }
                    }
                } else {
                    DeleteBtn {
                        small: true,
                        onclick: move |event: MouseEvent| {
                            event.stop_propagation();
                            confirming.set(true);
                        },
                    }
                }
            }
            div {
                class: "sidebar__hp",
                onclick: move |_| roster.active().set(index),
                "HP {character.current_health()} / {max_health}"
            }
        }
    }
}

/// Appends `character` to the roster and switches to it.
fn add_character(roster: Store<Roster>, character: Character) {
    let mut characters = roster.characters();
    characters.push(character);
    roster.active().set(characters.len() - 1);
}

/// Removes the character at `index`, keeping the roster non-empty and the
/// active index in range.
fn delete_character(roster: Store<Roster>, index: usize) {
    let mut characters = roster.characters();

    if characters.len() <= 1 {
        characters.set(vec![Character::default()]);
    } else {
        characters.remove(index);
    }

    let last = characters.len() - 1;
    let mut active = roster.active();
    if active() > last {
        active.set(last);
    }
}

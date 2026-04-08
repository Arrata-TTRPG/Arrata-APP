use arrata_lib::{character::Character, combat};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::bs_icons::{BsBoxArrowInDown, BsList, BsPersonPlusFill, BsTrash},
};

use crate::render::pick_character_file;
use crate::{ACTIVE_IDX, CHARACTER, CHARACTERS, SIDEBAR_OPEN};

/// Sidebar toggle button — only rendered when sidebar is closed.
#[component]
pub(crate) fn SidebarToggle() -> Element {
    if SIDEBAR_OPEN() {
        return rsx! {};
    }
    rsx! {
        button {
            class: "fixed left-0 top-[20px] z-50 bg-slate-800 hover:bg-slate-600 border border-slate-600 rounded-r-lg p-3",
            onclick: move |_| *SIDEBAR_OPEN.write() = true,
            Icon {
                width: 28,
                height: 28,
                fill: "white",
                icon: BsList,
            }
        }
    }
}

/// Collapsible sidebar listing all characters in the roster.
#[component]
pub(crate) fn CharacterSidebar() -> Element {
    // Per-entry delete-confirm state: `Some(i)` means entry `i` is pending confirm.
    let mut confirm_delete: Signal<Option<usize>> = use_signal(|| None);

    rsx! {
        // Full-screen on small devices, fixed 260px column on larger ones.
        // Slide in/out via width + opacity transition.
        div {
            class: "h-full flex flex-col bg-slate-900 overflow-hidden transition-all duration-300 shrink-0",
            class: if SIDEBAR_OPEN() { "sm:w-64 w-full opacity-100 border-r border-slate-700" } else { "w-0 opacity-0 border-r-0" },

            // Header row
            div { class: "flex items-center justify-between px-4 py-3 border-b border-slate-700",
                span { class: "font-mono font-extrabold text-2xl", "Characters" }
                button {
                    class: "font-mono text-xl hover:text-slate-400 transition-colors px-1",
                    title: "Close sidebar",
                    onclick: move |_| *SIDEBAR_OPEN.write() = false,
                    "✕"
                }
            }

            // New + Import buttons
            div { class: "flex border-b border-slate-700",
                button {
                    class: "flex items-center justify-center gap-2 flex-1 px-3 py-3 font-mono text-lg bg-slate-800 hover:bg-slate-700 border-r border-slate-700",
                    title: "New Character",
                    onclick: move |_| {
                        sync_active_to_roster();
                        let mut chars = CHARACTERS.write();
                        chars.push(Character::default());
                        let new_idx = chars.len() - 1;
                        drop(chars);
                        switch_to(new_idx);
                    },
                    Icon {
                        width: 20,
                        height: 20,
                        fill: "white",
                        icon: BsPersonPlusFill,
                    }
                    "New"
                }
                button {
                    class: "flex items-center justify-center gap-2 flex-1 px-3 py-3 font-mono text-lg bg-slate-800 hover:bg-slate-700",
                    title: "Import as New Character",
                    onclick: move |_| {
                        spawn(async move {
                            if let Some(character) = pick_character_file().await {
                                sync_active_to_roster();
                                let mut chars = CHARACTERS.write();
                                chars.push(character);
                                let new_idx = chars.len() - 1;
                                drop(chars);
                                switch_to(new_idx);
                            }
                        });
                    },
                    Icon {
                        width: 20,
                        height: 20,
                        fill: "white",
                        icon: BsBoxArrowInDown,
                    }
                    "Import"
                }
            }

            // Character list
            div { class: "flex flex-col overflow-y-auto flex-grow",
                for (i, character) in CHARACTERS().iter().enumerate() {
                    {
                        let is_active = ACTIVE_IDX() == i;
                        let will_qty = character.stats.first().map_or(1, |s| s.quantity);
                        let forte_qty = character.stats.get(5).map_or(1, |s| s.quantity);
                        let max_hp = combat::max_health(will_qty, forte_qty);
                        let current_hp = character.current_health;
                        let pending = confirm_delete() == Some(i);
                        let display_name = if character.name.is_empty() {
                            "Unnamed".to_string()
                        } else {
                            character.name.clone()
                        };

                        rsx! {
                            div {
                                key: "{i}",
                                class: "flex flex-col px-4 py-3 border-b border-slate-800 cursor-pointer gap-1 transition-colors",
                                class: if is_active { "bg-slate-700" } else { "hover:bg-slate-800" },

                                // Name row + delete button
                                div { class: "flex items-center justify-between gap-2",
                                    span {
                                        class: "font-mono text-lg font-bold truncate flex-grow",
                                        onclick: move |_| {
                                            sync_active_to_roster();
                                            switch_to(i);
                                        },
                                        "{display_name}"
                                    }
                                    if pending {
                                        div { class: "flex gap-2 shrink-0",
                                            button {
                                                class: "font-mono text-base text-red-400 hover:text-red-300 border border-red-700 rounded px-2 py-1",
                                                onclick: move |evt| {
                                                    evt.stop_propagation();
                                                    delete_character(i);
                                                    confirm_delete.set(None);
                                                },
                                                "Yes"
                                            }
                                            button {
                                                class: "font-mono text-base text-slate-400 hover:text-slate-300 border border-slate-600 rounded px-2 py-1",
                                                onclick: move |evt| {
                                                    evt.stop_propagation();
                                                    confirm_delete.set(None);
                                                },
                                                "No"
                                            }
                                        }
                                    } else {
                                        button {
                                            class: "text-slate-500 hover:text-red-400 shrink-0 transition-colors",
                                            onclick: move |evt| {
                                                evt.stop_propagation();
                                                confirm_delete.set(Some(i));
                                            },
                                            Icon {
                                                width: 18,
                                                height: 18,
                                                fill: "currentColor",
                                                icon: BsTrash,
                                            }
                                        }
                                    }
                                }

                                // HP row
                                div {
                                    class: "font-mono text-base text-slate-300",
                                    onclick: move |_| {
                                        sync_active_to_roster();
                                        switch_to(i);
                                    },
                                    "HP {current_hp} / {max_hp}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Flush `CHARACTER` (the live edited copy) back into `CHARACTERS[ACTIVE_IDX]`.
pub(crate) fn sync_active_to_roster() {
    let idx = ACTIVE_IDX();
    let character = CHARACTER();
    let mut chars = CHARACTERS.write();
    if let Some(slot) = chars.get_mut(idx) {
        *slot = character;
    }
}

/// Switch the active character to index `idx`, loading from the roster.
fn switch_to(idx: usize) {
    let character = CHARACTERS().into_iter().nth(idx).unwrap_or_default();
    *CHARACTER.write() = character;
    *ACTIVE_IDX.write() = idx;
}

/// Delete character at `idx`, adjusting active index as needed.
fn delete_character(idx: usize) {
    sync_active_to_roster();
    let mut chars = CHARACTERS.write();
    if chars.len() <= 1 {
        // Always keep at least one character
        chars[0] = Character::default();
        drop(chars);
        *CHARACTER.write() = Character::default();
        *ACTIVE_IDX.write() = 0;
        return;
    }
    chars.remove(idx);
    let new_idx = ACTIVE_IDX().min(chars.len() - 1);
    let character = chars[new_idx].clone();
    drop(chars);
    *CHARACTER.write() = character;
    *ACTIVE_IDX.write() = new_idx;
}


use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsTrash, Icon};

use crate::{character::Item, CHARACTER};

#[component]
pub(crate) fn RenderInventory() -> Element {
    rsx! {
        div { class: "w-[704px] flex-auto justify-center",
            div { class: "flex justify-center content-center items-center",
                h2 { class: "inline-flex py-4 px-4 text-center text-4xl font-bold font-mono",
                    "Inventory"
                }
                button {
                    class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold py-1 px-4 rounded",
                    onclick: move |_| CHARACTER.write().inventory.push(Item::default()),
                    "+ Add Item"
                }
            }
            div { class: "flex justify-center",
                div { class: "grid grid-cols-2 gap-4 justify-center justify-items-center max-w-2xl",
                    for (i , item) in CHARACTER().inventory.iter().enumerate() {
                        div { class: "justify-center content-center items-center justify-items-center border border-spacing-2 px-3 py-3 top-2 bottom-2 left-2 right-2 rounded-lg",
                            div { class: "inline-flex items-center content-center",
                                div {
                                    input {
                                        class: "w-44 font-mono text-lg text-center border-spacing-1 border rounded-lg py-2 px-2",
                                        r#type: "text",
                                        value: "{item.name}",
                                        oninput: move |evt| {
                                            CHARACTER.write().inventory[i].name = evt.value().to_string();
                                        }
                                    }
                                }
                                div {
                                    input {
                                        class: "w-16 border rounded-lg py-2 px-2",
                                        r#type: "number",
                                        value: i64::try_from(item.quantity).unwrap_or_default(),
                                        oninput: move |evt| {
                                            CHARACTER
                                                .with_mut(|character| {
                                                    character.inventory[i].quantity = evt
                                                        .value()
                                                        .parse::<usize>()
                                                        .unwrap_or(0);
                                                });
                                        }
                                    }
                                }
                                div { class: "px-2 py-2",
                                    button {
                                        class: "text-mono bg-slate-900 hover:bg-slate-600 text-white font-bold py-1 px-2 space-x-5 rounded",
                                        onclick: move |_| {
                                            let _ = CHARACTER.write().inventory.remove(i);
                                        },
                                        Icon { width: 20, height: 20, fill: "white", icon: BsTrash }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsTrash, Icon};

use arrata_lib::Item;

use crate::CHARACTER;

#[component]
pub(crate) fn RenderInventory() -> Element {
    rsx! {
        div { class: "min-[1920px]:w-1/3 w-full h-full flex flex-col justify-center px-2 gap-4",
            div { class: "flex justify-center content-center items-center gap-4",
                h2 { class: "inline-flex text-center text-4xl font-bold font-mono",
                    "Inventory"
                }
                button {
                    class: "inline-flex bg-slate-900 hover:bg-slate-500 text-white font-bold border py-1 px-4 rounded",
                    onclick: move |_| CHARACTER.write().inventory.push(Item::default()),
                    "+ Add Item"
                }
            }
            div { class: "grid min-[1920px]:grid-cols-2 lg:grid-cols-3 md:grid-cols-2 grid-cols-1 gap-4 justify-center w-full",
                for (i , item) in CHARACTER().inventory.iter().enumerate() {
                    div { class: "flex justify-center items-center border border-spacing-2 space-x-2 p-2 rounded-lg",
                        input {
                            class: "flex-grow font-mono text-lg text-center border-spacing-1 border rounded-lg min-w-10 p-2",
                            r#type: "text",
                            value: "{item.name}",
                            oninput: move |evt| CHARACTER.write().inventory[i].name = evt.value().to_string()
                        }
                        div {
                            input {
                                class: "w-16 border rounded-lg p-2",
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
                        button {
                            class: "bg-red-950 hover:bg-red-600 p-2 border-2 rounded-lg",
                            onclick: move |_| std::mem::drop(CHARACTER.write().inventory.remove(i)),
                            Icon { width: 25, height: 25, fill: "white", icon: BsTrash }
                        }
                    }
                }
            }
        }
    }
}

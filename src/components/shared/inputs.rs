//! Form controls.
//!
//! Every input takes a `WriteSignal<T>` rather than a value plus a callback, so
//! a caller wires one up by handing over the store field it edits and nothing
//! else. Use [`field!`](super::field) for types that don't derive `Store`.

use std::fmt::Display;

use dioxus::prelude::*;

use arrata_lib::Quality;

/// Free-text field.
#[component]
pub fn TextInput(
    mut value: WriteSignal<String>,
    placeholder: Option<String>,
    class: Option<String>,
) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        input {
            class: "input {class}",
            r#type: "text",
            value: "{value}",
            placeholder,
            oninput: move |event| value.set(event.value()),
        }
    }
}

/// Unsigned counter, clamped to `[min, max]` on every keystroke.
#[component]
pub fn NumberInput(
    mut value: WriteSignal<usize>,
    #[props(default = 0)] min: usize,
    #[props(default = usize::MAX)] max: usize,
    class: Option<String>,
) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        input {
            class: "input input--count {class}",
            r#type: "number",
            min,
            max,
            value: "{value}",
            oninput: move |event| {
                value.set(event.value().parse::<usize>().unwrap_or_default().clamp(min, max));
            },
        }
    }
}

/// Signed counter, for values that can go negative (damage modifiers).
#[component]
pub fn SignedInput(
    mut value: WriteSignal<isize>,
    #[props(default = isize::MIN)] min: isize,
    #[props(default = isize::MAX)] max: isize,
    class: Option<String>,
) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        input {
            class: "input input--count {class}",
            r#type: "number",
            min,
            max,
            value: "{value}",
            oninput: move |event| {
                value.set(event.value().parse::<isize>().unwrap_or_default().clamp(min, max));
            },
        }
    }
}

/// Auto-growing notes field. Height tracks content via CSS `field-sizing`.
#[component]
pub fn NotesArea(
    mut value: WriteSignal<String>,
    placeholder: Option<String>,
    class: Option<String>,
) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        textarea {
            class: "textarea {class}",
            placeholder,
            value: "{value}",
            oninput: move |event| value.set(event.value()),
        }
    }
}

/// Dropdown over a fixed set of values, matched by position so `T` needs no
/// parsing — only [`Display`] for the option labels.
#[component]
pub fn Dropdown<T: Clone + PartialEq + Display + 'static>(
    mut value: WriteSignal<T>,
    options: Vec<T>,
    class: Option<String>,
) -> Element {
    let selected = value();
    let choices = options.clone();
    let class = class.unwrap_or_default();

    rsx! {
        select {
            class: "select {class}",
            onchange: move |event| {
                if let Some(choice) = event.value().parse::<usize>().ok().and_then(|i| choices.get(i))
                {
                    value.set(choice.clone());
                }
            },
            for (index , option) in options.iter().enumerate() {
                option { value: "{index}", selected: *option == selected, "{option}" }
            }
        }
    }
}

/// The Basic / Adept / Superb selector.
#[component]
pub fn QualityInput(quality: WriteSignal<Quality>, class: Option<String>) -> Element {
    rsx! {
        Dropdown {
            value: quality,
            options: vec![Quality::Basic, Quality::Adept, Quality::Superb],
            class,
        }
    }
}

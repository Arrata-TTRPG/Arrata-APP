//! Layout primitives. Each is a thin wrapper over one class in
//! `assets/layout.css`, so spacing and alignment live in CSS and callers stay
//! declarative.

use dioxus::prelude::*;
use thousands::Separable;

/// Wrapping, centred flex row.
#[component]
pub fn Row(class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        div { class: "row {class}", {children} }
    }
}

/// Centred flex column.
#[component]
pub fn Col(class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        div { class: "col {class}", {children} }
    }
}

/// Bordered, rounded panel.
#[component]
pub fn Card(class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        div { class: "card {class}", {children} }
    }
}

/// Bordered tray that wraps its children into equal-width cards.
#[component]
pub fn CardGrid(class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        div { class: "card-grid {class}", {children} }
    }
}

/// Auto-fitting grid. Pass `class: "grid--pair"` to pin it to two columns.
#[component]
pub fn Grid(class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        div { class: "grid {class}", {children} }
    }
}

/// A `label: control` pair.
#[component]
pub fn Field(label: String, class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        div { class: "field {class}",
            span { class: "label", {label} }
            {children}
        }
    }
}

/// A collapsible block: heading, optional item count, optional `+` button and a
/// Show/Hide toggle. Children render only while expanded.
///
/// This is the shape every list on the sheet takes — skills, inventory, weapons,
/// armor, talents, quirk categories — so they all get identical behaviour and
/// each keeps its own open state instead of sharing a global.
#[component]
pub fn Section(
    title: String,
    /// Rendered after the title, comma-separated.
    count: Option<usize>,
    /// Shown as a `+` button when present.
    onadd: Option<EventHandler<MouseEvent>>,
    /// Use the smaller heading, for sections nested inside another.
    #[props(default)]
    small: bool,
    children: Element,
) -> Element {
    let mut open = use_signal(|| false);
    let heading = match count {
        Some(count) => format!("{title} {}", count.separate_with_commas()),
        None => title,
    };
    let size = if small { "section__title--sm" } else { "" };

    rsx! {
        div { class: "field fill",
            h2 { class: "section__title {size}", {heading} }
            if let Some(onadd) = onadd {
                button {
                    class: "btn btn--add",
                    onclick: move |event| onadd.call(event),
                    "+"
                }
            }
            button {
                class: "btn",
                onclick: move |_| open.toggle(),
                if open() { "Hide" } else { "Show" }
            }
        }
        if open() {
            {children}
        }
    }
}

//! Buttons. One component per shape, with the visual variant chosen by
//! [`BtnKind`] so callers never spell out `btn--*` classes.

use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon, IconShape,
    icons::bs_icons::{BsTrash, BsX},
};

/// Visual variants of [`Btn`] and [`IconBtn`], mapping onto `assets/controls.css`.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum BtnKind {
    /// Standard bordered action.
    #[default]
    Plain,
    /// Tighter horizontal padding.
    Small,
    /// The `+` beside a section heading.
    Add,
    /// Full-width primary action.
    Large,
    /// Destructive action.
    Danger,
    /// Destructive confirmation in a prompt.
    Confirm,
    /// Dismissal in a prompt.
    Cancel,
    /// No chrome at all — for bare icon triggers.
    Ghost,
}

impl BtnKind {
    const fn class(self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Small => "btn--sm",
            Self::Add => "btn--add",
            Self::Large => "btn--lg",
            Self::Danger => "btn--danger",
            Self::Confirm => "btn--confirm",
            Self::Cancel => "btn--cancel",
            Self::Ghost => "btn--ghost",
        }
    }
}

/// Text button.
#[component]
pub fn Btn(
    #[props(default)] kind: BtnKind,
    class: Option<String>,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let class = format!("btn {} {}", kind.class(), class.unwrap_or_default());
    rsx! {
        button { class, onclick: move |event| onclick.call(event), {children} }
    }
}

/// Icon-only button. The icon inherits the button's text colour.
#[component]
pub fn IconBtn<I: IconShape + Clone + PartialEq + 'static>(
    icon: I,
    #[props(default)] kind: BtnKind,
    #[props(default = 25)] size: u32,
    class: Option<String>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let class = format!("btn {} {}", kind.class(), class.unwrap_or_default());
    rsx! {
        button { class, onclick: move |event| onclick.call(event),
            Icon { width: size, height: size, icon }
        }
    }
}

/// Trash button. `small` picks the tighter variant used inside dense rows.
#[component]
pub fn DeleteBtn(#[props(default)] small: bool, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        IconBtn {
            icon: BsTrash,
            kind: BtnKind::Danger,
            size: if small { 18 } else { 25 },
            class: if small { "btn--sm" } else { "" },
            onclick,
        }
    }
}

/// The ✕ that dismisses an overlay, positioned in its top-right corner.
#[component]
pub fn CloseBtn(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "overlay__close",
            IconBtn {
                icon: BsX,
                kind: BtnKind::Ghost,
                class: "btn--close",
                size: 35,
                onclick,
            }
        }
    }
}

/// Inline "are you sure?" strip, used wherever an action destroys data.
#[component]
pub fn Confirm(
    prompt: Option<String>,
    #[props(default = "Confirm".into())] yes: String,
    #[props(default = "Cancel".into())] no: String,
    on_yes: EventHandler<MouseEvent>,
    on_no: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "confirm",
            if let Some(prompt) = prompt {
                p { class: "confirm__prompt", {prompt} }
            }
            div { class: "row",
                Btn { kind: BtnKind::Confirm, onclick: on_yes, {yes} }
                Btn { kind: BtnKind::Cancel, onclick: on_no, {no} }
            }
        }
    }
}

//! Building blocks shared by every part of the sheet.
//!
//! Nothing in here knows about characters — these are generic layout, input and
//! button primitives that render the classes defined in `assets/`. Feature
//! modules compose them instead of hand-writing `div`s and class strings.

mod buttons;
mod inputs;
mod layout;

pub use buttons::{Btn, BtnKind, CloseBtn, Confirm, DeleteBtn, IconBtn};
pub use inputs::{Dropdown, NotesArea, NumberInput, QualityInput, SignedInput, TextInput};
pub use layout::{Card, CardGrid, Col, Field, Grid, Row, Section};

/// Borrows one field of a store as its own `WriteSignal`.
///
/// The generic inputs in [`inputs`] all take `WriteSignal<T>`. Types that derive
/// `Store` get that for free from the generated accessors; this macro covers the
/// ones that don't (`Weapon`, `Armor`, `Talent`, `Quirk`) without falling back
/// to whole-value writes.
///
/// ```ignore
/// TextInput { value: field!(weapon, name) }
/// ```
macro_rules! field {
    ($store:expr, $($path:tt).+) => {
        ::dioxus::prelude::WritableExt::map_mut(
            $store,
            |value| &value.$($path).+,
            |value| &mut value.$($path).+,
        )
    };
}

pub(crate) use field;

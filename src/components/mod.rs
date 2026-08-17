//! The component tree.
//!
//! `shared` holds the generic primitives; every other module owns one region of
//! the sheet and takes the store slice it edits as a prop.

pub mod character;
pub mod combat;
pub mod entrypoint;
pub mod io;
pub mod popup;
pub mod premade_quirks;
pub mod quirks;
pub mod rolls;
pub mod shared;
pub mod sidebar;
pub mod stats;

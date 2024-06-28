// If we're not in a prod compilation, allow a window for debug prints
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(any(feature = "web", feature = "desktop"))]
use dioxus::prelude::*;

fn main() {
    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(any(feature = "web", feature = "desktop"))]
    {
        dioxus_sdk::storage::set_dir!();
        launch(arrata_app::render::App);
    }
}

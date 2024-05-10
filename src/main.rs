// If we're not in a prod compilation, allow a window for debug prints
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dioxus::prelude::*;

fn main() {
    launch(arrata_app::render::App);
}

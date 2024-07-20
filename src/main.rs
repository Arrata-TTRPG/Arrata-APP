// If we're not in a prod compilation, allow a window for debug prints
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

use dioxus::prelude::*;

fn main() {
    // Desktop setup
    #[cfg(feature = "desktop")]
    {
        if let Ok(mut path) = std::env::current_exe() {
            println!("{:?}", path);
            if path.pop() {
                path.push("appdata");
                std::fs::create_dir_all(path.clone()).unwrap();
                arrata_app::storage::set_directory(path);
            }
        }
    }

    // Web setup
    #[cfg(feature = "web")]
    {
        wasm_logger::init(wasm_logger::Config::default());
    }

    launch(arrata_app::render::App);
}

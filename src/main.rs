// If we're not in a prod compilation, allow a window for debug prints
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_family = "wasm"))]
use dioxus_desktop::{Config, WindowBuilder};

use arrata_app::app::render::app;

fn main() {
    launch();
}

#[cfg(not(target_family = "wasm"))]
fn launch() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            .with_window(WindowBuilder::new().with_title("Arrata Character Sheet Manager"))
            //.with_icon(icon)
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.into()),
    );
}

#[cfg(target_family = "wasm")]
fn launch() {
    dioxus_web::launch(app);
}

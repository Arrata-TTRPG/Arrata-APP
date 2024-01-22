// If we're not in a prod compilation, allow a window for debug prints
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(any(feature = "desktop", feature = "web"))]
use arrata_app::*;

#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};

fn main() {
    launch();
}

#[cfg(feature = "desktop")]
fn launch() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            .with_window(WindowBuilder::new().with_title("Arrata Character Sheet Manager"))
            //.with_icon(icon)
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.into()),
    );
}

#[cfg(feature = "web")]
fn launch() {
    dioxus_web::launch(app);
}

#[cfg(all(not(feature = "desktop"), not(feature = "web")))]
fn launch() {
    panic!("No feature selected!")
}

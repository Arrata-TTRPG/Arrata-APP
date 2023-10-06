#![windows_subsystem = "windows"]

use arrata_app::*;
use dioxus_desktop::{Config, WindowBuilder};

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            .with_window(WindowBuilder::new().with_title("Arrata Character Sheet Manager"))
            .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string()),
    );
}

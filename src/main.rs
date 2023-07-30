#![windows_subsystem = "windows"]

use arrata_app::*;
use dioxus_desktop::{Config, WindowBuilder};

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(
            WindowBuilder::new().with_title("Arrata Character Sheet Manager")
        )
    );
}

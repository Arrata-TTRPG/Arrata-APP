#![windows_subsystem = "windows"]

use arrata_app::*;
use dioxus_desktop::{tao::window::Icon, Config, WindowBuilder};

fn main() {
    let icon_rgb: Vec<u8> = vec![255; 256];
    let icon = match Icon::from_rgba(icon_rgb, 8, 8) {
        Ok(i) => i,
        Err(e) => {
            println!("Error thrown: {e:#?}");
            return;
        }
    };

    dioxus_desktop::launch_cfg(
        app,
        Config::default()
            .with_window(WindowBuilder::new().with_title("Arrata Character Sheet Manager"))
            .with_icon(icon),
    );
}

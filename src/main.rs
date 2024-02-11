// If we're not in a prod compilation, allow a window for debug prints
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "app")]
fn main() {
    #[cfg(not(target_family = "wasm"))]
    dioxus_desktop::launch_cfg(
        arrata_app::app::render::app,
        dioxus_desktop::Config::new()
            .with_window(
                dioxus_desktop::WindowBuilder::new().with_title("Arrata Character Sheet Manager"),
            )
            //.with_icon(icon)
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.into()),
    );

    #[cfg(target_family = "wasm")]
    dioxus_web::launch(arrata_app::app::render::app);
}

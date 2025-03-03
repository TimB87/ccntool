#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ccntool_gui::EguiSandbox;
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use image::load_from_memory;
    tracing_subscriber::fmt::init();

    let _icon = load_from_memory(include_bytes!("../../assets/HSDCIT.png"))
        .expect("Failed to open icon path");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_inner_size([400.0, 100.0])
            .with_min_inner_size([400.0, 100.0])
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "TDQU",
        native_options,
        Box::new(|cc| Ok(Box::new(EguiSandbox::new(cc)) as Box<dyn eframe::App>)),
    )
    .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "TDQU",
            web_options,
            Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}

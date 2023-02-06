#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.maximized = false;
    native_options.decorated = true;
    native_options.drag_and_drop_support = false;
    native_options.initial_window_pos = Option::from(egui::pos2(350.0, 350.0));
    native_options.initial_window_size = Option::from(egui::vec2(400.0, 330.0));
    native_options.resizable = false;

    eframe::run_native(
        "rser2tcp",
        native_options,
        Box::new(|cc| Box::new(rser2tcp::MyApp::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(rser2tcp::MyApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}

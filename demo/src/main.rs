use eframe::egui::vec2;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(600.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(Box::new(eguikit_demo::Demo::default()), native_options);
}

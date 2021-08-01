use eframe::{
    egui::{vec2, CentralPanel, CtxRef, ScrollArea},
    epi,
};
use eguikit::Spinner;

struct Demo;

impl epi::App for Demo {
    fn name(&self) -> &str {
        "Eguikit Demo"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                ui.heading("Spinner");
                ui.add(Spinner::default());
            });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(900.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(Box::new(Demo), native_options);
}

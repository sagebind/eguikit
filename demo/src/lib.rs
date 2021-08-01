use eframe::{
    egui::{CentralPanel, CtxRef, ScrollArea},
    epi,
};
use eguikit::Spinner;

#[derive(Default)]
pub struct Demo;

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

mod wasm {
    #[cfg(target_arch = "wasm32")]
    use eframe::wasm_bindgen::{self, prelude::*};

    #[wasm_bindgen]
    #[cfg(target_arch = "wasm32")]
    pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
        eframe::start_web(canvas_id, Box::new(crate::Demo::default()))
    }
}

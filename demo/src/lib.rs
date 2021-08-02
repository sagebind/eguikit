use eframe::{
    egui::{CentralPanel, CtxRef, ScrollArea},
    epi,
};
use eguikit::{Spinner, spinner::Style};

pub struct Demo {
    spinner_style: Style,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            spinner_style: Style::Dots,
        }
    }
}

impl epi::App for Demo {
    fn name(&self) -> &str {
        "Eguikit Demo"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                ui.heading("Spinner");

                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.spinner_style, Style::Dots, "Dots");
                    ui.radio_value(&mut self.spinner_style, Style::Bars, "Bars");
                    ui.radio_value(&mut self.spinner_style, Style::Squares, "Squares");
                });

                ui.add(Spinner::default().style(self.spinner_style));
            });
        });
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use eframe::wasm_bindgen::{self, prelude::*};

    #[wasm_bindgen]
    pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
        eframe::start_web(canvas_id, Box::new(crate::Demo::default()))
    }
}

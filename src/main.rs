#![windows_subsystem = "windows"]

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // create a window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 500.0]),
        ..Default::default()
    };

    // run egui app
    eframe::run_native(
        "TinyCalc", // title
        options, 
        Box::new(|cc| {
            // return eframe::App trait
            Ok(Box::new(TinyCalc::new(cc)))
        }),
    )
}

//define TinyCalc struct
struct TinyCalc {
    // input
    input: String,
}

//TinyCalc struct new function
impl TinyCalc {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {        
        Self {
            input: String::new(),
        }
    }
}

//complete eframe::App trait 
impl eframe::App for TinyCalc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

        });
    }
}

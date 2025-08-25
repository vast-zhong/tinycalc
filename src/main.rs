#![windows_subsystem = "windows"]

use eframe::egui::{self, Color32, Painter, Pos2, Rect, Vec2};

fn show_block(ui: &mut egui::Ui) {
    let full_rect = ui.max_rect();
    let block_size = Vec2::new(full_rect.width(), full_rect.height() / 5.0);

    let block_rect = Rect::from_min_size(full_rect.min, block_size);

    let painter: &Painter = ui.painter();
    painter.rect_filled(
        block_rect, 
        5.0,  
        Color32::from_rgba_premultiplied(150, 150, 150, 60),
    );
}

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
    result: String,
}

//TinyCalc struct new function
impl TinyCalc {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {        
        Self {
            result: String::new(),
        }
    }
}

//complete eframe::App trait 
impl eframe::App for TinyCalc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            show_block(ui);
        });
    }
}

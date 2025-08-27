#![windows_subsystem = "windows"]

use eframe::egui::{self, Color32, Painter, Pos2, Rect, Vec2, StrokeKind};

fn result_block(ui: &mut egui::Ui) {
    let full_rect = ui.max_rect();
    let block_size = Vec2::new(full_rect.width(), full_rect.height() / 5.0);
    let block_rect = Rect::from_min_size(full_rect.min, block_size);
    let painter: &Painter = ui.painter();
    painter.rect_filled(
        block_rect, 
        5.0,  
        Color32::from_rgb(20, 26, 34), // #141A22
    );
}

fn input_block(ui: &mut egui::Ui) {
    let full_rect = ui.max_rect();

    let result_h = full_rect.height() / 5.0;
    let top_left = Pos2::new(full_rect.min.x, full_rect.min.y + result_h);
    let block_size = Vec2::new(full_rect.width(), full_rect.height() - result_h);
    let block_rect = Rect::from_min_size(top_left, block_size);
    let painter: &Painter = ui.painter();
    painter.rect_filled(
        block_rect, 
        5.0,  
        Color32::from_rgb(42, 48, 58), // #2A303A
    );
    let cell_w = block_rect.width() / 4.0;
    let cell_h = block_rect.height() / 5.0;
    
    let gap = 4.0; 

    let labels = [
        ["CE", "C", "DEL", "/"],
        ["7", "8", "9", "*"],
        ["4", "5", "6", "-"],
        ["1", "2", "3", "+"],
        ["%", "0", ".", "="],
    ];

    for (row_i, row) in labels.iter().enumerate() {
        for (col_i, &text) in row.iter().enumerate() {
            let x = block_rect.min.x + col_i as f32 * cell_w;
            let y = block_rect.min.y + row_i as f32 * cell_h;
            
            let button_rect = Rect::from_min_size(
                Pos2::new(x + gap / 2.0, y + gap / 2.0), 
                Vec2::new(cell_w - gap, cell_h - gap)
            );

            if ui
                .interact(button_rect, ui.id().with((row_i, col_i)), egui::Sense::click())
                .clicked()
            {
                println!("Pressed {text}");
            }

            painter.rect_stroke(
                button_rect,
                egui::CornerRadius::same(8),
                egui::Stroke::new(0.4, Color32::GRAY),
                egui::StrokeKind::Inside,
            );
            painter.text(
                button_rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(20.0),
                Color32::WHITE,
            );
        }
    }
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
            result_block(ui);
            input_block(ui);
        });
    }
}

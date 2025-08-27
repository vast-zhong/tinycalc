use eframe::egui::{self, Color32, Painter, Pos2, Rect, Vec2};
use crate::state::AppState;

pub fn render_calculator(ui: &mut egui::Ui, state: &mut AppState) {
    render_result_block(ui, state);
    render_input_block(ui, state);
}

fn render_result_block(ui: &mut egui::Ui, state: &AppState) {
    let full_rect = ui.max_rect();
    let block_size = Vec2::new(full_rect.width(), full_rect.height() / 5.0);
    let block_rect = Rect::from_min_size(full_rect.min, block_size);
    let painter: &Painter = ui.painter();
    
    // 绘制背景
    painter.rect_filled(
        block_rect, 
        5.0,  
        Color32::from_rgb(20, 26, 34), // #141A22
    );
    
    // 显示计算结果或当前输入
    let display_text = state.get_display_text();
    let text_color = if state.get_error_message().is_some() {
        Color32::from_rgb(255, 100, 100) // 错误时显示红色
    } else {
        Color32::WHITE
    };
    
    // 计算文本位置，右对齐
    let text_rect = Rect::from_min_size(
        Pos2::new(block_rect.min.x + 10.0, block_rect.min.y + 10.0),
        Vec2::new(block_rect.width() - 20.0, block_rect.height() - 20.0)
    );
    
    painter.text(
        Pos2::new(text_rect.max.x - 10.0, text_rect.center().y),
        egui::Align2::RIGHT_CENTER,
        display_text,
        egui::FontId::proportional(32.0),
        text_color,
    );
    
    // 如果有错误消息，显示在较小的字体
    if let Some(error) = state.get_error_message() {
        painter.text(
            Pos2::new(text_rect.max.x - 10.0, text_rect.max.y - 5.0),
            egui::Align2::RIGHT_BOTTOM,
            error,
            egui::FontId::proportional(14.0),
            Color32::from_rgb(255, 150, 150),
        );
    }
}

fn render_input_block(ui: &mut egui::Ui, state: &mut AppState) {
    let full_rect = ui.max_rect();

    let result_h = full_rect.height() / 5.0;
    let top_left = Pos2::new(full_rect.min.x, full_rect.min.y + result_h);
    let block_size = Vec2::new(full_rect.width(), full_rect.height() - result_h);
    let block_rect = Rect::from_min_size(top_left, block_size);
    let painter: &Painter = ui.painter();
    
    // 绘制背景
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

            // 处理按钮交互
            let response = ui.interact(
                button_rect, 
                ui.id().with((row_i, col_i)), 
                egui::Sense::click()
            );
            
            if response.clicked() {
                state.handle_button_press(text);
            }

            // 根据按钮类型设置不同的颜色
            let (bg_color, text_color) = get_button_colors(text, response.hovered());
            
            // 绘制按钮背景
            if response.hovered() {
                painter.rect_filled(
                    button_rect,
                    egui::Rounding::same(8.0),
                    bg_color,
                );
            }
            
            // 绘制按钮边框
            painter.rect_stroke(
                button_rect,
                egui::Rounding::same(8.0),
                egui::Stroke::new(0.4, Color32::GRAY),
            );
            
            // 绘制按钮文本
            painter.text(
                button_rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(20.0),
                text_color,
            );
        }
    }
}

fn get_button_colors(text: &str, hovered: bool) -> (Color32, Color32) {
    let base_alpha = if hovered { 100 } else { 50 };
    
    match text {
        "=" => {
            // 等号按钮 - 橙色
            (Color32::from_rgba_unmultiplied(255, 149, 0, base_alpha), Color32::WHITE)
        }
        "+" | "-" | "*" | "/" | "%" => {
            // 运算符按钮 - 蓝色
            (Color32::from_rgba_unmultiplied(0, 122, 255, base_alpha), Color32::WHITE)
        }
        
        "CE" | "C" | "DEL" => {
            // 清除按钮 - 红色
            (Color32::from_rgba_unmultiplied(255, 59, 48, base_alpha), Color32::WHITE)
        }
        
        _ => {
            // 数字和小数点按钮 - 灰色
            (Color32::from_rgba_unmultiplied(142, 142, 147, base_alpha), Color32::WHITE)
        }
    }
}
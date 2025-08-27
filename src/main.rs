// Only show the window without cmd window in windows
#![windows_subsystem = "windows"]

use eframe::egui::{self, Color32, Painter, Pos2, Rect, Vec2};

// Draw the result block
// display is the text content we want to show
fn result_block(ui: &mut egui::Ui, display: &str) {
    // get the full rect of the UI area
    let full_rect = ui.max_rect();
    // the size of the result block, width: full rect width, height: 1/5 full rect height
    let block_size = Vec2::new(full_rect.width(), full_rect.height() / 5.0);
    // create the result block rect, full_rect.min: left-up as start point
    let block_rect = Rect::from_min_size(full_rect.min, block_size);
    // use ui.painter to draw the result block
    let painter: &Painter = ui.painter();
    // draw the background color
    painter.rect_filled(
        block_rect, 
        5.0,  
        Color32::from_rgb(20, 26, 34), // #141A22
    );
    // draw the text
    painter.text(
        block_rect.center(),
        egui::Align2::CENTER_CENTER,
        display,
        egui::FontId::proportional(24.0),
        Color32::WHITE,
    );
}

// Draw the input block
fn input_block(ui: &mut egui::Ui) -> Option<String> {
    let full_rect = ui.max_rect();
    let mut clicked_button = None;

    let result_h = full_rect.height() / 5.0;
    // get the start position of the input block
    let top_left = Pos2::new(full_rect.min.x, full_rect.min.y + result_h);
    // the size of the input block, width: full rect width, height: 4/5 full rect height
    let block_size = Vec2::new(full_rect.width(), full_rect.height() - result_h);
    let block_rect = Rect::from_min_size(top_left, block_size);
    // then draw the input block
    let painter: &Painter = ui.painter();
    painter.rect_filled(
        block_rect, 
        5.0,  
        Color32::from_rgb(42, 48, 58), // #2A303A
    );
    // the size of each cell in input block
    let cell_w = block_rect.width() / 4.0;
    let cell_h = block_rect.height() / 5.0;
    // the gap between each cell
    let gap = 4.0; 
    // each cell in the input block
    let labels = [
        ["CE", "C", "DEL", "/"],
        ["7", "8", "9", "*"],
        ["4", "5", "6", "-"],
        ["1", "2", "3", "+"],
        ["%", "0", ".", "="],
    ];

    // double loop to draw each cell
    // every row
    for (row_i, row) in labels.iter().enumerate() {
        // every col
        for (col_i, &text) in row.iter().enumerate() {
            // get the position of the cell
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
                clicked_button = Some(text.to_string());
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
    
    clicked_button
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
    // the result we display
    display: String,
    // the expression
    expression: String,
    // calculated
    just_calculated: bool,
}

//TinyCalc struct new function
impl TinyCalc {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {        
        Self {
            display: "0".to_string(),
            expression: String::new(),
            just_calculated: false,
        }
    }
    
    // 处理按钮点击
    fn handle_button(&mut self, button: &str) {
        match button {
            "C" => {
                self.display = "0".to_string();
                self.expression.clear();
                self.just_calculated = false;
            }
            "CE" => {
                self.display = "0".to_string();
                self.just_calculated = false;
            }
            "DEL" => {
                if !self.expression.is_empty() {
                    self.expression.pop();
                    if self.expression.is_empty() {
                        self.display = "0".to_string();
                    } else {
                        self.display = self.expression.clone();
                    }
                }
                self.just_calculated = false;
            }
            "=" => {
                if !self.expression.is_empty() {
                    if let Ok(result) = self.evaluate_expression(&self.expression) {
                        self.display = self.format_number(result);
                        self.expression = self.display.clone();
                        self.just_calculated = true;
                    }
                }
            }
            "+" | "-" | "*" | "/" | "%" => {
                if self.just_calculated {
                    self.just_calculated = false;
                }
                if !self.expression.is_empty() && !self.ends_with_operator() {
                    self.expression.push_str(button);
                    self.display = self.expression.clone();
                }
            }
            "." => {
                if self.just_calculated {
                    self.expression = "0.".to_string();
                    self.display = self.expression.clone();
                    self.just_calculated = false;
                } else if self.can_add_decimal() {
                    if self.expression.is_empty() {
                        self.expression = "0.".to_string();
                    } else {
                        self.expression.push('.');
                    }
                    self.display = self.expression.clone();
                }
            }
            _ => {
                // 数字按钮
                if self.just_calculated {
                    self.expression = button.to_string();
                    self.just_calculated = false;
                } else {
                    if self.display == "0" && self.expression.is_empty() {
                        self.expression = button.to_string();
                    } else {
                        self.expression.push_str(button);
                    }
                }
                self.display = self.expression.clone();
            }
        }
    }
    
    fn ends_with_operator(&self) -> bool {
        if let Some(last_char) = self.expression.chars().last() {
            matches!(last_char, '+' | '-' | '*' | '/' | '%')
        } else {
            false
        }
    }
    
    fn can_add_decimal(&self) -> bool {
        if self.expression.is_empty() {
            return true;
        }
        
        let chars: Vec<char> = self.expression.chars().collect();
        let mut i = chars.len();
        
        while i > 0 {
            i -= 1;
            let ch = chars[i];
            if ch == '.' {
                return false;
            }
            if matches!(ch, '+' | '-' | '*' | '/' | '%') {
                break;
            }
        }
        
        true
    }
    
    fn evaluate_expression(&self, expr: &str) -> Result<f64, String> {
        // 简单的表达式求值，支持基本四则运算和取余
        let expr = expr.replace(" ", "");
        
        // 这里使用一个简单的求值方法
        self.simple_eval(&expr)
    }
    
    fn simple_eval(&self, expr: &str) -> Result<f64, String> {
        // 简单的递归下降解析器
        let mut chars = expr.chars().peekable();
        self.parse_expression(&mut chars)
    }
    
    fn parse_expression(&self, chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<f64, String> {
        let mut result = self.parse_term(chars)?;
        
        while let Some(&op) = chars.peek() {
            if op == '+' || op == '-' {
                chars.next();
                let term = self.parse_term(chars)?;
                if op == '+' {
                    result += term;
                } else {
                    result -= term;
                }
            } else {
                break;
            }
        }
        
        Ok(result)
    }
    
    fn parse_term(&self, chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<f64, String> {
        let mut result = self.parse_factor(chars)?;
        
        while let Some(&op) = chars.peek() {
            if op == '*' || op == '/' || op == '%' {
                chars.next();
                let factor = self.parse_factor(chars)?;
                if op == '*' {
                    result *= factor;
                } else if op == '/' {
                    if factor == 0.0 {
                        return Err("除零错误".to_string());
                    }
                    result /= factor;
                } else if op == '%' {
                    if factor == 0.0 {
                        return Err("除零错误".to_string());
                    }
                    result %= factor;
                }
            } else {
                break;
            }
        }
        
        Ok(result)
    }
    
    fn parse_factor(&self, chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<f64, String> {
        let mut num_str = String::new();
        
        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                chars.next();
            } else {
                break;
            }
        }
        
        if num_str.is_empty() {
            Err("无效的数字".to_string())
        } else {
            num_str.parse::<f64>().map_err(|_| "数字解析错误".to_string())
        }
    }
    
    fn format_number(&self, num: f64) -> String {
        if num.fract() == 0.0 && num.abs() < 1e15 {
            format!("{}", num as i64)
        } else {
            let formatted = format!("{:.10}", num);
            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }
}

//complete eframe::App trait 
impl eframe::App for TinyCalc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            result_block(ui, &self.display);
            if let Some(button) = input_block(ui) {
                self.handle_button(&button);
            }
        });
    }
}

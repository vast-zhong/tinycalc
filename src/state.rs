use crate::calculator::Calculator;

#[derive(Debug, Clone)]
pub struct AppState {
    pub calculator: Calculator,
    pub display_text: String,
    pub error_message: Option<String>,
    pub history: Vec<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            calculator: Calculator::new(),
            display_text: "0".to_string(),
            error_message: None,
            history: Vec::new(),
        }
    }

    pub fn handle_button_press(&mut self, button: &str) {
        self.error_message = None;
        
        match button {
            "CE" => {
                self.calculator.clear_entry();
                self.update_display();
            }
            "C" => {
                self.calculator.clear();
                self.display_text = "0".to_string();
            }
            "DEL" => {
                self.calculator.delete_last();
                self.update_display();
            }
            "=" => {
                let expression = self.calculator.get_expression().to_string();
                if !expression.is_empty() {
                    self.calculator.add_input("=");
                    if let Some(result) = self.calculator.get_result() {
                        self.history.push(format!("{} = {}", expression, result));
                        // 保持历史记录在合理范围内
                        if self.history.len() > 50 {
                            self.history.remove(0);
                        }
                    }
                    self.update_display();
                }
            }
            _ => {
                self.calculator.add_input(button);
                self.update_display();
            }
        }
    }

    fn update_display(&mut self) {
        let expression = self.calculator.get_expression();
        
        if expression.is_empty() {
            self.display_text = "0".to_string();
        } else {
            self.display_text = expression.to_string();
        }
        
        // 如果有计算结果，显示结果
        if let Some(result) = self.calculator.get_result() {
            self.display_text = self.format_number(result);
        }
    }

    fn format_number(&self, num: f64) -> String {
        // 如果是整数，不显示小数点
        if num.fract() == 0.0 && num.abs() < 1e15 {
            format!("{}", num as i64)
        } else {
            // 限制小数位数，避免显示过长
            let formatted = format!("{:.10}", num);
            // 移除尾随的零
            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }

    pub fn get_display_text(&self) -> &str {
        &self.display_text
    }

    pub fn get_error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
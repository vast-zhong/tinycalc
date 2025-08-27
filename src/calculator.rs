use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone)]
pub struct Calculator {
    expression: String,
    result: Option<f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            expression: String::new(),
            result: None,
        }
    }

    pub fn clear(&mut self) {
        self.expression.clear();
        self.result = None;
    }

    pub fn clear_entry(&mut self) {
        self.expression.clear();
    }

    pub fn delete_last(&mut self) {
        self.expression.pop();
    }

    pub fn add_input(&mut self, input: &str) {
        match input {
            "=" => {
                self.calculate();
            }
            "+" | "-" | "*" | "/" | "%" => {
                if !self.expression.is_empty() && !self.ends_with_operator() {
                    self.expression.push_str(input);
                }
            }
            "." => {
                if self.can_add_decimal() {
                    self.expression.push('.');
                }
            }
            _ => {
                // 数字输入
                self.expression.push_str(input);
            }
        }
    }

    pub fn get_expression(&self) -> &str {
        &self.expression
    }

    pub fn get_result(&self) -> Option<f64> {
        self.result
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
            return false;
        }
        
        // 检查当前数字是否已经包含小数点
        let chars: Vec<char> = self.expression.chars().collect();
        let mut i = chars.len();
        
        while i > 0 {
            i -= 1;
            let ch = chars[i];
            if ch == '.' {
                return false; // 已经有小数点了
            }
            if matches!(ch, '+' | '-' | '*' | '/' | '%') {
                break; // 遇到操作符，说明是新的数字
            }
        }
        
        true
    }

    fn calculate(&mut self) {
        if let Ok(tokens) = self.tokenize(&self.expression) {
            if let Ok(result) = self.evaluate(tokens) {
                self.result = Some(result);
                self.expression = result.to_string();
            }
        }
    }

    fn tokenize(&self, expr: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut current_number = String::new();
        
        for ch in expr.chars() {
            match ch {
                '0'..='9' | '.' => {
                    current_number.push(ch);
                }
                '+' | '-' | '*' | '/' | '%' => {
                    if !current_number.is_empty() {
                        if let Ok(num) = current_number.parse::<f64>() {
                            tokens.push(Token::Number(num));
                        } else {
                            return Err("Invalid number".to_string());
                        }
                        current_number.clear();
                    }
                    tokens.push(Token::Operator(ch));
                }
                '(' => {
                    if !current_number.is_empty() {
                        if let Ok(num) = current_number.parse::<f64>() {
                            tokens.push(Token::Number(num));
                        }
                        current_number.clear();
                    }
                    tokens.push(Token::LeftParen);
                }
                ')' => {
                    if !current_number.is_empty() {
                        if let Ok(num) = current_number.parse::<f64>() {
                            tokens.push(Token::Number(num));
                        }
                        current_number.clear();
                    }
                    tokens.push(Token::RightParen);
                }
                ' ' => {} // 忽略空格
                _ => return Err(format!("Invalid character: {}", ch)),
            }
        }
        
        if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<f64>() {
                tokens.push(Token::Number(num));
            } else {
                return Err("Invalid number".to_string());
            }
        }
        
        Ok(tokens)
    }

    fn evaluate(&self, tokens: Vec<Token>) -> Result<f64, String> {
        let postfix = self.to_postfix(tokens)?;
        self.evaluate_postfix(postfix)
    }

    fn to_postfix(&self, tokens: Vec<Token>) -> Result<Vec<Token>, String> {
        let mut output = Vec::new();
        let mut operators = Vec::new();
        
        for token in tokens {
            match token {
                Token::Number(_) => output.push(token),
                Token::Operator(op) => {
                    while let Some(Token::Operator(top_op)) = operators.last() {
                        if self.precedence(*top_op) >= self.precedence(op) {
                            output.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    operators.push(token);
                }
                Token::LeftParen => operators.push(token),
                Token::RightParen => {
                    while let Some(op) = operators.pop() {
                        if op == Token::LeftParen {
                            break;
                        }
                        output.push(op);
                    }
                }
            }
        }
        
        while let Some(op) = operators.pop() {
            output.push(op);
        }
        
        Ok(output)
    }

    fn evaluate_postfix(&self, tokens: Vec<Token>) -> Result<f64, String> {
        let mut stack = Vec::new();
        
        for token in tokens {
            match token {
                Token::Number(num) => stack.push(num),
                Token::Operator(op) => {
                    if stack.len() < 2 {
                        return Err("Invalid expression".to_string());
                    }
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    
                    let result = match op {
                        '+' => a + b,
                        '-' => a - b,
                        '*' => a * b,
                        '/' => {
                            if b == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            a / b
                        }
                        '%' => {
                            if b == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            a % b
                        }
                        _ => return Err(format!("Unknown operator: {}", op)),
                    };
                    
                    stack.push(result);
                }
                _ => return Err("Invalid token in postfix expression".to_string()),
            }
        }
        
        if stack.len() == 1 {
            Ok(stack[0])
        } else {
            Err("Invalid expression".to_string())
        }
    }

    fn precedence(&self, op: char) -> i32 {
        match op {
            '+' | '-' => 1,
            '*' | '/' | '%' => 2,
            _ => 0,
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}
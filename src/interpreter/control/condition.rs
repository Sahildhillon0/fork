use crate::token::Token;
use super::super::Interpreter;

impl Interpreter {
    /// Evaluate a simple condition: <var|num> <op> <var|num>
    pub fn eval_condition(&self, tokens: &[Token]) -> bool {
        let mut i = 0;
        while i < tokens.len() && matches!(&tokens[i], Token::Whitespace) { i += 1; }
        let left = tokens.get(i);
        let mut j = i + 1;
        while j < tokens.len() && matches!(&tokens[j], Token::Whitespace) { j += 1; }
        let op = tokens.get(j);
        let mut k = j + 1;
        while k < tokens.len() && matches!(&tokens[k], Token::Whitespace) { k += 1; }
        let right = tokens.get(k);
        let left_val = match left {
            Some(Token::Identifier(var)) => self.variables.get(var).copied().unwrap_or(0.0),
            Some(Token::Number(num)) => num.parse().unwrap_or(0.0),
            _ => 0.0
        };
        let right_val = match right {
            Some(Token::Identifier(var)) => self.variables.get(var).copied().unwrap_or(0.0),
            Some(Token::Number(num)) => num.parse().unwrap_or(0.0),
            _ => 0.0
        };
        match op {
            Some(Token::Symbol('<')) => left_val < right_val,
            Some(Token::Symbol('>')) => left_val > right_val,
            Some(Token::Symbol('=')) => {
                if let Some(Token::Symbol('=')) = tokens.get(j+1) {
                    left_val == right_val
                } else {
                    false
                }
            }
            Some(Token::Symbol('!')) => {
                if let Some(Token::Symbol('=')) = tokens.get(j+1) {
                    left_val != right_val
                } else {
                    false
                }
            }
            _ => false
        }
    }
}

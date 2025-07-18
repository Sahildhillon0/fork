use crate::token::Token;
use super::super::Interpreter;

impl Interpreter {
    pub fn interpret_while_loop(&mut self, tokens: &[Token], i: &mut usize) {
        // Find start of condition (skip "while" and whitespace)
        let mut cursor = *i + 1;
        while cursor < tokens.len() && matches!(&tokens[cursor], Token::Whitespace) { cursor += 1; }
        // Condition must be in (...)
        if cursor >= tokens.len() || !matches!(&tokens[cursor], Token::Symbol('(')) {
            return; // Invalid syntax, optionally handle error
        }
        // Find end of condition
        let (cond_start, cond_end) = {
            let mut start = cursor + 1;
            while start < tokens.len() && matches!(&tokens[start], Token::Whitespace) { start += 1; }
            let mut end = start;
            let mut paren_count = 1;
            while end < tokens.len() && paren_count > 0 {
                match &tokens[end] {
                    Token::Symbol('(') => paren_count += 1,
                    Token::Symbol(')') => paren_count -= 1,
                    _ => {}
                }
                if paren_count > 0 { end += 1; }
            }
            (start, end)
        };
        // Find start of block (skip whitespace)
        let mut block_start = cond_end + 1;
        while block_start < tokens.len() && matches!(&tokens[block_start], Token::Whitespace) { block_start += 1; }
        if block_start >= tokens.len() || !matches!(&tokens[block_start], Token::Symbol('{')) {
            return; // Invalid syntax, optionally handle error
        }
        // Find end of block
        let (body_start, body_end) = {
            let mut start = block_start + 1;
            let mut end = start;
            let mut brace_count = 1;
            while end < tokens.len() && brace_count > 0 {
                match &tokens[end] {
                    Token::Symbol('{') => brace_count += 1,
                    Token::Symbol('}') => brace_count -= 1,
                    _ => {}
                }
                if brace_count > 0 { end += 1; }
            }
            (start, end)
        };
        let condition_tokens = &tokens[cond_start..cond_end];
        let block_tokens = &tokens[body_start..body_end];
        // Main while loop logic
        while self.eval_condition(condition_tokens) {
            self.interpret(block_tokens);
            // println!("[DEBUG]block_tokens);
        }
        // Move i to end of block
        *i = body_end;
    }
}

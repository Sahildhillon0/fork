use crate::token::Token;
use super::super::Interpreter;

impl Interpreter {
    /// Interpret tokens, handling control flow (e.g. while loops)
    pub fn interpret(&mut self, tokens: &[Token]) {
    // // println!("interpret: tokens = {:?}", tokens);
        let mut i = 0;
        while i < tokens.len() {
            // // println!("[DEBUG] [TOP LEVEL] i: {}, next tokens: {:?}", i, &tokens[i..(i+5).min(tokens.len())]);
            match &tokens[i] {
                Token::Keyword(k) if k == "for" => {
                    self.interpret_for_loop(tokens, &mut i);
                    continue;
                }
                Token::Keyword(k) if k == "if" => {
                    self.interpret_if_else(tokens, &mut i);
                    continue;
                }
                Token::Keyword(k) if k == "while" => {
    self.interpret_while_loop(tokens, &mut i);
    continue;
}
                _ => {
    // Interpret one top-level statement (assignment, print, etc.)
    // Find the end of the statement (usually at a semicolon)
    let mut stmt_end = i;
    let mut in_block = false;
    let mut brace_count = 0;
    while stmt_end < tokens.len() {
        match &tokens[stmt_end] {
            Token::Symbol('{') => {
                in_block = true;
                brace_count += 1;
            }
            Token::Symbol('}') => {
                if brace_count > 0 {
                    brace_count -= 1;
                    if brace_count == 0 {
                        stmt_end += 1;
                        break;
                    }
                }
            }
            Token::Symbol(';') => {
                if !in_block {
                    stmt_end += 1;
                    break;
                }
            }
            Token::Keyword(k) if k == "for" || k == "while" => {
                break;
            }
            _ => {}
        }
        stmt_end += 1;
    }
    // Skip stray closing braces
    if matches!(&tokens[i], Token::Symbol('}')) {
        i += 1;
        continue;
    }
    // Actually execute the statement
    self.interpret_one_statement(&tokens[i..stmt_end]);
    i = stmt_end;
}
            }
        }
        i += 1;
    }
}
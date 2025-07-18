use crate::token::Token;
use super::super::Interpreter;

impl Interpreter {
    /// Interpret an if/else if/else block starting at tokens[i].
    /// Advances i to the end of the control structure.
    pub fn interpret_if_else(&mut self, tokens: &[Token], i: &mut usize) {
        let mut j = *i + 1;
        while j < tokens.len() && matches!(&tokens[j], Token::Whitespace) { j += 1; }
        if j < tokens.len() && matches!(&tokens[j], Token::Symbol('(')) {
            // Parse condition inside (...)
            let cond_start = j + 1;
            let mut cond_end = cond_start;
            let mut paren_count = 1;
            while cond_end < tokens.len() && paren_count > 0 {
                match &tokens[cond_end] {
                    Token::Symbol('(') => paren_count += 1,
                    Token::Symbol(')') => paren_count -= 1,
                    _ => {}
                }
                if paren_count > 0 { cond_end += 1; }
            }
            let cond_tokens = &tokens[cond_start..cond_end];
            let mut block_start = cond_end + 1;
            while block_start < tokens.len() && matches!(&tokens[block_start], Token::Whitespace) { block_start += 1; }
            if block_start < tokens.len() && matches!(&tokens[block_start], Token::Symbol('{')) {
                // Find end of block
                let mut block_end = block_start + 1;
                let mut brace_count = 1;
                while block_end < tokens.len() && brace_count > 0 {
                    match &tokens[block_end] {
                        Token::Symbol('{') => brace_count += 1,
                        Token::Symbol('}') => brace_count -= 1,
                        _ => {}
                    }
                    if brace_count > 0 { block_end += 1; }
                }
                let block_tokens = &tokens[block_start + 1..block_end];
                if self.eval_condition(cond_tokens) {
                    // Debug: print if block tokens
                    // println!("[DEBUG] Executing if block: {:?}", block_tokens);
                    self.interpret(block_tokens);
                } else {
                    // Check for else if or else directly after the if block
                    let mut next = block_end + 1;
                    while next < tokens.len() && matches!(&tokens[next], Token::Whitespace) { next += 1; }
                    if next < tokens.len() && matches!(&tokens[next], Token::Keyword(k) if k == "else") {
                        let mut after_else = next + 1;
                        while after_else < tokens.len() && matches!(&tokens[after_else], Token::Whitespace) { after_else += 1; }
                        if after_else < tokens.len() && matches!(&tokens[after_else], Token::Keyword(k) if k == "if") {
                            // else if
                            self.interpret_if_else(&tokens[after_else..], &mut 0);
                            return;
                        } else if after_else < tokens.len() && matches!(&tokens[after_else], Token::Symbol('{')) {
                            // else
                            let else_block_start = after_else;
                            let mut else_block_end = else_block_start + 1;
                            let mut brace_count = 1;
                            while else_block_end < tokens.len() && brace_count > 0 {
                                match &tokens[else_block_end] {
                                    Token::Symbol('{') => brace_count += 1,
                                    Token::Symbol('}') => brace_count -= 1,
                                    _ => {}
                                }
                                if brace_count > 0 { else_block_end += 1; }
                            }
                            // The block should be between { ... }, so from else_block_start+1 to else_block_end-1
                            if else_block_start + 1 <= else_block_end - 1 && else_block_end <= tokens.len() {
    let else_block_tokens = &tokens[else_block_start + 1..else_block_end];
    
    self.interpret(else_block_tokens);
} else {
    
}
                        }
                    }
                }
                // Always finish naturally

            }
        }
        // If not a valid if statement, just advance
        *i += 1;
    }
}

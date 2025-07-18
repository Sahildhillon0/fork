use crate::token::Token;
use super::super::Interpreter;

impl Interpreter {
    /// Interpret tokens, handling control flow (e.g. while loops)
    pub fn interpret(&mut self, tokens: &[Token]) {
    // println!("interpret: tokens = {:?}", tokens);
        let mut i = 0;
        while i < tokens.len() {
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
                    let mut j = i + 1;
                    while j < tokens.len() && matches!(&tokens[j], Token::Whitespace) { j += 1; }
                    if j < tokens.len() && matches!(&tokens[j], Token::Symbol('(')) {
                        let mut cond_start = j + 1;
                        while cond_start < tokens.len() && matches!(&tokens[cond_start], Token::Whitespace) { cond_start += 1; }
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
                        if paren_count == 0 {
                            let mut block_start = cond_end + 1;
                            while block_start < tokens.len() && matches!(&tokens[block_start], Token::Whitespace) { block_start += 1; }
                            if block_start < tokens.len() && matches!(&tokens[block_start], Token::Symbol('{')) {
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
                                if brace_count == 0 {
                                    let condition_tokens = &tokens[cond_start..cond_end];
                                    while self.eval_condition(condition_tokens) {
    if let Some(val) = self.variables.get("x") {
        // debug removed x = {}", val);
    }

                                        let block_tokens = &tokens[block_start + 1..block_end];
                                        let mut k = 0;
                                        while k < block_tokens.len() {
                                            // Find end of next statement
                                            let mut stmt_end = k;
                                            let mut in_block = false;
                                            let mut brace_count = 0;
                                            while stmt_end < block_tokens.len() {
                                                match &block_tokens[stmt_end] {
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
                                            if stmt_end > k {
                                                self.interpret_one_statement(&block_tokens[k..stmt_end]);
                                            }
                                            k = stmt_end;
                                        }
                                        // Ensure the last statement is executed if any tokens remain
                                        if k < block_tokens.len() {
                                            self.interpret_one_statement(&block_tokens[k..]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
    let mut stmt_end = i;
    let mut in_block = false;
    let mut brace_count = 0;
    while stmt_end < tokens.len() {
        match &tokens[stmt_end] {
            Token::Symbol('{') => {
                in_block = true;
                brace_count += 1;
            },
            Token::Symbol('}') => {
                if brace_count > 0 {
                    brace_count -= 1;
                    if brace_count == 0 {
                        stmt_end += 1;
                        break;
                    }
                }
            },
            Token::Symbol(';') => {
                if !in_block {
                    stmt_end += 1;
                    break;
                }
            },
            Token::Keyword(k) if k == "for" || k == "while" => {
                break;
            },
            _ => {}
        }
        stmt_end += 1;
    }
    if stmt_end > i {
        self.interpret_one_statement(&tokens[i..stmt_end]);
    }
    i = stmt_end;
}
            }
        }
    }
}

use crate::token::Token;
use super::super::Interpreter;

impl Interpreter {
    pub fn interpret_for_loop(&mut self, tokens: &[Token], i: &mut usize) {
        // Parse for (init; cond; update) { block }
        let mut j = *i + 1;
        while j < tokens.len() && matches!(&tokens[j], Token::Whitespace) { j += 1; }
        if j < tokens.len() && matches!(&tokens[j], Token::Symbol('(')) {
            let header_start = j + 1;
            let mut header_end = header_start;
            let mut paren_count = 1;
            while header_end < tokens.len() && paren_count > 0 {
                match &tokens[header_end] {
                    Token::Symbol('(') => paren_count += 1,
                    Token::Symbol(')') => paren_count -= 1,
                    _ => {}
                }
                if paren_count > 0 { header_end += 1; }
            }
            if paren_count == 0 {
                // Split header by ';'
                let mut semi_indices = vec![];
                for idx in header_start..header_end {
                    if matches!(&tokens[idx], Token::Symbol(';')) {
                        semi_indices.push(idx);
                    }
                }
                // Check for type keywords or colons in header
                let header_tokens = &tokens[header_start..header_end];
                if header_tokens.iter().any(|t| matches!(t, Token::Keyword(k) if k == "int" || k == "float" || k == "let")) {
                    eprintln!("SyntaxError: variable declarations are not supported in for-loop headers.");
                    // Skip to after the for-loop block
                    let mut block_start = header_end + 1;
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
                        *i = block_end + 1;
                    } else {
                        *i = header_end + 1;
                    }
                    return;
                } else if header_tokens.iter().any(|t| matches!(t, Token::Unknown(s) if s == ":")) {
                    eprintln!("SyntaxError: use semicolons ';' to separate for-loop header parts.");
                    // Skip to after the for-loop block
                    let mut block_start = header_end + 1;
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
                        *i = block_end + 1;
                    } else {
                        *i = header_end + 1;
                    }
                    return;
                } else if semi_indices.len() == 2 {
                    let init_tokens = &tokens[header_start..semi_indices[0]];
                    let cond_tokens = &tokens[semi_indices[0]+1..semi_indices[1]];
                    let update_tokens = &tokens[semi_indices[1]+1..header_end];
                    // Find block start '{'
                    let mut block_start = header_end + 1;
                    while block_start < tokens.len() && matches!(&tokens[block_start], Token::Whitespace) { block_start += 1; }
                    if block_start < tokens.len() && matches!(&tokens[block_start], Token::Symbol('{')) {
                        // Find block end '}'
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
                            // Execute init
                            self.interpret_one_statement(init_tokens);
                            // Loop
                            while self.eval_condition(cond_tokens) {
                                let block_tokens = &tokens[block_start + 1..block_end];
                                // Custom statement splitter: treat if/else as one statement
                                let mut k = 0;
                                while k < block_tokens.len() {
                                    // Skip whitespace
                                    while k < block_tokens.len() && matches!(&block_tokens[k], Token::Whitespace) { k += 1; }
                                    if k >= block_tokens.len() { break; }
                                    let start = k;
                                    let mut end = k + 1;
                                    // If statement: scan for full if/else chain
                                    if matches!(&block_tokens[start], Token::Keyword(s) if s == "if") {
                                        // Find condition (...)
                                        while end < block_tokens.len() && matches!(&block_tokens[end], Token::Whitespace) { end += 1; }
                                        if end < block_tokens.len() && matches!(&block_tokens[end], Token::Symbol('(')) {
                                            let mut paren_count = 1;
                                            end += 1;
                                            while end < block_tokens.len() && paren_count > 0 {
                                                match &block_tokens[end] {
                                                    Token::Symbol('(') => paren_count += 1,
                                                    Token::Symbol(')') => paren_count -= 1,
                                                    _ => {}
                                                }
                                                end += 1;
                                            }
                                            // Find block {...}
                                            while end < block_tokens.len() && matches!(&block_tokens[end], Token::Whitespace) { end += 1; }
                                            if end < block_tokens.len() && matches!(&block_tokens[end], Token::Symbol('{')) {
                                                let mut brace_count = 1;
                                                end += 1;
                                                while end < block_tokens.len() && brace_count > 0 {
                                                    match &block_tokens[end] {
                                                        Token::Symbol('{') => brace_count += 1,
                                                        Token::Symbol('}') => brace_count -= 1,
                                                        _ => {}
                                                    }
                                                    end += 1;
                                                }
                                            }
                                            // else/else if chains
                                            loop {
                                                let mut after = end;
                                                while after < block_tokens.len() && matches!(&block_tokens[after], Token::Whitespace) { after += 1; }
                                                if after < block_tokens.len() && matches!(&block_tokens[after], Token::Keyword(s) if s == "else") {
                                                    end = after + 1;
                                                    while end < block_tokens.len() && matches!(&block_tokens[end], Token::Whitespace) { end += 1; }
                                                    if end < block_tokens.len() && matches!(&block_tokens[end], Token::Keyword(s) if s == "if") {
                                                        // else if (...)
                                                        let mut paren_count = 1;
                                                        let mut tmp = end + 1;
                                                        while tmp < block_tokens.len() && matches!(&block_tokens[tmp], Token::Whitespace) { tmp += 1; }
                                                        if tmp < block_tokens.len() && matches!(&block_tokens[tmp], Token::Symbol('(')) {
                                                            tmp += 1;
                                                            while tmp < block_tokens.len() && paren_count > 0 {
                                                                match &block_tokens[tmp] {
                                                                    Token::Symbol('(') => paren_count += 1,
                                                                    Token::Symbol(')') => paren_count -= 1,
                                                                    _ => {}
                                                                }
                                                                tmp += 1;
                                                            }
                                                        }
                                                        while tmp < block_tokens.len() && matches!(&block_tokens[tmp], Token::Whitespace) { tmp += 1; }
                                                        if tmp < block_tokens.len() && matches!(&block_tokens[tmp], Token::Symbol('{')) {
                                                            let mut brace_count = 1;
                                                            tmp += 1;
                                                            while tmp < block_tokens.len() && brace_count > 0 {
                                                                match &block_tokens[tmp] {
                                                                    Token::Symbol('{') => brace_count += 1,
                                                                    Token::Symbol('}') => brace_count -= 1,
                                                                    _ => {}
                                                                }
                                                                tmp += 1;
                                                            }
                                                        }
                                                        end = tmp;
                                                        continue;
                                                    } else if end < block_tokens.len() && matches!(&block_tokens[end], Token::Symbol('{')) {
                                                        let mut brace_count = 1;
                                                        end += 1;
                                                        while end < block_tokens.len() && brace_count > 0 {
                                                            match &block_tokens[end] {
                                                                Token::Symbol('{') => brace_count += 1,
                                                                Token::Symbol('}') => brace_count -= 1,
                                                                _ => {}
                                                            }
                                                            end += 1;
                                                        }
                                                        break;
                                                    } else {
                                                        break;
                                                    }
                                                } else {
                                                    break;
                                                }
                                            }
                                        }
                                    } else {
                                        // Not an if: scan to next ; or top-level }
                                        let mut in_block = 0;
                                        while end < block_tokens.len() {
                                            match &block_tokens[end] {
                                                Token::Symbol('{') => in_block += 1,
                                                Token::Symbol('}') => if in_block == 0 { break; } else { in_block -= 1; },
                                                Token::Symbol(';') => if in_block == 0 { end += 1; break; },
                                                _ => {}
                                            }
                                            end += 1;
                                        }
                                    }
                                    if end > start {
                                        self.interpret_one_statement(&block_tokens[start..end]);
                                    }
                                    k = end;
                                }
                                self.interpret_one_statement(update_tokens);
                            }
                            *i = block_end + 1;
                        } else {
                            eprintln!("SyntaxError: unmatched '{{' in for loop block.");
                        }
                    } else {
                        eprintln!("SyntaxError: expected '{{' after for loop header.");
                    }
                } else {
                    eprintln!("SyntaxError: malformed for loop header. Expected two semicolons.");
                }
            } else {
                eprintln!("SyntaxError: unmatched '(' in for loop header.");
                // Skip to after the for-loop block
                let mut block_start = header_end;
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
                    *i = block_end + 1;
                } else {
                    *i = block_start + 1;
                }
                return;
            }
        } else {
            eprintln!("SyntaxError: expected '(' after 'for'.");
        }
    }
}

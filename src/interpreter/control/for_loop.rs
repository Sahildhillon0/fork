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
                                self.interpret(block_tokens);
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

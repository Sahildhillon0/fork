//! Interpreter control flow logic: while loops, condition evaluation.
use crate::token::Token;
use super::Interpreter;

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

    /// Interpret tokens, handling control flow (e.g. while loops)
    pub fn interpret(&mut self, tokens: &[Token]) {
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                Token::Keyword(k) if k == "for" => {
                    // Parse for (init; cond; update) { block }
                    let mut j = i + 1;
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
                                    i = block_end + 1;
                                } else {
                                    i = header_end + 1;
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
                                    i = block_end + 1;
                                } else {
                                    i = header_end + 1;
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
                                i = block_end + 1;
                            } else {
                                i = block_start + 1;
                            }
                            return;
                        }
                    } else {
                        eprintln!("SyntaxError: expected '(' after 'for'.");
                    }
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
                                        let block_tokens = &tokens[block_start + 1..block_end];
                                        self.interpret(block_tokens);
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
                            _ => {}
                        }
                        stmt_end += 1;
                    }
                    self.interpret_one_statement(&tokens[i..stmt_end]);
                    i = stmt_end;
                }
            }
        }
    }
}

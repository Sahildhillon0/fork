//! Interpreter statement logic: assignment, print, etc.
use crate::token::Token;
use super::Interpreter;

impl Interpreter {
    /// Execute a single assignment or print statement from a slice
    pub fn interpret_one_statement(&mut self, tokens: &[Token]) {
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                // Handle assignment: Identifier = ... ;
                Token::Identifier(var) => {
                    let mut j = i + 1;
                    while j < tokens.len() && matches!(&tokens[j], Token::Whitespace) { j += 1; }
                    if j < tokens.len() && matches!(&tokens[j], Token::Symbol('=')) {
                        let mut k = j + 1;
                        while k < tokens.len() && matches!(&tokens[k], Token::Whitespace) { k += 1; }
                        // Support: x = 1;
                        if k < tokens.len() && matches!(&tokens[k], Token::Number(_)) {
                            let mut l = k + 1;
                            while l < tokens.len() && matches!(&tokens[l], Token::Whitespace) { l += 1; }
                            if l < tokens.len() && matches!(&tokens[l], Token::Symbol(';')) {
                                let val = match &tokens[k] { Token::Number(n) => n.parse::<f64>().unwrap_or(0.0), _ => 0.0 };
                                self.variables.insert(var.clone(), val);
                                return;
                            } else {
                                eprintln!("SyntaxError: missing semicolon at end of assignment.");
                                return;
                            }
                        }
                        // Support: x = y + 1; or x = y - 1;
                        if k + 2 < tokens.len() &&
                            (matches!(&tokens[k], Token::Identifier(_)) || matches!(&tokens[k], Token::Number(_))) &&
                            matches!(&tokens[k+1], Token::Symbol(op) if *op == '+' || *op == '-') &&
                            (matches!(&tokens[k+2], Token::Identifier(_)) || matches!(&tokens[k+2], Token::Number(_))) {
                            let mut l = k + 3;
                            while l < tokens.len() && matches!(&tokens[l], Token::Whitespace) { l += 1; }
                            if l < tokens.len() && matches!(&tokens[l], Token::Symbol(';')) {
                                let left = match &tokens[k] {
                                    Token::Identifier(v) => *self.variables.get(v).unwrap_or(&0.0),
                                    Token::Number(n) => n.parse::<f64>().unwrap_or(0.0),
                                    _ => 0.0
                                };
                                let right = match &tokens[k+2] {
                                    Token::Identifier(v) => *self.variables.get(v).unwrap_or(&0.0),
                                    Token::Number(n) => n.parse::<f64>().unwrap_or(0.0),
                                    _ => 0.0
                                };
                                let result = match &tokens[k+1] {
                                    Token::Symbol('+') => left + right,
                                    Token::Symbol('-') => left - right,
                                    _ => left
                                };
                                self.variables.insert(var.clone(), result);
                                return;
                            }
                        }
                    }
                    i += 1;
                }
                Token::Keyword(k) if k == "print" => {
                    let mut j = i + 1;
                    while j < tokens.len() && matches!(&tokens[j], Token::Whitespace) { j += 1; }
                    if j < tokens.len() && matches!(&tokens[j], Token::Symbol('(')) {
                        let mut args = Vec::new();
                        let mut k = j + 1;
                        loop {
                            while k < tokens.len() && matches!(&tokens[k], Token::Whitespace) { k += 1; }
                            if k < tokens.len() && matches!(&tokens[k], Token::Symbol(')')) { break; }
                            if k < tokens.len() && (matches!(&tokens[k], Token::Identifier(_)) || matches!(&tokens[k], Token::StringLiteral(_))) {
                                args.push(k);
                                k += 1;
                                while k < tokens.len() && matches!(&tokens[k], Token::Whitespace) { k += 1; }
                                if k < tokens.len() && matches!(&tokens[k], Token::Symbol(',')) { k += 1; continue; }
                            } else { break; }
                        }
                        if k < tokens.len() && matches!(&tokens[k], Token::Symbol(')')) {
                            let mut m = k + 1;
                            while m < tokens.len() && matches!(&tokens[m], Token::Whitespace) { m += 1; }
                            if m < tokens.len() && matches!(&tokens[m], Token::Symbol(';')) {
                                let mut output = String::new();
                                for (idx, &arg_idx) in args.iter().enumerate() {
                                    if idx > 0 { output.push(' '); }
                                    match &tokens[arg_idx] {
                                        Token::Identifier(var) => {
                                            if let Some(val) = self.variables.get(var) {
                                                output.push_str(&val.to_string());
                                            } else {
                                                output.push_str(&format!("NameError: name '{}' is not defined", var));
                                            }
                                        },
                                        Token::StringLiteral(s) => { output.push_str(s); },
                                        _ => {}
                                    }
                                }
                                println!("{}", output);
                                return;
                            }
                        }
                    }
                    i += 1;
                }
                Token::Whitespace => { i += 1; }
                _ => {
                    if i < tokens.len() {
                        let token = &tokens[i];
                        let msg = match token {
                            Token::Keyword(k) => format!("SyntaxError: unknown or unsupported keyword '{}'.", k),
                            Token::Identifier(id) => format!("SyntaxError: unexpected identifier '{}'.", id),
                            Token::Symbol(s) => format!("SyntaxError: unexpected symbol '{}'.", s),
                            Token::Number(n) => format!("SyntaxError: unexpected number '{}'.", n),
                            Token::StringLiteral(s) => format!("SyntaxError: unexpected string literal '{}'.", s),
                            Token::Whitespace => String::new(),
                            Token::Unknown(u) => format!("SyntaxError: unknown token '{}'.", u),
                        };
                        if !msg.is_empty() {
                            eprintln!("{}", msg);
                        }
                    }
                    // skip to next semicolon or end
                    while i < tokens.len() && !matches!(&tokens[i], Token::Symbol(';')) { i += 1; }
                    i += 1;
                }
            }
        }
    }
}
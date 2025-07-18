//! Interpreter statement logic: assignment, print, etc.
use crate::token::Token;
use super::Interpreter;

pub mod assignment;
pub mod print;

impl Interpreter {


    /// Execute a single assignment or print statement from a slice
    pub fn interpret_one_statement(&mut self, tokens: &[Token]) {
    // debug removed interpret_one_statement tokens: {:?}", tokens);
    // println!("interpret_one_statement: tokens = {:?}", tokens);
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                // Handle assignment: Identifier = ... ;
                // Also handle increment/decrement: i++ or i--
                Token::Identifier(var) => {
                    let mut j = i + 1;
                    while j < tokens.len() && matches!(&tokens[j], Token::Whitespace) { j += 1; }
                    // Handle increment: i++;
                    if j + 1 < tokens.len() && matches!(&tokens[j], Token::Symbol('+')) && matches!(&tokens[j+1], Token::Symbol('+')) {
                        let mut k = j + 2;
                        while k < tokens.len() && matches!(&tokens[k], Token::Whitespace) { k += 1; }
                        if k < tokens.len() && matches!(&tokens[k], Token::Symbol(';')) {
                            if let Some(val) = self.variables.get_mut(var) {
                                *val += 1.0;
                            } else {
                                eprintln!("NameError: name '{}' is not defined", var);
                            }
                            return;
                        }
                    }
                    // Handle decrement: i--;
                    if j + 1 < tokens.len() && matches!(&tokens[j], Token::Symbol('-')) && matches!(&tokens[j+1], Token::Symbol('-')) {
                        let mut k = j + 2;
                        while k < tokens.len() && matches!(&tokens[k], Token::Whitespace) { k += 1; }
                        if k < tokens.len() && matches!(&tokens[k], Token::Symbol(';')) {
                            if let Some(val) = self.variables.get_mut(var) {
                                *val -= 1.0;
                            } else {
                                eprintln!("NameError: name '{}' is not defined", var);
                            }
                            return;
                        }
                    }
                    let mut j = i + 1;
                    while j < tokens.len() && matches!(&tokens[j], Token::Whitespace) { j += 1; }
                    if j < tokens.len() && matches!(&tokens[j], Token::Symbol('=')) {
                        let mut k = j + 1;
                        while k < tokens.len() && matches!(&tokens[k], Token::Whitespace) { k += 1; }
                        // Support: x = 1; or x = 1
                        if k < tokens.len() && matches!(&tokens[k], Token::Number(_)) {
                            let val = match &tokens[k] { Token::Number(n) => n.parse::<f64>().unwrap_or(0.0), _ => 0.0 };
                            self.variables.insert(var.clone(), val);
                            // skip to next semicolon (if present), but always execute assignment
                            return;
                        }
                        // Support: x = y + 1; or x = y + 1 + 2 - z, etc.
                        // Evaluate the right-hand side as a left-associative chain of + and -
                        let mut acc = None;
                        let mut op = None;
                        let mut idx = k;
                        while idx < tokens.len() {
                            match &tokens[idx] {
                                Token::Whitespace => { idx += 1; },
                                Token::Identifier(v) => {
                                    let val = *self.variables.get(v).unwrap_or(&0.0);
                                    acc = Some(match (acc, op) {
                                        (None, _) => val,
                                        (Some(lhs), Some('+')) => lhs + val,
                                        (Some(lhs), Some('-')) => lhs - val,
                                        (Some(lhs), Some('*')) => lhs * val,
                                        (Some(lhs), Some('/')) => lhs / val,
                                        _ => val
                                    });
                                    op = None;
                                    idx += 1;
                                },
                                Token::Number(n) => {
                                    let val = n.parse::<f64>().unwrap_or(0.0);
                                    acc = Some(match (acc, op) {
                                        (None, _) => val,
                                        (Some(lhs), Some('+')) => lhs + val,
                                        (Some(lhs), Some('-')) => lhs - val,
                                        (Some(lhs), Some('*')) => lhs * val,
                                        (Some(lhs), Some('/')) => lhs / val,
                                        _ => val
                                    });
                                    op = None;
                                    idx += 1;
                                },
                                Token::Symbol(s) if *s == '+' || *s == '-' || *s == '*' || *s == '/' => {
                                    op = Some(*s);
                                    idx += 1;
                                },
                                Token::Symbol(';') => {
                                    break;
                                },
                                _ => { break; }
                            }
                        }
                        if let Some(val) = acc {
                            self.variables.insert(var.clone(), val);
                            return;
                        }
                        // If we get here, it's an assignment but not a recognized form; emit error
                        eprintln!("SyntaxError: unsupported assignment expression.");
                        return;
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
                            // Support arithmetic expressions as print arguments
                            if k < tokens.len() {
                                // Parse an expression until ',' or ')'
                                let expr_start = k;
                                let mut expr_end = k;
                                let mut paren_level = 0;
                                while expr_end < tokens.len() {
                                    match &tokens[expr_end] {
                                        Token::Symbol(',') if paren_level == 0 => break,
                                        Token::Symbol(')') if paren_level == 0 => break,
                                        Token::Symbol('(') => paren_level += 1,
                                        Token::Symbol(')') => paren_level -= 1,
                                        _ => {}
                                    }
                                    expr_end += 1;
                                }
                                if expr_start < expr_end {
                                    args.push((expr_start, expr_end));
                                }
                                k = expr_end;
                                while k < tokens.len() && matches!(&tokens[k], Token::Whitespace) { k += 1; }
                                if k < tokens.len() && matches!(&tokens[k], Token::Symbol(',')) { k += 1; continue; }
                            } else { break; }
                        }
                        if k < tokens.len() && matches!(&tokens[k], Token::Symbol(')')) {
                            let mut m = k + 1;
                            while m < tokens.len() && matches!(&tokens[m], Token::Whitespace) { m += 1; }
                            if m < tokens.len() && matches!(&tokens[m], Token::Symbol(';')) {
                                let mut output = String::new();
                                for (idx, &(start, end)) in args.iter().enumerate() {
                                    if idx > 0 { output.push(' '); }
                                    let val = assignment::eval_expression(self,&tokens[start..end]);
                                    output.push_str(&val);
                                }
                                println!("{}", output);
                                return;
                            }
                        }
                    }
                    i += 1;
                }
                Token::Whitespace => { i += 1; }
                Token::Keyword(k) if k == "if" => {
                    self.interpret_if_else(&tokens[i..], &mut i);
                    return;
                }
                Token::Keyword(k) if k == "else" => {
                    // 'else' is handled as part of 'if', so just skip
                    i += 1;
                    return;
                }
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
// End of impl Interpreter
}
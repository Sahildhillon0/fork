use crate::token::Token;
use super::super::Interpreter;

pub fn eval_expression(interp: &Interpreter, tokens: &[Token]) -> String {
    // If the whole expression is a single string literal, return it
    if tokens.len() == 1 {
        if let Token::StringLiteral(s) = &tokens[0] {
            return s.clone();
        }
    }
    let mut acc = None;
    let mut op: Option<char> = None;
    let mut idx = 0;
    while idx < tokens.len() {
        match &tokens[idx] {
            Token::Whitespace => { idx += 1; },
            Token::Identifier(v) => {
                let val = *interp.variables.get(v).unwrap_or(&0.0);
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
                // If there's a pending operator and a previous operand, process them
                if let Some(opc) = op {
                    if idx > 0 {
                        let prev = &tokens[idx-1];
                        let val = match prev {
                            Token::Number(n) => n.parse::<f64>().unwrap_or(0.0),
                            Token::Identifier(v) => *interp.variables.get(v).unwrap_or(&0.0),
                            _ => 0.0
                        };
                        acc = Some(match (acc, Some(opc)) {
                            (Some(lhs), Some('+')) => lhs + val,
                            (Some(lhs), Some('-')) => lhs - val,
                            (Some(lhs), Some('*')) => lhs * val,
                            (Some(lhs), Some('/')) => lhs / val,
                            _ => val
                        });
                    }
                }
                break;
            },
            _ => { break; }
        }
    }
    if let Some(val) = acc {
        val.to_string()
    } else {
        "0".to_string()
    }
}

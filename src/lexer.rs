use std::collections::HashSet;
use crate::token::Token;

/// The Lexer is responsible for converting source code into a stream of tokens.
pub struct Lexer {
    keywords: HashSet<&'static str>,
}

impl Lexer {
    /// Creates a new Lexer with a set of recognized keywords.
    pub fn new() -> Self {
        let keywords = [
            "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with", "yield", "print"
        ]
        .iter()
        .copied()
        .collect();
        Self { keywords }
    }

    /// Tokenizes the input string into a vector of tokens.
    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.next();
                tokens.push(Token::Whitespace);
            } else if ch == '"' {
                chars.next();
                let mut literal = String::new();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == '"' {
                        break;
                    }
                    literal.push(c);
                }
                tokens.push(Token::StringLiteral(literal));
            } else if ch.is_ascii_digit() {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number));
            } else if ch.is_ascii_alphabetic() || ch == '_' {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if self.keywords.contains(ident.as_str()) {
                    tokens.push(Token::Keyword(ident));
                } else {
                    tokens.push(Token::Identifier(ident));
                }
            } else if Self::is_symbol(ch) {
                tokens.push(Token::Symbol(ch));
                chars.next();
            } else {
                let mut unknown = String::new();
                unknown.push(ch);
                chars.next();
                tokens.push(Token::Unknown(unknown));
            }
        }
        tokens
    }

    /// Returns true if the given character is a recognized symbol.
    fn is_symbol(ch: char) -> bool {
        matches!(ch, ',' | ';' | '(' | ')' | '{' | '}' | '[' | ']' | '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '%')
    }
}

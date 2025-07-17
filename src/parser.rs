use crate::token::Token;

/// The Parser is responsible for converting a stream of tokens into an AST or intermediate structure.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Creates a new Parser from a vector of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the token stream.
    pub fn parse(&mut self) {
        // For demonstration, this is a stub.
        while let Some(_token) = self.next_token() {
            // Parsing logic would go here
        }
    }

    /// Returns the next token, if any.
    fn next_token(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let tok = &self.tokens[self.pos];
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }
}

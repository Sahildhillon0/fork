/// Represents a lexical token in the Fork language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Keywords like 'while', 'print', etc.
    Keyword(String),
    /// Identifiers (variable names)
    Identifier(String),
    /// Numeric literals
    Number(String),
    /// String literals
    StringLiteral(String),
    /// Symbols like '+', '-', '=', etc.
    Symbol(char),
    /// Whitespace (for token stream separation)
    Whitespace,
    /// Unknown or invalid tokens
    Unknown(String),
}

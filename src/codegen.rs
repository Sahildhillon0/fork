use crate::token::Token;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator
    }

    pub fn generate(&self, tokens: &[Token]) {
        // For demonstration, just print a stub for real code generation

        for (_i, _token) in tokens.iter().enumerate() {

        }

    }
}

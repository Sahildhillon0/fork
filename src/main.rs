mod token;
mod lexer;
mod parser;
mod semantic;
mod icg;
mod optimizer;
mod codegen;
mod interpreter;

use std::env;
use std::fs;
use std::io::{self, Write};
use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;
use icg::ICG;
use optimizer::Optimizer;
use codegen::CodeGenerator;
use interpreter::Interpreter;

fn main() {
    let lexer = Lexer::new();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // File mode
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(content) => {
                let tokens = lexer.tokenize(&content);
                let mut interpreter = Interpreter::new();
                interpreter.interpret(&tokens);
            },
            Err(e) => {
                eprintln!("Error reading file {}: {}", filename, e);
                return;
            }
        }
    } else {
        // REPL mode
        let mut interpreter = Interpreter::new();
        let stdin = io::stdin();
        loop {
            print!("fork> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            if stdin.read_line(&mut input).is_err() || input.trim().is_empty() {
                break;
            }
            let trimmed = input.trim();
            if trimmed == "exit" || trimmed == "quit" {
                break;
            }
            let tokens = lexer.tokenize(trimmed);
            // No pipeline in REPL, just interpret
            interpreter.interpret(&tokens);
        }
    }

}

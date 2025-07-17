//! Interpreter context: holds variable state for execution.
use std::collections::HashMap;

/// Holds the variable context for the interpreter.
pub struct Interpreter {
    pub(crate) variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }
}

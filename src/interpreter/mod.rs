//! The Interpreter receives STIR data as an input and executes it

use colored::Colorize;

pub struct Interpreter {}

impl Interpreter {
    /// Create a new interpreter
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    /// Parse and execute a line of STIR code
    // FIXME: Use interpreter::Error instead
    pub fn execute(&self, input: &String) -> Result<(), std::io::Error> {
        match input.eq("") {
            true => Ok(()),
            // FIXME: Use actual Error type and Error message
            false => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{}", "Invalid syntax".red()),
            )),
        }
    }
}

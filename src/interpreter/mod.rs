//! The Interpreter receives STIR data as an input and executes it

use colored::Colorize;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn execute(&self, input: &String) -> Result<(), std::io::Error> {
        match input.eq("") {
            true => Ok(()),
            false => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("{}", "Invalid syntax".red()))),
        }
    }
}

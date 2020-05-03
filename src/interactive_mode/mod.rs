//! Interactive mode for the `fry` interpreter

use linefeed::{Interface, ReadResult};

mod prompt;
use prompt::Prompt;

use crate::interpreter::Interpreter;

/// Start the interactive interpreter
pub fn launch() -> Result<(), std::io::Error> {
    let prompt = Prompt::new();
    let line_reader = Interface::new("fry")?;
    let interpreter = Interpreter::new();

    line_reader.set_prompt(prompt.get())?;

    while let ReadResult::Input(input) = line_reader.read_line().unwrap() {
        match interpreter.execute(&input) {
            Err(e) => println!("{}", e),
            _ => (),
        }
    }

    Ok(())
}

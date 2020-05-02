//! Interactive mode for the `fry` interpreter

use linefeed::{Interface, ReadResult};

mod prompt;

use prompt::Prompt;

pub fn launch() -> Result<(), std::io::Error> {
    let prompt = Prompt::new();
    let line_reader = Interface::new("fry")?;

    line_reader.set_prompt(prompt.get())?;

    while let ReadResult::Input(input) = line_reader.read_line().unwrap() {
        println!("{}", input);
    }

    Ok(())
}

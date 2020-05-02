//! Interactive mode for the `fry` interpreter

use linefeed::{Interface, ReadResult};

mod prompt;

use prompt::Prompt;

pub fn launch() {
    let prompt = Prompt::new();
    let line_reader = Interface::new("fry").unwrap();

    line_reader.set_prompt(prompt.get()).unwrap();

    while let ReadResult::Input(input) = line_reader.read_line().unwrap() {
        println!("{}", input);
    }
}

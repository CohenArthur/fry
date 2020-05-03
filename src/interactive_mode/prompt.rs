//! The Prompt is the text showed at the beginning of the line in interactive
//! mode. It changes based on the context.

use colored::Colorize;

pub struct Prompt {
    data: String,
}

impl Prompt {
    pub fn new() -> Prompt {
        Prompt {
            data: "in: ".purple().to_string(),
        }
    }

    pub fn get(&self) -> &String {
        &self.data
    }
}

//! The Prompt is the text showed at the beginning of the line in interactive
//! mode. It changes based on the context

pub struct Prompt {
    data: String,
}

impl Prompt {
    pub fn new() -> Prompt {
        Prompt {
            data: "in: ".to_string(),
        }
    }

    pub fn get(&self) -> &String {
        &self.data
    }
}

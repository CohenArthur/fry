mod interactive_mode;
mod interpreter;
mod lexer;

fn main() {
    interactive_mode::launch().unwrap();
}

use std::io;

use std::io::Write;
use std::env;
use std::path::PathBuf;

mod parser;
mod builtin;
mod utils;

fn main() {
    loop {
        let pwd = env::current_dir().expect("Failed to get working directory");
        print!("{} > ", pwd.to_string_lossy().to_string());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        // remove trailing newline
        if input.ends_with('\n') {
            input.pop();
        }

        parser::parse_input(&input);
    }
}

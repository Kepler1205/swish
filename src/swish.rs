use std::io;

use std::io::Write;
use std::env;
use std::path::PathBuf;

mod parser;
mod builtin;
mod utils;

// entry point
fn main() -> io::Result<()> {
    loop {
        let pwd = env::current_dir().expect("Failed to get working directory");
        print!("{} > ", pwd.to_string_lossy().to_string());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        if input.trim_end() == String::from("exit") { break; }

        // remove trailing newline for parser
        if input.ends_with('\n') { input.pop(); }

        parser::parse_input(&input);

    }
    Ok(())
}

use std::io;
use std::io::Write;
use std::env;

mod parser;
mod builtin;

// converts /home/user/path to ~/path
// fn prompt_pwd(dir: String) -> String 
// {
//
//     match env::var("HOME") {
//         Ok(home) => {
//
//         }
//         Err(err) => {
//             eprintln!("{}", err);
//             dir
//         }
//     }
// }

// use std::path::PathBuf;

fn main() 
{
    loop {
        let pwd = env::current_dir().expect("Failed to get working directory");
        print!("{} > ", pwd.to_string_lossy().to_string());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        // extra space is needed for parser
        // to parse the last argument
        if input.ends_with('\n') {
            input.pop();
        }
        // if !input.ends_with(" ") {
        //     input.push(' ');
        // }
        parser::parse_input(&input);
    }
}

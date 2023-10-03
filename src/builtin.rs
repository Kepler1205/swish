use crate::utils::{Token, TokenType, Type};
use std::path::PathBuf;

pub static mut CD_HIST: Vec<PathBuf> = Vec::new();
// list of built-in functions
const FUNCTIONS: [&str; 3] = ["cd", "if", "pwd"];
// list of built-in operators
const OPERATORS: [&str; 4] = ["+", "-", "*", "/"];

pub fn is_builtin_function(input: &str) -> bool {
    FUNCTIONS.contains(&input)
}

pub fn is_builtin_operator(input: &str) -> bool {
    OPERATORS.contains(&input)
}

// takes a 
pub fn handle_builtin(input: &Vec<Token>) -> Option<String> {
    // no return value functions
    match input[0].body.as_str() {
        "cd" => function::cd(&input),
        _ => (),
    };
    None
}

mod function
{
    use crate::utils::{Token, TokenType};
    use std::env;


    // change directory
    pub fn cd(argv: &Vec<Token>) {
        use std::path::PathBuf;

        match argv.len() {
            1 => {
                let home_path = PathBuf::from(env!("HOME"));
                if let Err(e) = env::set_current_dir(home_path) {
                    eprintln!("cd: {}", e)
                }
            }
            2 => {
                if argv[1].body == "-" {
                    if let Err(e) = env::set_current_dir(PathBuf::from("/")) {
                        eprintln!("cd {}", e);
                    }
                } else {
                    let target_path = PathBuf::from(argv[1].body.clone());
                    if let Err(e) = env::set_current_dir(target_path) {
                        eprintln!("cd: {}", e);
                    } 
                }
            }
            _ => {
                eprintln!("cd: too many arguments")
            }
        };
    }

    // create an integer variable
    fn int(argv: &Vec<Token>) {

    }
}

mod r_function
{
    use crate::utils::{ Token, TokenType };

    // if statement
    // if conditional {
    //      code
    // }
    //
    // or
    //
    // if conditional {
    //      code
    // } else {
    //      other code
    // }
    //
    // a conditional must evaluate to a number.
    // 0 is false and !0 is true
    // if (5 - 5) {} is valid
    // if ()
    //
    pub fn if_block(argv: &Vec<String>) -> bool {
        if argv.len() > 3 {
            eprintln!("if: too many arguments");
        }

        // if argv[2] ==

        false
    }

}

mod operator 
{
    // mathematical operators
    pub fn plus     (a: f64, b: f64) -> f64 {a + b}
    pub fn minus    (a: f64, b: f64) -> f64 {a - b}
    pub fn multiply (a: f64, b: f64) -> f64 {a * b}
    pub fn divide   (a: f64, b: f64) -> f64 {a / b}
    pub fn modulo   (a: f64, b: f64) -> f64 {a % b}

    // string operators
    pub fn str_plus (a: &String, b: &String) -> String {a.clone() + b}

    // one-line if statement using C style syntax
    // conditional ? do-thing : do-other-thing
    pub fn if_line (argv: &Vec<String>) {

    }
}

pub mod utils 
{
    pub enum Types {
        Int,
        Float,
        String,
        Bool,
    }

    impl Types {

    }

    // list of built-in functions
    const FUNCTIONS: [&str; 3] = ["cd", "if", "pwd"];
    // list of built-in operators
    const OPERATORS: [&str; 4] = ["+", "-", "*", "/"];

    fn is_builtin_function(input: &str) -> bool 
    {
        FUNCTIONS.contains(&input)
    }

    fn is_builtin_operator(input: &str) -> bool 
    {
        OPERATORS.contains(&input)
    }

    pub fn handle_builtin(input: &str) -> Option<String>
    {
        None
    }
}



pub mod function
{
    use crate::builtin::utils::Types;

    use std::env;
    use std::path::PathBuf;

    pub fn cd(argv: &Vec<String>) 
    {
        if argv.len() > 2 {
            eprintln!("cd: too many arguments")
        }
    
        let target_path = PathBuf::from(&argv[1]);

        if let Err(e) = env::set_current_dir(target_path.clone()) {
            eprintln!("cd: {}", e);
        } 
    }
}

pub mod r_function
{
    pub fn _if(argv: &Vec<String>) -> bool
    {
        if argv.len() > 3 {
            eprintln!("if: too many arguments");
        }

        // if argv[2] ==

        false
    }

}

pub mod operator 
{

}

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

    pub fn is_builtin_function(input: &str) -> bool 
    {
        FUNCTIONS.contains(&input)
    }

    pub fn is_builtin_operator(input: &str) -> bool 
    {
        OPERATORS.contains(&input)
    }

    pub fn handle_builtin(input: &Vec<String>) -> Option<String>
    {
        None
    }
}



mod function
{
    use crate::builtin::utils::Types;
    use std::env;

    fn cd(argv: &Vec<String>) 
    {
        use std::path::PathBuf;

        if argv.len() > 2 {
            eprintln!("cd: too many arguments")
        }
    
        let target_path = PathBuf::from(&argv[1]);

        if let Err(e) = env::set_current_dir(target_path.clone()) {
            eprintln!("cd: {}", e);
        } 
    }

    //fn int(ar)
}

mod r_function
{
    fn _if(argv: &Vec<String>) -> bool
    {
        if argv.len() > 3 {
            eprintln!("if: too many arguments");
        }

        // if argv[2] ==

        false
    }

}

mod operator 
{
    fn plus    (argv: &Vec<f64>) -> f64 {argv[0] + argv[1]}
    fn minus   (argv: &Vec<f64>) -> f64 {argv[0] - argv[1]}
    fn multiply(argv: &Vec<f64>) -> f64 {argv[0] * argv[1]}
    fn divide  (argv: &Vec<f64>) -> f64 {argv[0] / argv[1]}
}

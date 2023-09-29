#[derive(Clone, PartialEq)]
enum TokenType {
    Command,    // for built-in or system commands
    Subshell,   // (), for command substitution
    Quote,      // "", '', for quoted arguments
    Block       // {}. for conditional/looped code exectution
}

#[derive(Clone)]
struct Token {
    body: String,
    kind: TokenType
}


fn print_tokens(input: &Vec<Token>)
{
    println!("Tokens:");
    for t in input {
        match t.kind {
            TokenType::Quote =>     println!("\ttoken: '{}' \ttype: Quote", t.body),
            TokenType::Command =>   println!("\ttoken: '{}' \ttype: Command", t.body),
            TokenType::Subshell =>  println!("\ttoken: '{}' \ttype: Subshell", t.body),
            TokenType::Block =>     println!("\ttoken: '{}' \ttype: Block", t.body),
            // _ =>                    println!("\ttoken: '{}' \ttype: Unknown", t.body),
        }
    }
}

fn tokenize(input: &str) -> Option<Vec<Token>>
{
    if input.is_empty() {
        return None;
    }

    println!("Input string: '{}'", input);

    let mut tokens: Vec<Token> = Vec::new();
    let mut token = String::new();

    let mut current_type: TokenType = TokenType::Command;

    let mut double_quote = false;
    let mut subshell_depth: u16 = 0;

    let open_delims: &str = "({";
    let close_delims: &str = ")}";

    macro_rules! push_token {
        () => {
            tokens.push(Token {body: token.clone(), kind: current_type.clone()});
            token.clear();
        };
    }

    for (i, c) in input.chars().enumerate() {
        match current_type {
            TokenType::Command => { 
                // allows for a delimiter at 
                // the end even if there isn't one

                // if c != ' ' && open_delims.contains(next_char) {
                //     println!("char {c}");
                //     token.push(c);
                //     tokens.push(Token {body: token.clone(), kind: current_type.clone()});
                //     token.clear();
                //     continue;
                // }
                
                if close_delims.contains(c) {
                    eprintln!("swish: unexpected '{}', no corresponding delimiter", c);
                    return None;
                }

                match c {
                    '"' => {
                        current_type = TokenType::Quote;
                        double_quote = true;
                    }
                    '\'' => {
                        current_type = TokenType::Quote;
                        double_quote = false;
                    }
                    '(' => {
                        if !token.is_empty() {
                            push_token!();
                        }
                        current_type = TokenType::Subshell;
                        subshell_depth += 1;
                    }
                    '{' => {
                        if !token.is_empty() {
                            push_token!();
                        }
                        current_type = TokenType::Block;
                        subshell_depth += 1;
                    }
                    ' ' => {
                        if input.chars().nth(i + 1).unwrap_or(' ') != ' ' {
                            push_token!();
                        }
                    }
                    _ => {
                        token.push(c);
                    }
                }
            }

            TokenType::Quote => {
                match c {
                    '"' if double_quote => {
                        push_token!();
                        current_type = TokenType::Command;
                    }
                    '\'' if !double_quote => {
                        push_token!();
                        current_type = TokenType::Command;
                    }
                    _ => token.push(c),
                }
            }

            TokenType::Subshell => {
                match c {
                    '(' => {
                        subshell_depth += 1;
                        token.push(c);
                    }
                    ')' => {
                        subshell_depth -= 1;
                        token.push(c);
                    }
                    _ => token.push(c),
                }
                if subshell_depth == 0 {
                    // remove trailing )
                    token.pop();

                    push_token!();
                    current_type = TokenType::Command;
                }
            }

            TokenType::Block => {
                match c {
                    '{' => subshell_depth += 1,
                    '}' => subshell_depth -= 1,
                    _ => token.push(c),
                }
                if subshell_depth == 0 {
                    push_token!();
                    current_type = TokenType::Command;
                }
            }
        }
    }

    if !token.is_empty() {
        push_token!();
    }

    // removes empty args not caused by ""
    tokens.retain(|t| !(t.body.is_empty() && t.kind != TokenType::Quote));

    Some(tokens)
}

// options for piping for stdout
// used for | or 
struct PipeOpts {
    pipe_stdout: bool,
    pipe_stderr: bool
}

fn execute_external(argv: Vec<String>, p_opts: Option<PipeOpts>) -> Result<(), std::io::Error>
{
    use std::process::{Command, Stdio};

    if let Some((first, args)) = argv.split_first() {

        let mut cmd = Command::new(first);
        cmd.args(args);

        match p_opts {
            Some(p) => {
                if p.pipe_stdout {
                    cmd.stdout(Stdio::piped());
                } else if p.pipe_stderr {
                    cmd.stderr(Stdio::piped());
                } else {
                    cmd.stdout(Stdio::inherit());
                    cmd.stderr(Stdio::inherit());
                }
            }
            None => {
                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());
            }
        };

        cmd.stdin(Stdio::inherit());

        match cmd.spawn() {
            Ok(mut child) => {
                child.wait()?;
                Ok(())
            }
            Err(e) => Err(e)
        }
    } else {
        use std::io::{Error, ErrorKind};
        Err(Error::new(ErrorKind::InvalidInput, "No arguments provided"))
    }
}

pub fn parse_input(input: &str) -> Option<String>
{
    let mut tokens = tokenize(input).unwrap_or(vec![]);

    // debug
    print_tokens(&tokens);
    
    // replace subshell token with the output of the contained commands
    for t in &mut tokens {
        if t.kind == TokenType::Subshell {
            print!("\nSubshell!\n");
            // re-parse subshell arguments
            match parse_input(t.body.as_str()) {
                Some(o) => {
                    t.body = o;
                    t.kind = TokenType::Quote;
                }
                None => return None
            }
        }
    }

    if tokens.is_empty() {return None}

    let tokens_strings: Vec<String> = tokens
        .iter()
        .map(|t| t.body.clone())
        .collect();

    use crate::builtin::utils;
    // detect opterators
    for t in &tokens {
        if t.kind == TokenType::Command {
            if utils::is_builtin_function(t.body.as_str()) {

            }
        }
    }

    // stdout of commands
    let e_out = execute_external(tokens_strings, None);
    return None;
    /* match e_out {
        Some(e) => return Some(e.body),
        None => return None,
    } */
}

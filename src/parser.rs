use crate::utils::{Token, TokenType};

fn tokenize(input: &str) -> Option<Vec<Token>> {
    if input.is_empty() {return None}

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
                if close_delims.contains(c) {
                    eprintln!("swish: unexpected '{}', no corresponding delimiter", c);
                    return None;
                }

                match c {
                    '"' => {
                        if !token.is_empty() { push_token!(); }
                        current_type = TokenType::Quote;
                        double_quote = true;
                    }
                    '\'' => {
                        if !token.is_empty() { push_token!(); }
                        current_type = TokenType::Quote;
                        double_quote = false;
                    }
                    '(' => {
                        if !token.is_empty() { push_token!(); }
                        current_type = TokenType::Subshell;
                        subshell_depth += 1;
                    }
                    '{' => {
                        if !token.is_empty() { push_token!(); }
                        current_type = TokenType::Block;
                        subshell_depth += 1;
                    }
                    ' ' => {
                        // ignore dupliate spaces
                        if input.chars().nth(i + 1).unwrap_or(' ') != ' ' {
                            push_token!();
                        }
                    }
                    '|' => {
                        // push pipe char as its own argument
                        push_token!();
                        tokens.push(Token {body: String::from("|"), kind: TokenType::Command});
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
struct ExecOpts {
    stdout: String,
    pipe_stdout: bool,
    pipe_stderr: bool
}

impl Default for ExecOpts {
    fn default () -> Self {
        ExecOpts { 
            stdout: String::new(),
            pipe_stdout: false,
            pipe_stderr: false 
        }
    }
}

enum ExecutionError {
    MissingArguments,
    SpawnError,
    Unknown,
}

impl From<std::io::Error> for ExecutionError {
    fn from(_: std::io::Error) -> Self { ExecutionError::Unknown }
}


fn execute_external(tokens: &Vec<Token>, opts: ExecOpts) -> Result<(), ExecutionError> {
    use std::process::{ Command, Stdio };

    let argv: Vec<String> = tokens
        .iter()
        .map(|t| t.body.clone())
        .collect();

    if let Some((first, args)) = argv.split_first() {

        let mut cmd = Command::new(first);
        cmd.args(args);


        if opts.pipe_stdout {
            cmd.stdout(Stdio::piped());
        } else if opts.pipe_stderr {
            cmd.stderr(Stdio::piped());
        } else {
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
        }

        cmd.stdin(Stdio::inherit());

        match cmd.spawn() {
            Ok(mut child) => {
                child.wait()?;
                Ok(())
            }
            Err(_) => Err(ExecutionError::SpawnError)
        }
    } else {
        Err(ExecutionError::MissingArguments)
    }
}

pub fn parse_input(input: &str) -> Option<String> {
    let mut tokens = tokenize(input).unwrap_or(vec![]);

    // DEBUG:
    println!("{:?}", tokens);
    
    // replace subshell token with the output of the contained commands
    for t in &mut tokens {
        if t.kind == TokenType::Subshell {
            print!("\nSubshell!\n");
            // re-parse subshell arguments
            match parse_input(t.body.as_str()) {
                Some(o) => {
                    t.body = o;
                    t.kind = TokenType::Quote; // could be Command instead
                }
                None => return None
            }
        }
    }

    if tokens.is_empty() {return None}

    let is_builtin = crate::builtin::handle_builtin(&tokens);

    return match is_builtin {
        Some(s) => Some(s),
        None => {
            execute_external(&tokens, ExecOpts::default());
            None
        }
    }

    // stdout of commands
    /* match e_out {
        Some(e) => return Some(e.body),
        None => return None,
    } */
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Command,    // for built-in or system commands
    Subshell,   // (), for command substitution
    Quote,      // "", '', for quoted arguments
    Block       // {}. for conditional/looped code exectution
}

#[derive(Debug, Clone)]
pub struct Token {
    pub body: String,
    pub kind: TokenType
}

// types for local variables
pub enum Type {
    Int,
    Float,
    String,
    Bool,
}

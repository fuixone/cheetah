use std::fmt;

#[derive(Debug)]
pub enum Errors {
    TokenError(&'static str),
    DeveloperError(&'static str),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Errors::TokenError(ref desc) => write!(f, "Syntax Error: {}", desc),
            Errors::DeveloperError(ref desc) => write!(f, "Developer Error: {}", desc),
        }
    }
}

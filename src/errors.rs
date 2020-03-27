use std::process::exit;
use std::fmt;

pub enum Error {
    CharacterError(char),
    TokenError(char),
    SyntaxError(&'static str, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Error::CharacterError(c) => format!("CharacterError: I found an unknown character '{}'.", c),
            Error::TokenError(c) => format!("TokenError: I found an unsuitable character '{}'.", c),
            Error::SyntaxError(expected, found) => format!("SyntaxError: I was expecting {}, but found {} before it.", expected, found),
        };
        write!(f, "{}", description)
    }
}

pub fn raise(e: Error) {
    panic!("{}", e);
    // eprintln!("{}", e);
    // exit(1);
}
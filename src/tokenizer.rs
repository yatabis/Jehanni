use std::fmt;
use crate::errors::{Error, raise};

fn is_small(c: char) -> bool {
    match c {
        'a' ..= 'z' => true,
        _ => false,
    }
}

fn is_large(c: char) -> bool {
    match c {
        'A' ..= 'Z' => true,
        _ => false,
    }
}

fn is_letter(c: char) -> bool {
    is_small(c) || is_large(c) || c == '_'
}

fn is_digit(c: char) -> bool {
    match c {
        '0' ..= '9' => true,
        _ => false,
    }
}

fn is_character(c: char) -> bool {
    is_letter(c) || is_digit(c) || c == ':' || c == '=' || c == '\n'
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Identifier(String),
    IntLiteral(String),
    Definion,
    Newline,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "'{}'", s),
            Token::IntLiteral(s) => write!(f, "'{}'", s),
            Token::Definion => write!(f, "':='"),
            Token::Newline => write!(f, "'\\n'"),
            Token::EOF => write!(f, "EOF"),

        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TokenList {
    pub tokens: Vec<Token>,
    source: Vec<char>,
    index: usize,
}

impl fmt::Display for TokenList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "[".to_string();
        for (i, t) in self.tokens.iter().enumerate() {
            if i != 0 {
                s += ", ";
            }
            s += &format!("{}", t);
        }
        s += "]";
        write!(f, "{}", s)
    }
}

impl TokenList {
    pub fn new(code: &str) -> Self {
        let mut token_list = TokenList{
            tokens: Vec::new(),
            source: code.chars().collect(),
            index: 0,
        };
        token_list.tokenize();
        token_list
    }

    fn current(&self) -> char {
        self.source[self.index]
    }

    fn tokenize(&mut self) {
        while self.index < self.source.len() {
            match self.current() {
                c if is_letter(c) => self.identifier(),
                c if is_digit(c) => self.int_literal(),
                c if c == ':' => self.definion(),
                c if c == '\n' => self.newline(),
                c if c == ' ' => self.index += 1,
                c if is_character(c) => raise(Error::TokenError(c)),
                c => raise(Error::CharacterError(c)),
            }
        }
    }

    fn identifier(&mut self) {
        let mut s = String::new();
        while self.index < self.source.len() {
            match self.current() {
                c if is_letter(c) || is_digit(c) => {
                    s.push(self.current());
                    self.index += 1;
                },
                c if c == '=' => raise(Error::TokenError(c)),
                _ => break,
            }
        }
        self.tokens.push(Token::Identifier(s));
    }

    fn int_literal(&mut self) {
        let mut s = String::new();
        while self.index < self.source.len() {
            match self.current() {
                c if is_digit(c) => {
                    s.push(self.current());
                    self.index += 1;
                },
                c if is_letter(c) || c == '=' => raise(Error::TokenError(c)),
                _ => break,
            }
        }
        self.tokens.push(Token::IntLiteral(s));
    }

    fn definion(&mut self) {
        if let Some(&c) = self.source.get(self.index + 1) {
            if c == '=' {
                self.tokens.push(Token::Definion);
                self.index += 2;
            } else {
                raise(Error::TokenError(c));
            }
        } else {
            raise(Error::TokenError(':'));
        }
    }

    fn newline(&mut self) {
        while self.index < self.source.len() && self.current() == '\n' {
            self.index += 1;
        }
        self.tokens.push(Token::Newline);
    }
}

// テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_identifier() {
        assert_eq!(vec![Token::Identifier("ident".to_string())], TokenList::new("ident").tokens);
    }

    #[test]
    fn identifier_containing_int_literal() {
        assert_eq!(vec![Token::Identifier("ident01".to_string())], TokenList::new("ident01").tokens);
        assert_eq!(vec![Token::Identifier("ident01test".to_string())], TokenList::new("ident01test").tokens);
    }

    #[test]
    fn sequential_identifier() {
        assert_eq!(
            vec![Token::Identifier("a".to_string()), Token::Identifier("b".to_string()), Token::Identifier("cd".to_string())],
            TokenList::new("a b cd").tokens
        );
    }

    #[test]
    fn simple_definion() {
        assert_eq!(vec![Token::Definion], TokenList::new(":=").tokens);
    }

    #[test]
    fn sequential_definion() {
        assert_eq!(vec![Token::Definion, Token::Definion, Token::Definion], TokenList::new(":= :=:=").tokens);
    }

    #[test]
    fn simple_int_literal() {
        assert_eq!(vec![Token::IntLiteral("3".to_string())], TokenList::new("3").tokens);
        assert_eq!(vec![Token::IntLiteral("05".to_string())], TokenList::new("05").tokens);
    }

    #[test]
    fn sequential_int_literal() {
        assert_eq!(
            vec![Token::IntLiteral("1".to_string()), Token::IntLiteral("2".to_string()), Token::IntLiteral("34".to_string())],
            TokenList::new("1 2 34").tokens
        );
    }

    #[test]
    fn int_literal_and_identifier() {
        assert_eq!(
            vec![Token::IntLiteral("7".to_string()), Token::Identifier("eleven".to_string())],
            TokenList::new("7 eleven").tokens
        );
    }

    #[test]
    fn simple_newline() {
        assert_eq!(vec![Token::Newline], TokenList::new("\n").tokens);
    }

    #[test]
    fn sequential_newline() {
        assert_eq!(vec![Token::Newline], TokenList::new("\n\n").tokens);
    }

    #[test]
    fn simple_definition() {
        assert_eq!(
            vec![Token::Identifier("test".to_string()), Token::Definion, Token::IntLiteral("3".to_string()), Token::Newline],
            TokenList::new("test := 3\n").tokens
        );
    }
    #[test]
    fn sequential_definition() {
        assert_eq!(
            vec![Token::Identifier("test01".to_string()), Token::Definion, Token::IntLiteral("3".to_string()), Token::Newline,
                Token::Identifier("test02".to_string()), Token::Definion, Token::IntLiteral("25".to_string()), Token::Newline],
            TokenList::new("test01 := 3\ntest02:=25\n").tokens
        );
    }

    #[test]
    #[should_panic(expected = "CharacterError: I found an unknown character '漢'.")]
    fn character_error() {
        TokenList::new("漢字");
    }

    #[test]
    #[should_panic(expected = "TokenError: I found an unsuitable character '='.")]
    fn token_beginning() {
        TokenList::new("=");
    }

    #[test]
    #[should_panic(expected = "TokenError: I found an unsuitable character '='.")]
    fn identifier_contains_equal() {
        TokenList::new("test=");
    }

    #[test]
    #[should_panic(expected = "TokenError: I found an unsuitable character '='.")]
    fn int_literal_contains_equal() {
        TokenList::new("20=");
    }

    #[test]
    #[should_panic(expected = "TokenError: I found an unsuitable character 'c'.")]
    fn int_literal_contains_letter() {
        TokenList::new("3cm");
    }

    #[test]
    #[should_panic(expected = "TokenError: I found an unsuitable character 'a'.")]
    fn broken_definion() {
        TokenList::new(":a");
    }

    #[test]
    #[should_panic(expected = "TokenError: I found an unsuitable character ':'.")]
    fn unfinished_definion() {
        TokenList::new(":");
    }

}

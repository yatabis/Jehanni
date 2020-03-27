use std::fmt;
use crate::tokenizer::{Token, TokenList};
use crate::errors::{Error, raise};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Node {
    Line(Box<Node>),
    VarDefinition(Box<Node>, Box<Node>),
    VarName(String),
    Value(String),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Line(node) => write!(f, "{}", node),
            Node::VarDefinition(n1, n2) => write!(f, "{}:={}", n1, n2),
            Node::VarName(s) => write!(f, "{}", s),
            Node::Value(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct AST {
    pub nodes: Vec<Node>,
    tokens: Vec<Token>,
    index: usize,
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "[".to_string();
        for (i, t) in self.nodes.iter().enumerate() {
            if i != 0 {
                s += ", ";
            }
            s += &format!("{}", t);
        }
        s += "]";
        write!(f, "{}", s)
    }
}

impl AST {
    pub fn new(token_list: TokenList) -> Self {
        let mut ast = AST{
            nodes: Vec::new(),
            tokens: token_list.tokens,
            index: 0,
        };
        ast.parse();
        ast
    }

    fn current(&self) -> &Token {
        if let Some(token) = self.tokens.get(self.index) {
            token
        } else {
            &Token::EOF
        }
    }

    fn parse(&mut self) {
        while self.index < self.tokens.len() {
            match self.line() {
                Ok(node) => self.nodes.push(node),
                Err(e) => raise(e),
            }
        }
    }

    fn line(&mut self) -> Result<Node, Error> {
        let node;
        match self.var_definition() {
            Ok(n) => {
                node = Node::Line(Box::new(n));
            },
            Err(e) => {
                return Err(e);
            },
        }
        match self.current() {
            Token::Newline => {
                self.index += 1;
                Ok(node)
            },
            token => {
                Err(Error::SyntaxError("a newline at the end of a line", format!("{}", token)))
            }
        }
    }

    fn var_definition(&mut self) -> Result<Node, Error> {
        let name;
        match self.var_name() {
            Ok(n) => {
                name = n;
            },
            Err(e) => {
                return Err(e);
            },
        }

        let token = self.current();
        if let Token::Definion = token {
            self.index += 1;
        } else {
            return Err(Error::SyntaxError("':=' for a variable definition", format!("{}", token)))
        }

        let val;
        match self.value() {
            Ok(n) => {
                val = n;
            },
            Err(e) => {
                return Err(e);
            },
        }

        Ok(Node::VarDefinition(Box::new(name), Box::new(val)))
    }

    fn var_name(&mut self) -> Result<Node, Error> {
        let token = self.current();
        if let Token::Identifier(s) = token {
            let name = s.to_string();
            self.index += 1;
            Ok(Node::VarName(name))
        } else {
            Err(Error::SyntaxError("the variable name at the beginning of a line", format!("{}", token)))
        }
    }

    fn value(&mut self) -> Result<Node, Error> {
        let token = self.current();
        if let Token::IntLiteral(s) = token {
            let val = s.to_string();
            self.index += 1;
            Ok(Node::Value(val))
        } else {
            Err(Error::SyntaxError("the value of the variable", format!("{}", token)))
        }
    }
}

// テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_line() {
        assert_eq!(vec![Node::Line(
            Box::new(Node::VarDefinition(
                Box::new(Node::VarName("test".to_string())),
                Box::new(Node::Value("10".to_string())),
            ))
        )],
        AST::new(TokenList::new("test := 10\n")).nodes);
    }

    #[test]
    #[should_panic(expected = "SyntaxError: I was expecting a newline at the end of a line, but found EOF before it.")]
    fn line_ends_eof() {
        AST::new(TokenList::new("test := 10"));
    }

    #[test]
    #[should_panic(expected = "SyntaxError: I was expecting a newline at the end of a line, but found 'foo' before it.")]
    fn line_ends_foo() {
        AST::new(TokenList::new("test := 10 foo"));
    }

    #[test]
    #[should_panic(expected = "SyntaxError: I was expecting the variable name at the beginning of a line, but found ':=' before it.")]
    fn line_starts_definion() {
        AST::new(TokenList::new(":="));
    }

    #[test]
    #[should_panic(expected = "SyntaxError: I was expecting ':=' for a variable definition, but found EOF before it.")]
    fn no_definion() {
        AST::new(TokenList::new("test"));
    }

    #[test]
    #[should_panic(expected = "SyntaxError: I was expecting the value of the variable, but found ':=' before it.")]
    fn sequential_definion() {
        AST::new(TokenList::new("x:=:="));
    }

}

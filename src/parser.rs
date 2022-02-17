use crate::token::Token;
use crate::lexer::Lexer;
use crate::interpreter::Interpreter;

use std::mem::discriminant;

/// A node in the Abstract Syntax Tree (AST)
pub trait Node {
    fn accept(&self, visitor: &Interpreter) -> Result<i64, String>;
}

/// Integer node
pub struct IntegerNode {
    pub token: Token,
}

impl From<i64> for IntegerNode {
    fn from(num: i64) -> Self {
        IntegerNode {
            token: Token::Integer(num)
        }
    }
}

impl Node for IntegerNode {
    fn accept(&self, visitor: &Interpreter) -> Result<i64, String> {
        visitor.visit_int(self)
    }
}

pub struct UnaryOpNode {
    pub token: Token,
    pub rhs: Box<dyn Node>,
}

impl Node for UnaryOpNode {
    fn accept(&self, visitor: &Interpreter) -> Result<i64, String> {
        visitor.visit_unop(self)
    }
}

pub struct BinaryOpNode {
    pub token: Token,
    pub lhs: Box<dyn Node>,
    pub rhs: Box<dyn Node>,
}

impl Node for BinaryOpNode {
    fn accept(&self, visitor: &Interpreter) -> Result<i64, String> {
        visitor.visit_binop(self)
    }
}

macro_rules! box_node {
    ($lhs:tt * $rhs:expr) => {
        Box::new(BinaryOpNode { token: Token::Mul, lhs: $lhs, rhs: $rhs })
    };
    ($lhs:tt / $rhs:expr) => {
        Box::new(BinaryOpNode { token: Token::Div, lhs: $lhs, rhs: $rhs })
    };
    ($lhs:tt + $rhs:expr) => {
        Box::new(BinaryOpNode { token: Token::Add, lhs: $lhs, rhs: $rhs })
    };
    ($lhs:tt - $rhs:expr) => {
        Box::new(BinaryOpNode { token: Token::Sub, lhs: $lhs, rhs: $rhs })
    };
    (+ $rhs:expr) => {
        Box::new(UnaryOpNode { token: Token::Add, rhs: $rhs })
    };
    (- $rhs:expr) => {
        Box::new(UnaryOpNode { token: Token::Sub, rhs: $rhs })
    };
    ($num:tt) => {
        Box::new(IntegerNode::from($num))
    }
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token
        }
    }

    fn eat(&mut self, token: &Token) -> Result<(), String> {
        if discriminant(&self.current_token) == discriminant(token) {
            self.current_token = self.lexer.next_token();
            Ok(())
        } else {
            Err("Invalid syntax".to_string())
        }
    }

    /// factor : (PLUS | MINUS) factor | INTEGER | LPAREN expr RPAREN
    fn factor(&mut self) -> Result<Box<dyn Node>, String> {
        match self.current_token {
            Token::Add => {
                self.eat(&Token::Add)?;
                let node = box_node!(+ self.factor()?);
                Ok(node)
            },
            Token::Sub => {
                self.eat(&Token::Sub)?;
                let node = box_node!(- self.factor()?);
                Ok(node)
            },
            Token::Integer(num) => {
                self.eat(&Token::Integer(0))?;
                Ok(box_node!(num))
            },
            Token::LParen => {
                self.eat(&Token::LParen)?;
                let result = self.expr();
                self.eat(&Token::RParen)?;
                result
            },
            Token::Illegal => Err("Illegal input".to_string()),
            _ => Err("Invalid syntax - expecting an integer \
                     or a left parenthesis".to_string()),
        }
    }

    /// term : factor ((MUL | DIV) factor)*
    fn term(&mut self) -> Result<Box<dyn Node>, String> {
        let mut node = self.factor()?;

        loop {
            match self.current_token {
                Token::Mul => {
                    self.eat(&Token::Mul)?;
                    node = box_node!(node * self.factor()?);
                },
                Token::Div => {
                    self.eat(&Token::Div)?;
                    node = box_node!(node / self.factor()?);
                },
                _ => break,
            }
        }

        Ok(node)
    }

    /// expr : term ((PLUS | MINUS) term)*
    pub fn expr(&mut self) -> Result<Box<dyn Node>, String> {
        let mut node = self.term()?;

        loop {
            match self.current_token {
                Token::Add => {
                    self.eat(&Token::Add)?;
                    node = box_node!(node + self.term()?);
                },
                Token::Sub => {
                    self.eat(&Token::Sub)?;
                    node = box_node!(node - self.term()?);
                },
                _ => break,
            }
        }

        Ok(node)
    }
}
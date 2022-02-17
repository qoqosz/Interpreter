use crate::token::Token;
use crate::parser::{Parser, Node, IntegerNode, UnaryOpNode, BinaryOpNode};

pub struct Interpreter<'a> {
    parser: &'a mut Parser<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Interpreter<'a> {
        Interpreter { parser }
    }

    pub fn interpret(&mut self) -> Result<i64, String> {
        let tree = self.parser.expr()?;
        self.visit(&tree)
    }

    pub fn visit(&self, tree: &Box<dyn Node>) -> Result<i64, String> {
        tree.accept(self)
    }

    pub fn visit_int(&self, node: &IntegerNode) -> Result<i64, String> {
        match node.token {
            Token::Integer(val) => Ok(val),
            _ => Err("Not an integer".to_string()),
        }
    }

    pub fn visit_unop(&self, node: &UnaryOpNode) -> Result<i64, String> {
        let rhs = self.visit(&node.rhs)?;

        match node.token {
            Token::Add => Ok(rhs),
            Token::Sub => Ok(-rhs),
            _ => Err("Not an unary operator".to_string()),
        }
    }

    pub fn visit_binop(&self, node: &BinaryOpNode) -> Result<i64, String> {
        let lhs = self.visit(&node.lhs)?;
        let rhs = self.visit(&node.rhs)?;

        match node.token {
            Token::Add => Ok(lhs + rhs),
            Token::Sub => Ok(lhs - rhs),
            Token::Mul => Ok(lhs * rhs),
            Token::Div => Ok(lhs / rhs),
            _ => Err("Not a binary operator".to_string())
        }
    }
}
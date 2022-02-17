use crate::token::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}


impl<'a> Lexer<'a> {
    pub fn new(line: &'a String) -> Self {
        Lexer {
            input: line.chars().peekable()
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.next() {
            Some(token) => token,
            None => Token::EndOfFile,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&ch) = self.peek_char() {
            if !ch.is_numeric() {
                break;
            }
            number.push(self.read_char().unwrap());
        }

        number
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.read_char() {
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Sub),
            Some('*') => Some(Token::Mul),
            Some('/') => Some(Token::Div),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),

            // `ch @ _` saves `_` in `ch` variable
            Some(ch @ _) => {
                if ch.is_numeric() {
                    Some(Token::Integer(self.read_number(ch)
                                            .parse::<i64>()
                                            .unwrap()))
                } else {
                    println!("ch <{}>", ch);
                    Some(Token::Illegal)
                }
            },

            None => None,
        }
    }
}
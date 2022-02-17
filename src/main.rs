/// Based on:
///     https://github.com/chr4/writing_an_interpreter_in_rust
pub mod interpreter;
pub mod lexer;
pub mod token;
pub mod parser;

use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, BufRead, Write};

/// debuging
pub fn print_tokens(line: &String) {
    let mut line = line.clone();
    let lexer = Lexer::new(&mut line);

    println!("[Debug] tokens read:");

    for token in lexer {
        println!("{:?}", token);
    }
}

fn main() {
    println!("Type 'q' to exit");
    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdin");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        if line.starts_with("q") {
            break;
        }

        print_tokens(&line);

        let mut lexer = Lexer::new(&mut line);
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);

        match interpreter.interpret() {
            Ok(result) => println!("={}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
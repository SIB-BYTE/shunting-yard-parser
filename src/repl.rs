// read-evaluate-print-loop
use crate::{parser, lexer};

use std::io;
use std::io::Write;

pub fn repl() {
    loop {
        print!("Enter an expression: ");
        io::stdout().flush().unwrap();

        let mut expr: String = String::new();
        io::stdin()
            .read_line(&mut expr)
            .expect("Couldn't read expression!");

        let tokens = lexer::tokenize(expr.as_ref());
        let rpn_tokens = parser::Parser::shunting_yard(tokens.as_ref().unwrap());
        let result = parser::evaluate(rpn_tokens.as_ref().unwrap());

        println!("Lexer output => {:?}\n\
                Parser output => {:?}\n\
                Result => {}", tokens, rpn_tokens, result);
    }
}
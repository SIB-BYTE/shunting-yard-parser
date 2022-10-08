#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::iter::Peekable;

const LA: u8 = 1;
const RA: u8 = 2;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Token {
    Num(i64),
    // operator : precedence, associativity
    Uop(char, u8, u8),
    Bop(char, u8, u8),
    Paren_pair(char),
}

/*

*/

pub fn tokenize(src: &str) -> Result<Vec<Token>, String> {
    let mut token_stream = vec![];
    let mut iterator = src.chars().peekable();

    while let Some(&token) = iterator.peek() {
        iterator.next();
        match token {
            ' ' | '\n' | '\t' => { /* do nothing */ },
            '(' | ')' => { token_stream.push(Token::Paren_pair(token)); },
            '~' => { token_stream.push(Token::Uop(token, 3, RA)); },
            '^' => { token_stream.push(Token::Bop(token, 1, RA)); },
            '+' | '-' => { token_stream.push(Token::Bop(token, 1, LA)); },
            '*' | '/' | '%' => { token_stream.push(Token::Bop(token, 2, LA)); },
            '0'..='9' => { token_stream.push(Token::Num(scan_number(token, &mut iterator))); },
            _ => { return Err(format!("Invalid token: {}", token)); }
        }
    }

    Ok(token_stream)
}

pub fn scan_number<T>(initial_digit: char, iterator: &mut Peekable<T>) -> i64
    where T: Iterator<Item = char>{
    let mut number: i64 = initial_digit.to_digit(10).unwrap() as i64;

    while let Some(digit) = iterator.next_if(|&next_digit| next_digit.is_numeric()) {
        let digit: i64 = digit.to_digit(10).unwrap() as i64;
        number = number * 10 + digit;
    }

    number
}

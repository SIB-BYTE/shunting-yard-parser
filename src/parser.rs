#![allow(dead_code, non_camel_case_types)]

use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub struct Parser {
    prev_was_op: bool,
    output_queue: Vec<Token>,
    operator_stack: Vec<Token>
}

impl Parser {
    pub fn new() -> Self {
        Self {
            prev_was_op: true,
            output_queue: vec![],
            operator_stack: vec![],
        }
    }

    pub fn scan_pair(operator_stack: &mut Vec<Token>, output_queue: &mut Vec<Token>, pattern: Token) -> bool {
        while let Some(token) = operator_stack.pop() {
            if token == pattern {
                return true;
            }

            output_queue.push(token);
        }

        false
    }

    pub fn shunting_yard(tokens: &Vec<Token>) -> Result<Vec<Token>, String> {
        let mut parser = Self::new();

        for token in tokens {
            match *token {
                Token::Num(_) => parser.output_queue.push(*token),
                Token::Paren_pair('(') => parser.operator_stack.push(*token),

                Token::Bop(_, prec, assoc) |
                Token::Uop(_, prec, assoc) => {
                    while let Some(&top) = parser.operator_stack.last() {
                        match top {
                            Token::Paren_pair('(') => break,
                            Token::Bop(_, top_prec, _) |
                            Token::Uop(_, top_prec, _) => {
                                if prec < top_prec || prec == top_prec && assoc == 0 {
                                    parser.output_queue.push(parser.operator_stack.pop().unwrap());
                                }
                                else { break; }
                            }
                            _ => unreachable!("Yeah this shouldn't happen"),
                        }
                    }
                    parser.operator_stack.push(*token);
                }

                Token::Paren_pair(')') => {
                    if !Self::scan_pair(&mut parser.operator_stack, &mut parser.output_queue, Token::Paren_pair('(')) {
                        return Err(format!("Unmatched: ')'"));
                    }
                }

                _ => panic!("Invalid operator"),
            }
        }

        if Self::scan_pair(&mut parser.operator_stack, &mut parser.output_queue, Token::Paren_pair('(')) {
            return Err(format!("Unmatched: ')'"));
        }

        Ok(parser.output_queue)
    }
}

pub fn evaluate(rpn_tokens: &Vec<Token>) -> f64 {
    let mut eval_stack = vec![];

    for token in rpn_tokens {
        match token {
            Token::Num(num) => eval_stack.push(*num as f64),
            Token::Bop(op, _, _) => {
                let first:  f64 = eval_stack.pop().unwrap() as f64;
                let second: f64 = eval_stack.pop().unwrap() as f64;

                match op {
                    '+' => eval_stack.push(second + first),
                    '-' => eval_stack.push(second - first),
                    '*' => eval_stack.push(second * first),
                    '/' => eval_stack.push(second / first),
                    '^' => eval_stack.push(second.powf(first)),
                    _ => panic!("Invalid binary operation!"),
                };
            }

            Token::Uop(op, _, _) => {
                let first: f64 = eval_stack.pop().unwrap() as f64;

                match op {
                    '~' => eval_stack.push(-first),
                    _ => panic!("Invalid unary operation!"),
                }
            }
            _ => continue,
        }
    }
    eval_stack.pop().unwrap()
}

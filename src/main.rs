#![feature(let_chains)]

mod number;
mod operator;
mod expression;
mod parser;

extern crate core;

use std::cmp::PartialEq;
use std::io;
use std::iter::Peekable;
use std::ops::{Add, Div, Mul, Sub};
use crate::expression::Expression;
use crate::number::Number;
use crate::number::Number::{Float, Int};
use crate::operator::Operator;
use crate::operator::Operator::*;
use crate::parser::Parser;

#[derive(Debug, Clone,Copy)]
enum Token{
    LeftBracket,
    RightBracket,
    Number(Number),
    Operator(Operator),
}
/// Converts your mathematical expression into tokens
fn lex(input: &str) -> Vec<Token>
{
    let mut tokens = Vec::new();


    let mut i = 0;
    while i < input.len() {

        if let Some(mut character) = input.chars().nth(i){

            match character {
                // ignores white space
                ' ' => {}
                // converting numbers to tokens
                '0'..='9' => {
                    let mut my_str = String::from(character);

                    let mut is_float = false;
                    while let Some(next_char)
                        = input.chars().nth(i + 1)
                        && (('0'..='9').contains(&next_char)
                        || (next_char == '.' && !is_float)) {
                        if next_char == '.'{
                            is_float = true;
                        }
                        character = next_char;
                        my_str.push(character);
                        i += 1;

                    }
                    if !is_float{
                        let as_num : i32 = my_str.parse().expect(format!("Unable to parse {my_str} as number").as_str());
                        tokens.push(Token::Number(Int(as_num)));
                    } else {
                        let as_num : f32 = my_str.parse().expect(format!("Unable to parse {my_str} as number").as_str());
                        tokens.push(Token::Number(Float(as_num)));
                    }

                }
                '+' => {
                    tokens.push(Token::Operator(Plus));
                },
                '-' => {
                    tokens.push(Token::Operator(Minus));
                },
                '*' => {
                    tokens.push(Token::Operator(Multiply));
                },
                '/' => {
                    tokens.push(Token::Operator(Divide));
                },
                '(' => {
                    tokens.push(Token::LeftBracket);
                },
                ')' => {
                    tokens.push(Token::RightBracket);
                },
                _ => {
                    panic!("Unrecognized character: {}", character);
                }

            }
            i += 1
        }
    }

    return tokens;
}




fn main() {
    println!("Input your mathematical expression:");
    let  input_accepter  = io::stdin().lines().next().unwrap().unwrap();
    let tokens = lex(input_accepter.as_str());
    let expression = Parser::new(tokens.into_iter()).compile();
    let result = expression.eval();
    println!("Result: {result}",)
}

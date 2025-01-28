#![feature(let_chains)]

extern crate core;
mod number;
mod operator;
mod expression;
mod parser;

use crate::number::Number;
use crate::number::Number::{Float, Int};
use crate::operator::Operator;
use crate::operator::Operator::*;
use crate::parser::{Parser, ProgramData, SyntaxTree};
use std::cmp::PartialEq;
use std::{io, thread};
use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;
use crate::expression::SyntaxNode;

#[derive(Debug, Clone)]
enum Token{
    LeftBracket,
    RightBracket,
    Number(Number),
    Operator(Operator),
    VariableName(String),
    Spawn,
    SemiColon,
    Yap,
    Equals
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

                },
                ';' => {
                    tokens.push(Token::SemiColon);
                }
                '=' => {
                    tokens.push(Token::Equals);
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut my_str = String::from(character);

                    while let Some(next_char)
                        = input.chars().nth(i + 1)
                        && (('a'..='z').contains(&next_char) || ('A'..='Z').contains(&next_char) ) {
                        character = next_char;
                        my_str.push(character);
                        i += 1;

                    }
                    match my_str.as_str() {
                        "spawn" => {
                            tokens.push(Token::Spawn);
                        },
                        "yap" => tokens.push(Token::Yap),
                        other => tokens.push(Token::VariableName(my_str))
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
    println!("Input your code:");
    let mut string = std::io::stdin().lines().next().unwrap().unwrap();
    let tokens = lex(string.as_str());
    let mut data = ProgramData::default();
    let syntax_tree: SyntaxTree = Parser::new(tokens.into_iter()).compile();
    syntax_tree.eval(&mut data);
    thread::sleep(Duration::from_secs(5));

}

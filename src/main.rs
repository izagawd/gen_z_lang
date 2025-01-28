#![feature(let_chains)]

extern crate core;
mod number;
mod operator;
mod expression;
mod parser;
mod Data;

use crate::number::Number;
use crate::number::Number::{Float, Int};
use crate::operator::Operator;
use crate::operator::Operator::*;
use crate::parser::{Parser, ProgramData, SyntaxTree};
use std::cmp::PartialEq;
use std::{io, thread};
use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;
use crate::Equals::Is;
use crate::expression::SyntaxNode;
use crate::Token::Cap;

#[derive(Debug, Clone)]
enum Equals{
    Is, EqualsSign
}
#[derive(Debug, Clone)]
enum Token{
    LeftBracket,
    RightBracket,
    Number(Number),
    Operator(Operator),
    VariableName(String),
    //meaning ! as in !true
    No,
    /// Means let. used to assign data to variables. EG bag a = 5;
    Bag,
    SemiColon,
    /// means true
    Yap,
    /// means false
    Cap,
    Fax,
    Assign,
    And,
    Or,
    Equals(Equals)
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
                    let mut look_for_characters = |i: &mut usize|{
                        while let Some(next_char)
                            = input.chars().nth(*i + 1)
                            && (('0'..='9').contains(&next_char)) {
                            character = next_char;
                            my_str.push(character);
                            *i += 1;
                        }
                    };
                    look_for_characters(&mut i);
                    if let Some('.')  = input.chars().nth(i + 1) {
                        i += 1;
                        is_float = true;
                    }
                    look_for_characters(&mut i);
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
                    if let Some('=') = input.chars().nth(i + 1){
                        tokens.push(Token::Equals(Equals::EqualsSign));
                        i += 1;
                    } else{
                        tokens.push(Token::Assign);
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut my_str = String::from(character);
                    let  possible_other_characters =('a'..='z').chain('A'..='Z').chain('0'..'9').collect::<Vec<_>>();
                    while let Some(next_char)
                        = input.chars().nth(i + 1)
                        && possible_other_characters.contains(&next_char) {
                        character = next_char;
                        my_str.push(character);
                        i += 1;

                    }
                    match my_str.as_str() {
                        "bag" => {
                            tokens.push(Token::Bag);
                        },
                        "yap" => tokens.push(Token::Yap),
                        "is" => tokens.push(Token::Equals(Is)),
                        "no" => tokens.push(Token::No),
                        "cap" => tokens.push(Token::Cap),
                        "fax" => tokens.push(Token::Fax),
                        "and" => tokens.push(Token::And),
                        "or" => tokens.push(Token::Or),
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
    let mut string = "bag a = no cap bruh";
    let tokens = lex(string);
    println!("{:?}",tokens);



}

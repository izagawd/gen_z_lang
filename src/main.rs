#![feature(let_chains)]

extern crate core;
mod number;
mod operator;
mod expression;
mod parser;
mod Data;
mod lexer;
mod tokens;
mod syntax_tree;

use crate::number::Number;
use crate::number::Number::{Float, Int};
use crate::operator::Operator;
use crate::operator::Operator::*;

use std::cmp::PartialEq;
use std::{io, thread};

use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;
use crate::lexer::lex;
use crate::operator::Equals::{EqualsSign, Is};
use crate::parser::{Parser, ProgramData};


/// Converts your mathematical expression into tokens




fn main() {

    println!("Input your code:");
    let mut string = "if 5 == 4 yap('yay'); else if 5 == 4 yap('aww'); else yap('BITCH');";
    let tokens = lex(string);
    println!("{tokens:?}");
    let mut parser = Parser::new(tokens.into_iter());
    let compiled =parser.compile();
    compiled.eval(&mut ProgramData::default())


}

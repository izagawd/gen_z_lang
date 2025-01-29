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
use std::{fs, io, thread};
use std::io::Read;
use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;
use crate::lexer::lex;
use crate::operator::Equals::{EqualsSign, Is};
use crate::parser::{Parser, ProgramData};


/// Converts your mathematical expression into tokens




fn main() {

    const CODE_FILE_NAME : &str = "rizz.gzl";
    loop {


        print!("\x1B[2J\x1B[1;1H");

        if let Ok(mut file) = fs::File::open(CODE_FILE_NAME) {
            let mut code = String::new();
            if let Ok(_) = file.read_to_string(&mut code) {
                let panic = std::panic::catch_unwind(|| {
                    println!("NOTE: ONCE UR DONE CODING IN THE FILE, PRESS ENTER HERE TO RUN THE CODE");
                    let tokens = lex(code.as_str());
                    let mut parser = Parser::new(tokens.into_iter());
                    let compiled = parser.compile();
                    compiled.eval(&mut ProgramData::default())
                });
            } else {
                println!("Couldn't read {CODE_FILE_NAME}");
            }

        } else {
            println!("Couldn't open {CODE_FILE_NAME}. Ensure you have a file called {CODE_FILE_NAME} in the same directory as the programs file");
        }
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }


}

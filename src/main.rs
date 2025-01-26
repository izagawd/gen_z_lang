#![feature(let_chains)]
extern crate core;

use crate::Expression::Int;
use crate::Operator::{Divide, Minus, Multiply, Plus};
use std::cmp::PartialEq;
use std::iter::Peekable;


#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator{
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operator{
    #[inline]
    const fn evaluate(first: i32, operator: Operator, second: i32)->i32{
        return match operator {
            Operator::Plus => {
                first + second
            }
            Operator::Minus => {
                first - second
            }
            Operator::Multiply => {
                first * second
            }
            Operator::Divide => {
                first / second
            }
        }
    }
}

enum Expression {
    Bracketed(Box<Self>),
    Int(i32),

    BinaryExpression{
        left: Box<Self>,
        operator: Operator,
        right: Box<Self>,
    },
}
impl Expression {

     fn eval(self) -> i32{
        match self {
            Expression::Bracketed(input) => {
                input.eval()
            }
            Expression::Int(i32) => {
                return i32
            }

            Expression::BinaryExpression { left,operator,right } => {
                Operator::evaluate(left.eval(), operator, right.eval())
            }
        }
    }
}
#[derive(Debug, Clone,Copy)]
enum Token{
    LeftBracket,
    RightBracket,
    Int(i32),
    Operator(Operator),
}
fn lex(input: &str) -> Vec<Token>
{
    let mut tokens = Vec::new();


    let mut i = 0;
    while i < input.len() {

        if let Some(mut character) = input.chars().nth(i){

            match character {
                ' ' => {}
                '0'..='9' => {
                    let mut my_str = String::from(character);


                    while let Some(next_char)
                        = input.chars().nth(i + 1)
                        && ('0'..='9').contains(&next_char) {
                        character = next_char;
                        my_str.push(character);
                        i += 1;

                    }
                    let as_num : i32 = my_str.parse().expect(format!("Unable to parse {my_str} as number").as_str());
                    tokens.push(Token::Int(as_num));
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

struct Parser<Iter : Iterator<Item=Token>>{
    peekable: Peekable<Iter>
}



impl<Iter : Iterator<Item=Token>> Parser<Iter>{
    fn new(tokens: Iter) -> Parser<Iter>{
        Parser{
            peekable: tokens.peekable()
        }
    }
    fn parse_mul(&mut self) -> Expression{
        let mut left = self.parse_div();
        while let Some(Token::Operator(Multiply)) = self.peekable.peek().cloned() {
            self.peekable.next();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                operator: Operator::Multiply,
                right: Box::new(self.parse_div())
            };
        };
        return left;
    }
    fn parse_div(&mut self) -> Expression{
        let mut left = self.parse_primary();
        while let Some(Token::Operator(Divide)) = self.peekable.peek().cloned() {
            self.peekable.next();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                operator: Operator::Divide,
                right: Box::new(self.parse_primary())
            };
        };
        return left;
    }
    fn parse_primary(&mut self) -> Expression{

        match self.peekable.next() {
            Some(Token::Int(number)) => {
                Expression::Int(number)
            },
            Some(Token::LeftBracket) => {
                let expr = self.parse_expr();
                if let Some(Token::RightBracket) = self.peekable.next() {
                    Expression::Bracketed(Box::new(expr))
                } else {
                    panic!("Expected right bracket");
                }
            }
            other => panic!("Expected int or bracket, got {:?}", other)

        }
    }
    fn parse_add_sub(&mut self) -> Expression{
        let mut left = self.parse_mul();
        while let Some(Token::Operator(operator @ (Operator::Plus | Operator::Minus))) = self.peekable.peek().cloned() {
            self.peekable.next();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(self.parse_mul())
            };
        };
        return left;
    }
    fn parse_expr(&mut self) -> Expression{
        self.parse_add_sub()
    }
    fn compile(&mut self) -> Expression{
         self.start()

    }
    fn start(&mut self) -> Expression{
        self.parse_expr()
    }
}

fn main() {

    let to_lex = "5+23/455*(73*45)";
    println!("{:?}",lex(to_lex));
    println!("{}",Parser::new(
        lex(to_lex).into_iter()
    ).compile().eval())
}

use std::iter::Peekable;
use crate::expression::Expression;
use crate::operator::Operator;
use crate::operator::Operator::{Divide, Multiply};
use crate::Token;

/// Used to convert a collection of tokens into an Expression enum, which can also be seen as a syntax tree
pub struct Parser<Iter : Iterator<Item=Token>>{
    peekable: Peekable<Iter>
}



impl<Iter : Iterator<Item=Token>> Parser<Iter>{
    pub fn new(tokens: Iter) -> Parser<Iter>{
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
            Some(Token::Number(number)) => {
                Expression::Number(number)
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
    pub fn compile(&mut self) -> Expression{
        self.start()

    }
    fn start(&mut self) -> Expression{
        self.parse_expr()
    }
}
use std::collections::HashMap;
use std::iter::Peekable;
use crate::expression::{Expression, SyntaxNode};
use crate::number::Number;
use crate::operator::Operator;
use crate::operator::Operator::{Divide, Multiply};
use crate::Token;

/// Used to convert a collection of tokens into an Expression enum, which can also be seen as a syntax tree
pub struct Parser<Iter : Iterator<Item=Token>>{
    peekable: Peekable<Iter>
}

#[derive(Default)]
pub struct ProgramData{
    pub variables: HashMap<String, Number>,
}

pub struct SyntaxTree{
    nodes: Vec<SyntaxNode>,
}
impl SyntaxTree {
    pub fn eval(mut self,program_data: &mut ProgramData){
        for i in self.nodes.into_iter(){
            i.eval(program_data);
        }
    }
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
    fn parse_finish_line(&mut self, syntax_node: SyntaxNode) -> SyntaxNode{
        if let Some(Token::SemiColon) = self.peekable.next(){
            return syntax_node;
        } else{
            panic!("Lines are supposed to end with semi colons")
        }
    }


    fn parse_let_and_yap(&mut self) -> Vec<SyntaxNode>{
        let mut syntax_nodes = Vec::new();
        while let Some(spawn_or_print @ (Token::Spawn | Token::Yap)) = self.peekable.peek().cloned() {
            match spawn_or_print {
                Token::Yap => {
                    while let Some(Token::Yap) = self.peekable.peek().cloned() {
                        self.peekable.next();
                        if let Some(Token::LeftBracket) = self.peekable.next()  {
                            let created = SyntaxNode::Yap(self.parse_expr());
                            if let Some(Token::RightBracket) = self.peekable.next()  {
                                syntax_nodes.push(
                                    self.parse_finish_line(created)
                                );
                            } else{
                                panic!("Right bracket missing on yap");
                            }
                        } else{
                            panic!("Expected left bracket");
                        }
                    };
                },
                Token::Spawn => {
                    self.peekable.next();
                    if let Some(Token::VariableName(name)) = self.peekable.next()
                        && let Some(Token::Equals) = self.peekable.next()   {
                        let created = SyntaxNode::Declaration { equals_to: self.parse_expr(),name };
                        syntax_nodes.push(
                            self.parse_finish_line(created)
                        );
                    } else{
                        panic!("Expected variable-name =");
                    }
                }
                _ => {}
            }
        };
        return syntax_nodes;
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
            },
            Some(Token::VariableName(name)) => {
                Expression::Variable(name)
            }
            other => panic!("Expected number or left bracket, got {:?}", other)

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
    pub fn compile(&mut self) -> SyntaxTree{
        SyntaxTree{nodes: self.start()}

    }
    fn start(&mut self) -> Vec<SyntaxNode>{
        self.parse_let_and_yap()
    }
}
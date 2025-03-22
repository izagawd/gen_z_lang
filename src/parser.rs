use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::iter::Peekable;
use crate::Data::Data;
use crate::Data::Data::{Boolean, Number};
use crate::expression::{Expression};


use crate::operator::Operator;
use crate::operator::Operator::{Divide, Multiply, No};
use crate::syntax_tree::{SyntaxNode, SyntaxNodeVariant};
use crate::tokens::Token;

/// Used to convert a collection of tokens into an Expression enum, which can also be seen as a syntax tree
pub struct Parser<Iter : Iterator<Item=Token>>{
    peekable: Peekable<Iter>,
    depth: i32,
}



pub struct DepthData{
    variables: HashMap<String, Data>,
}


#[derive(Default)]
pub struct ProgramData{
    depth_datas: HashMap<i32, DepthData>,
}
impl ProgramData{

    pub fn get_variable_with_depth(&self,input: &str,mut depth: i32) -> Option<(Data,i32)>{

        while depth >= 0{
            if let Some(depth_data) = self.depth_datas.get(&depth)
                && let Some(gotten_data) = depth_data.variables.get(&input.to_string()){
                return Some((gotten_data.clone(), depth));
            }
            depth -= 1;
        }
        return None;

    }
    pub fn get_variable(&self,input: &str,mut depth: i32) -> Option<Data>{

        self.get_variable_with_depth(input,depth).map(|x| x.0)

    }
    pub fn erase_depth(&mut self, depth: i32){
        self.depth_datas.remove(&depth);
    }
    pub fn set_variable(&mut self,input: &str, depth: i32, data: Data){
        let mut depth_data : &mut DepthData;
        if let Some(gotten_depth_data) = self.depth_datas.get_mut(&depth){
            depth_data = gotten_depth_data;
        } else{
            self.depth_datas.insert(depth, DepthData{variables: HashMap::new()});
            depth_data = self.depth_datas.get_mut(&depth).unwrap();
        }

       if  depth_data.variables.insert(input.to_string(),data.clone()).is_some() {
           depth_data.variables.remove(&input.to_string());
           depth_data.variables.insert(input.to_string(),data);
       }


    }
}
pub struct SyntaxTree{

    nodes: Vec<SyntaxNode>,

}
impl Debug for SyntaxTree{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.nodes.fmt(f)
    }
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
            peekable: tokens.peekable(),
            depth: 0,
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



    fn parse_instruction(&mut self) -> SyntaxNode {

        if let Some(spawn_or_print) = self.peekable.peek().cloned() {
            match spawn_or_print {
                Token::While => {
                    self.peekable.next();
                    let expression = self.parse_expr();
                    let while_code = self.parse_instruction();
                    let node = SyntaxNode::new(SyntaxNodeVariant::While {
                        condition: expression,
                        execution: Box::new(while_code),
                    },self.depth);
                    return node;
                }
                Token::Name(name) =>{
                    self.peekable.next();
                    if let Some(Token::Assign) = self.peekable.next()   {
                        let created = SyntaxNode::new(
                            SyntaxNodeVariant::Reassignment { equals_to: self.parse_expr(),name },
                            self.depth);

                         return   self.parse_finish_line(created);
                    } else{
                        panic!("Expected  =");
                    }
                }
                Token::If => {
                    self.peekable.next();
                    let expression = self.parse_expr();
                    let mut else_cond = None;
                    let if_code = self.parse_instruction();
                    if let Some(Token::Else) = self.peekable.peek().cloned(){
                        self.peekable.next();
                        let parsed_instruction = self.parse_instruction();
                        else_cond = Some(parsed_instruction);
                    }
                    let the_node = SyntaxNode::new(SyntaxNodeVariant::If {
                        condition: expression,
                        else_execution: else_cond.map(|x| Box::new(x)),
                        execution: Box::new(if_code),

                    },self.depth);
                    return the_node;
                }
                Token::LeftCurlyBrace => {

                    self.peekable.next();
                    self.depth += 1;
                    let mut instructions = Vec::new();
                    loop {
                        match  self.peekable.peek().cloned(){
                            Some(Token::RightCurlyBrace) => {
                                self.peekable.next();
                                break;
                            }
                            _ => {
                                instructions.push(self.parse_instruction());
                            }
                        }
                    }

                    let the_node =
                        SyntaxNode::new(SyntaxNodeVariant::Block {
                            instructions: instructions,
                        } ,self.depth);
                    self.depth -= 1;
                   return the_node;



                }
                Token::Yap => {
                    while let Some(Token::Yap) = self.peekable.peek().cloned() {
                        self.peekable.next();
                        if let Some(Token::LeftBracket) = self.peekable.next()  {
                            let created = SyntaxNode::new(
                                SyntaxNodeVariant::Yap(self.parse_expr()),
                            self.depth); ;
                            if let Some(Token::RightBracket) = self.peekable.next()  {
                                return
                                    self.parse_finish_line(created);
                            } else{
                                panic!("Right bracket missing on yap");
                            }
                        } else{
                            panic!("Expected left bracket");
                        }
                    };
                },
                Token::Bag => {
                    self.peekable.next();
                    if let Some(Token::Name(name)) = self.peekable.next()
                        && let Some(Token::Assign) = self.peekable.next()   {
                        let created = SyntaxNode::new(
                             SyntaxNodeVariant::Declaration { equals_to: self.parse_expr(),name },
                            self.depth);
                        return
                            self.parse_finish_line(created);
                    } else{
                        panic!("Expected variable-name =");
                    }
                }
                _ => {panic!("Invalid token.")}
            }
        };
        panic!("Unexpected token: {:?}", self.peekable.next());
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
            Some(Token::Operator(Operator::Minus)) => {
                Expression::SingularExpression {
                    operator: Operator::Minus,
                    expression: Box::new(self.parse_primary())
                }
            }
            Some(Token::StringLiteral(string)) => {
                Expression::Data(Data::String(string))
            }
            Some(Token::Operator(Operator::No)) =>{
                Expression::SingularExpression {
                    operator: No,
                    expression: Box::new(self.parse_primary())
                }
            },
            Some(Token::Number(number)) => {
                Expression::Data(Number(number))
            },
            Some(Token::Cap) => {
                Expression::Data(Boolean(false))
            },
            Some(Token::Fax) => {
                Expression::Data(Boolean(true))
            },
            Some(Token::LeftBracket) => {
                let expr = self.parse_expr();
                if let Some(Token::RightBracket) = self.peekable.next() {
                    Expression::Bracketed(Box::new(expr))
                } else {
                    panic!("Expected right bracket");
                }
            },
            Some(Token::Name(name)) => {
                Expression::Variable{
                    name,
                    depth: self.depth,
                }
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
    fn parse_less_than_greater_than(&mut self) -> Expression{
        let mut left = self.parse_add_sub();
        while let Some(Token::Operator(operator @ (Operator::LessThan | Operator::GreaterThan))) = self.peekable.peek().cloned() {
            self.peekable.next();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(self.parse_add_sub())
            };
        };
        return left;
    }
    fn parse_equals_to(&mut self) -> Expression{
        let mut left = self.parse_less_than_greater_than();
        while let Some(Token::Operator(operator @ (Operator::Equals | Operator::NotEquals))) = self.peekable.peek().cloned() {
            self.peekable.next();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(self.parse_less_than_greater_than())
            };
        };
        return left;
    }
    fn parse_and(&mut self) -> Expression{
        let mut left = self.parse_equals_to();
        while let Some(Token::Operator(operator @ (Operator::And))) = self.peekable.peek().cloned() {
            self.peekable.next();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(self.parse_equals_to())
            };
        };
        return left;
    }
    fn parse_or(&mut self) -> Expression{
        let mut left = self.parse_and();
        while let Some(Token::Operator(operator @ (Operator::Or))) = self.peekable.peek().cloned() {
            self.peekable.next();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(self.parse_and())
            };
        };
        return left;
    }
    fn parse_expr(&mut self) -> Expression{
        self.parse_or()
    }

    pub fn compile(&mut self) -> SyntaxTree{
        SyntaxTree{nodes: self.start()}

    }
    fn start(&mut self) -> Vec<SyntaxNode>{
        let mut za_vec = Vec::new();
        while let Some(_) = self.peekable.peek(){
            za_vec.push(self.parse_instruction());
        }


        return za_vec;
    }
}
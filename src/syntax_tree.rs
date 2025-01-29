use std::fmt::{Debug, Formatter};
use crate::Data::Data;
use crate::expression::Expression;
use crate::parser::ProgramData;

pub struct SyntaxNode{
    syntax_node_variant: SyntaxNodeVariant,
    depth: i32
}
impl Debug for SyntaxNode{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.syntax_node_variant.fmt(f)
    }
}
impl SyntaxNode{
    pub fn new(syntax_node_variant: SyntaxNodeVariant, depth: i32) -> SyntaxNode{
        SyntaxNode{syntax_node_variant, depth}
    }
}

#[derive(Debug)]
pub enum SyntaxNodeVariant{
    If{
        condition: Expression,
        execution: Box<SyntaxNode>,
        else_execution: Option<Box<SyntaxNode>>,
    },
    Block{

        instructions: Vec<SyntaxNode>,
    },
    Reassignment {name: String, equals_to: Expression},
    Declaration {name: String, equals_to: Expression},
    Expression(Expression),
    Yap(Expression),
    While{
        condition: Expression,
        execution: Box<SyntaxNode>,
    }
}

impl SyntaxNode{
    pub fn eval(&self, program_data: &mut ProgramData){
        match &self.syntax_node_variant {
            SyntaxNodeVariant::Declaration {name,  equals_to}  =>{
                let gotten_with_depth = program_data.get_variable_with_depth(name.as_str(), self.depth);

                if gotten_with_depth.is_some() && gotten_with_depth.as_ref().unwrap().1 == self.depth{
                    panic!("Variable {} already exists in depth {}!", name,self.depth);
                } else{
                    let evaluated =equals_to.eval(program_data);
                    program_data.set_variable(name.as_str(),self.depth, evaluated);
                }

            },

            SyntaxNodeVariant::Yap(expression)=> {
                println!("{}",expression.eval(program_data))
            }
            SyntaxNodeVariant::Block {   instructions} => {
                for i in instructions{
                    i.eval(program_data);
                }
                program_data.erase_depth(self.depth);
            }
            SyntaxNodeVariant::If{condition, execution, else_execution} => {
                if let Data::Boolean(condition_happened)  = condition.eval(program_data) {
                    if condition_happened{

                            execution.eval(program_data);


                    } else{
                        if let Some(else_execution) = else_execution{

                                else_execution.eval(program_data);


                        }
                    }

                } else{
                    panic!("If condition not a boolean!")
                }
            }
            SyntaxNodeVariant::While { condition, execution } => {
                while let Data::Boolean(condition_happened)  = condition.eval(program_data) {
                    if condition_happened{
                        execution.eval(program_data);
                    }
                }
            }
            SyntaxNodeVariant::Reassignment { equals_to, name} => {
                let gotten_with_depth = program_data.get_variable_with_depth(name.as_str(), self.depth);
                if  let Some(data) =gotten_with_depth{
                    let evaled =equals_to.eval(program_data);
                    program_data.set_variable(name.as_str(),data.1,evaled );

                } else{
                    panic!("Variable {} has not been declared!", name);
                }
            }
            _ => {}
        }
    }
}
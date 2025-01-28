
use crate::number::Number;
use crate::operator::Operator;
use crate::parser::ProgramData;

pub enum SyntaxNode{
    Declaration {name: String, equals_to: Expression},
    Expression(Expression),
    Yap(Expression),
}

impl SyntaxNode{
    pub fn eval(self, program_data: &mut ProgramData){
        match self {
            SyntaxNode::Declaration {name,  equals_to}  =>{
                if program_data.variables.get(&name).is_some(){
                   panic!("Variable {} already exists!", name);
                } else{
                    let evaluated =equals_to.eval(program_data);
                    program_data.variables.insert(name,evaluated);
                }

            },

            SyntaxNode::Yap(expression)=> {
                println!("{}",expression.eval(program_data))
            }
            _ => {}
        }
    }
}
pub enum Expression {
    Variable(String),
    Bracketed(Box<Self>),
    Number(Number),

    BinaryExpression{
        left: Box<Self>,
        operator: Operator,
        right: Box<Self>,
    },
}
impl Expression {

    pub fn eval(self, program_data: &mut ProgramData) -> Number{
        match self {
            Expression::Variable(name) => {
                program_data.variables.get(&name).expect(format!("Variable {} not found!", name).as_str()).clone()
            }
            Expression::Bracketed(input) => {
                input.eval(program_data)
            }
            Expression::Number(number) => {
                return number;
            }

            Expression::BinaryExpression { left,operator,right } => {
                Operator::evaluate(left.eval(program_data), operator, right.eval(program_data))
            }
        }
    }
}
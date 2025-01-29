use crate::Data::Data;
use crate::number::Number;
use crate::operator::Operator;
use crate::parser::ProgramData;


#[derive(Debug)]
pub enum Expression {
    Variable{
        name: String,
        depth: i32,
    },
    Bracketed(Box<Self>),
    Data(Data),

    SingularExpression{
        operator: Operator,
        expression: Box<Self>
    },
    BinaryExpression{
        left: Box<Self>,
        operator: Operator,
        right: Box<Self>,
    },
}
impl Expression {


    pub fn eval(&self, program_data: &mut ProgramData) -> Data{
        match self {
            Expression::Variable{ name, depth} => {
                program_data.get_variable(name.as_str(), *depth).expect(format!("Variable {} not found!", name).as_str()).clone()
            }
            Expression::Bracketed(input) => {
                input.eval(program_data)
            }
            Expression::Data(data) => {
                return data.clone();
            }

            Expression::BinaryExpression { left,operator,right } => {
                Operator::evaluate(left.eval(program_data), *operator, right.eval(program_data))
            }
            Expression::SingularExpression { operator, expression } => {
                Operator::evaluate_single(*operator, expression.eval(program_data))
            }
        }
    }
}
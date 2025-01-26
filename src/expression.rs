use crate::number::Number;
use crate::operator::Operator;

pub enum Expression {
    Bracketed(Box<Self>),
    Number(Number),

    BinaryExpression{
        left: Box<Self>,
        operator: Operator,
        right: Box<Self>,
    },
}
impl Expression {

    pub fn eval(self) -> Number{
        match self {
            Expression::Bracketed(input) => {
                input.eval()
            }
            Expression::Number(number) => {
                return number;
            }

            Expression::BinaryExpression { left,operator,right } => {
                Operator::evaluate(left.eval(), operator, right.eval())
            }
        }
    }
}
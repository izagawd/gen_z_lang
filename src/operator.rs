use crate::number::Number;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator{
    Plus,
    Minus,
    Multiply,
    Divide,
}


impl Operator{
    #[inline]
    pub fn evaluate(first: Number, operator: Operator, second: Number)-> Number{
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

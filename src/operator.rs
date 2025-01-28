use std::fmt::{Debug, Display, Formatter};
use crate::Data::Data;
use crate::Data::Data::{Boolean, Number, String};
use crate::number::Number::Int;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Equals{
    Is, EqualsSign
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator{
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    No,

    And,
    Or,

}

impl Display for Operator{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
impl Operator{
    pub fn evaluate_single(operator: Operator, data: Data) -> Data{
        return match (operator,data) {
            (Operator::Minus, Data::Number(n)) =>{
                return Data::Number(-n);
            }
            (Operator::Plus, Data::Number(n)) =>{
                return Data::Number(n);
            }
            (Operator::No, Data::Boolean(b)) =>{
                return Data::Boolean(!b);
            }
            _ => {
                return panic!()
            }
        }
    }
    #[inline]
    pub fn evaluate(first: Data, operator: Operator, second: Data)-> Data{
        return match ( first,operator, second) {

            (Number(number_one), Operator::Plus, Number(number_two)) =>{
                return Data::Number(number_one + number_two);
            },
            (Number(number_one), Operator::Minus, Number(number_two)) => {
                return Data::Number(number_one - number_two);
            }
            (Number(number_one), Operator::Multiply, Number(number_two)) => {
                return Data::Number(number_one * number_two);
            }
            (Number(number_one), Operator::Divide, Number(number_two)) => {
                return Data::Number(number_one / number_two);
            }
            (Boolean(boolean_one), Operator::And, Boolean(number_two)) =>{
                return Data::Boolean(boolean_one && number_two);
            },
            (Boolean(boolean_one), Operator::Or, Boolean(number_two))=>{
                return Data::Boolean(boolean_one || number_two);
            },
            (String(string_one), Operator::Plus, String(string_two))=>{
                let mut copy = string_one.clone();
                copy.push_str(string_two.as_str());
                return Data::String(copy);
            },
            (String(string), Operator::Plus, Number(number))=>{
                return String(format!("{string}{number}"))
            }
            (Number(number), Operator::Plus, String(string))=>{
                return String(format!("{number}{string}"))
            }
            (first, Operator::Equals, second)=>{
                return Data::Boolean(first == second)
            }
            (first,operator,second) => panic!("Cannot evaluate {} and {} with {}!",first.to_string(),second.to_string(),
            operator.to_string(),)
        }
    }
}

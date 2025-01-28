use crate::number::Number;
use crate::operator::Operator;

#[derive(Debug, Clone)]
pub enum Token{
    LeftBracket,
    RightBracket,
    Number(Number),
    Operator(Operator),
    Name(String),
    //meaning ! as in !true
    Cap,
    Fax,
    /// Means let. used to assign data to variables. EG bag a = 5;
    Bag,
    SemiColon,
    /// means true
    Yap,
    /// means false
    Assign,

}
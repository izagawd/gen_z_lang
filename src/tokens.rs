use std::fmt::{Display, Formatter};
use crate::number::Number;
use crate::operator::Operator;

#[derive(Debug, Clone)]
pub enum Token{
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftBracket,
    RightBracket,
    Number(Number),
    Operator(Operator),
    Name(String),
    StringLiteral(String),
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

impl Display for Token{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}
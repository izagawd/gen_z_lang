use std::collections::HashMap;
use std::fmt::{format, Display};
use crate::number::Number;
use crate::operator::Equals;

#[derive(Debug, Clone)]
pub enum Data{
    Number(Number),
    String(String),
    Object(HashMap<String, Data>),
    Boolean(bool),
    Array(Vec<Data>),
    None
}
impl PartialEq for Data{
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Data::Number(a),Data::Number(b)) => a == b,
            (Data::String(a),Data::String(b)) => a == b,
            (Data::Boolean(a),Data::Boolean(b)) => a == b,
            (Data::None,Data::None) => true,
            _ => panic!("Cannot compare {self} to {other}")
        }
    }
}
impl Display for Data{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Data::Number(number) => { number.to_string() }
            Data::String(string) => { string.to_string() }
            Data::Object(_) => { "[Object]".to_string() }
            Data::Boolean(val) => {
                match val {
                    true => "fax".to_string(),
                    false => "cap".to_string()
                }
            }
            Data::Array(array) => {
                let mut to_send = String::from("[");
                for i in array.iter() {
                    to_send.push_str(format!("{},", i.to_string().as_str()).as_str());
                };
                to_send.push_str("]");
                to_send
            }
            Data::None => {
                "none".to_string()
            }
        };
        write!(f, "{}", str)
    }
}

impl Data{

}
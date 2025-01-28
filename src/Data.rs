use std::collections::HashMap;
use crate::number::Number;

enum Data{
    Number(Number),
    String(String),
    Object(HashMap<String, Data>),
    Boolean(bool),
    Array(Vec<Data>),
    None
}

impl Data{

}
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::io::stderr;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq)]
pub enum Number{
    Int(i32),
    Float(f32),
}
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
impl Add for Number{
    type Output = Number;
    fn add(self, other: Number) -> Number{
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a + b),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f32 + b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a + b as f32),
        }
    }
}
impl PartialOrd for Number{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => a.partial_cmp(b),
            (Number::Int(a), Number::Float(b)) => (*a as f32).partial_cmp(b),
            (Number::Float(a), Number::Float(b)) => a.partial_cmp(b),
            (Number::Float(a), Number::Int(b)) => a.partial_cmp(&(*b as f32))
        }
    }
}
impl Neg for Number{
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Int(int) => { Number::Int(-int)}
            Number::Float(float) => { Number::Float(-float)}
        }
    }
}
impl Debug for Number{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{:?}", i),
            Number::Float(a) => write!(f, "{:?}", a),
        }
    }
}
impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a - b),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f32 - b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a - b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a - b as f32),
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a * b),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f32 * b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a * b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a * b as f32),
        }
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a / b), // Integer division
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f32 / b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a / b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a / b as f32),
        }
    }
}
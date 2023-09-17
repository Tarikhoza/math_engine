use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Braces {
    pub math: Box<Math>,
    pub exponent: Option<Box<Math>>,
}

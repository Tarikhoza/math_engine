use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Braces {
    pub inner: Box<Math>,
    pub exponent: Option<Box<Math>>,
}

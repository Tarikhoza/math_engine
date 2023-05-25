use crate::math::Math;

#[derive(Debug, Clone)]
pub struct Fraction {
    pub numerator: Box<Math>,
    pub denominator: Box<Math>,
}

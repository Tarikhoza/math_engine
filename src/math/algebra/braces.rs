use crate::math::algebra::variable::Variable;
use crate::math::Math;
use rust_decimal_macros::dec;

use crate::solver::step::Step;

#[derive(Debug, Clone, PartialEq)]
pub struct Braces {
    pub math: Box<Math>,
    pub exponent: Option<Box<Math>>,
}

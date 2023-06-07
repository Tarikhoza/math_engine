use std::default;

use crate::math::algebra::braces::Braces;
use crate::math::Math;
//use crate::variable::Variable;
//use std::ops;

#[derive(Debug, Clone, Default)]
pub struct Root {
    pub math: Box<Math>,
    pub base: Option<Box<Math>>,
}

use crate::math::algebra::variable::Variable;
use crate::math::Math;

#[derive(Debug, Clone)]
pub struct Function {
    pub label: String,
    pub args: Vec<String>,
    pub definition: Option<Box<Math>>,
    pub exponent: Option<Box<Math>>,
}

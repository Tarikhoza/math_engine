use crate::math::algebra::variable::Variable;
use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub label: String,
    pub args: Vec<String>,
    pub definition: Option<fn(Math) -> Math>,
    pub exponent: Option<Box<Math>>,
}

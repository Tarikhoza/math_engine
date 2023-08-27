use crate::math::Math;

#[derive(Clone, PartialEq, Debug)]
pub struct Product {
    pub begining: Box<Math>,
    pub end: Box<Math>,
    pub iter_suffix: String,
    pub math: Box<Math>,
}

use crate::math::Math;

#[derive(Clone, PartialEq, Debug)]
pub struct Product {
    pub start: Box<Math>,
    pub end: Box<Math>,
    pub iter: String,
    pub inner: Box<Math>,
}

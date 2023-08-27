use crate::math::Math;

#[derive(Clone, PartialEq, Debug)]
pub struct Sum {
    pub begining: Box<Math>,
    pub end: Box<Math>,
    pub iter_suffix: String,
    pub math: Box<Math>,
}

//\sum_{n=1}^{\infty} 2^{-n}

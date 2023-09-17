use crate::math::algebra::operations::Operations;
use crate::math::linear_algebra::vector::Vector;
use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub factors: Vec<Vector>,
}

impl Matrix {
    pub fn add_all(&self) -> Math {
        self.factors
            .iter()
            .map(|v| v.add_all())
            .fold(Math::default(), |acc, e| acc.add(&e))
    }
}

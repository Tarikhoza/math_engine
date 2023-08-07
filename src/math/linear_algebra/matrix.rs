use crate::math::linear_algebra::vector::Vector;
use crate::math::operator::algebra::{Operations, Operator};
use crate::math::Math;
use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub factors: Vec<Vector>,
}

impl Matrix {
    pub fn add_all(&self) -> Math {
        let mut result: Math = self.factors.get(0).unwrap().clone().add_all();
        for factor in self.factors.iter().skip(1) {
            result = result.add(&factor.add_all());
        }
        result
    }
}

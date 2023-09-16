use crate::castable::Castable;
use crate::math::algebra::operations::{Operations, Operator};
use crate::math::linear_algebra::vector::Vector;
use crate::math::Math;
use crate::parser::ParsablePrimitiveAsVariable;
use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub factors: Vec<Vector>,
}

impl Matrix {
    pub fn add_all(&self) -> Math {
        let mut result: Math;
        if let Some(first) = self.factors.first() {
            result = first.add_all();
        } else {
            return 0_i64.as_variable().as_math();
        }
        for factor in self.factors.iter().skip(1) {
            result = result.add(&factor.add_all());
        }
        result
    }
}

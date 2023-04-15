use crate::math::linear_algebra::vector::Vector;
use crate::math::operator::algebra::{Operations, Operator};
use crate::math::Math;
use std::ops;

#[derive(Debug, Clone)]
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

impl ops::Add<Math> for Matrix {
    type Output = Math;
    fn add(self, rhs: Math) -> Math {
        match rhs {
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Matrix {
    type Output = Math;
    fn sub(self, rhs: Math) -> Math {
        match rhs {
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Matrix {
    type Output = Math;
    fn mul(self, rhs: Math) -> Math {
        match rhs {
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Matrix {
    type Output = Math;
    fn div(self, _rhs: Math) -> Math {
        todo!()
    }
}

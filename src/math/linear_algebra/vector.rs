use crate::math::algebra::variable::Variable;
use crate::math::linear_algebra::matrix::Matrix;
use crate::math::operator::algebra::{Operations, Operator};
use crate::math::Math;
use crate::parser::Parsable;
use itertools::Itertools;
use rust_decimal_macros::dec;
use std::ops;

#[cfg(feature = "step-tracking")]
use crate::solver::step::Step;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub factors: Vec<Math>,
    #[cfg(feature = "step-tracking")]
    pub step: Option<Step>,
}

impl Vector {
    fn get_bases(&self) -> Vec<String> {
        self.factors
            .iter()
            .map(|m| m.add_sub_base())
            .unique()
            .collect()
    }

    fn non_matching_to_zero(&self, base: &str) -> Vector {
        let factors = self
            .factors
            .iter()
            .map(|m| {
                if m.add_sub_base() == base {
                    m.clone()
                } else {
                    Math::Variable(Variable {
                        value: dec!(0),
                        suffix: String::new(),
                        exponent: None,
                        #[cfg(feature = "step-tracking")]
                        step: None,
                    })
                }
            })
            .collect();
        Vector {
            factors,
            #[cfg(feature = "step-tracking")]
            step: None,
        }
    }
    pub fn to_based_matrix(&self) -> Matrix {
        Matrix {
            factors: self
                .get_bases()
                .iter()
                .map(|m| self.non_matching_to_zero(m))
                .collect(),
        }
    }

    pub fn add_all(&self) -> Math {
        if self.factors.len() == 0 {
            return Math::Variable(Variable::default());
        }
        let mut result: Math = self.factors.get(0).unwrap().clone();
        for factor in self.factors.iter().skip(1) {
            if factor.clone().to_tex() != "0" {
                result = result.add(factor);
            }
        }
        result
    }
}

impl ops::Add<Math> for Vector {
    type Output = Math;
    fn add(self, rhs: Math) -> Math {
        match rhs {
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Vector {
    type Output = Math;
    fn sub(self, rhs: Math) -> Math {
        match rhs {
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Vector {
    type Output = Math;
    fn mul(self, rhs: Math) -> Math {
        match rhs {
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Vector {
    type Output = Math;
    fn div(self, _rhs: Math) -> Math {
        todo!()
    }
}

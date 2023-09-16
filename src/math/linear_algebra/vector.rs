use crate::castable::Castable;
use crate::math::algebra::operations::{Operations, Operator};
use crate::math::algebra::variable::Variable;
use crate::math::linear_algebra::matrix::Matrix;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable};
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
        if self.factors.is_empty() {
            return Math::Variable(Variable::default());
        }
        let mut factors: Vec<Math> = self.factors.clone();

        let mut result: Math;

        if let Some(first) = factors.first() {
            result = first.clone();
        } else {
            return 0_i64.as_variable().as_math();
        }

        for factor in factors.iter().skip(1) {
            result = result.add(factor);
        }
        result
    }
}

use crate::logging::env_info;
use crate::math::algebra::operations::Operations;
use crate::math::algebra::polynom::{Polynom, PolynomPart};
use crate::math::algebra::variable::Variable;
use crate::math::linear_algebra::matrix::Matrix;
use crate::math::Math;
use itertools::Itertools;
use rust_decimal_macros::dec;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub factors: Vec<Math>,
}

impl Vector {
    fn get_bases(&self) -> Vec<String> {
        env_info("helper", format!("get_bases vector {:#?}", self));
        self.factors
            .iter()
            .map(|m| m.add_sub_base())
            .unique()
            .collect()
    }

    fn not_matching_to_zero(&self, base: &str) -> Vector {
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
                    })
                }
            })
            .collect();
        Vector { factors }
    }

    pub fn to_based_matrix(&self) -> Matrix {
        env_info(
            "helper",
            format!("to_based_matrix vector before {:#?}", self),
        );
        let res = Matrix {
            factors: self
                .get_bases()
                .iter()
                .map(|m| self.not_matching_to_zero(m))
                .collect(),
        };
        env_info("helper", format!("to_based_matrix vector after {:#?}", res));
        res
    }

    pub fn add_all(&self) -> Math {
        env_info("helper", format!("add_all vector before {:#?}", self));

        let mut parts = Vec::<PolynomPart>::new();

        let result = self
            .factors
            .iter()
            .fold(Math::default(), |acc, e| acc.add(e));

        if let Math::Polynom(r) = result {
            let morphed = r.morph_ops();
            env_info("helper", format!("add_all vector after {:#?}", morphed));
            return Math::Polynom(morphed.morph_ops());
        }

        env_info("helper", format!("add_all vector after {:#?}", result));
        result
    }
}

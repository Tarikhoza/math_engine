use crate::math::Math;
use crate::matrix::Matrix;
use crate::parser::Parsable;
use crate::variable::Variable;
use itertools::Itertools;
use rust_decimal_macros::dec;

#[derive(Debug, Clone)]
pub struct Vector {
    pub factors: Vec<Math>,
}

impl Parsable for Vector {
    fn to_tex(&self) -> String {
        let s: String = self
            .factors
            .iter()
            .map(|m| m.to_tex())
            .collect::<Vec<_>>()
            .join(",\t");

        format!("[{}]", s)
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        todo!()
    }

    #[must_use]
    fn on_begining(_tex: String) -> Option<String> {
        None
    }
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
                    })
                    .clone()
                }
            })
            .collect();
        Vector { factors }
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
}

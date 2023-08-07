use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::variable::Variable;
use crate::math::linear_algebra::vector::Vector;
use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};
use crate::math::operator::Operator;
use crate::math::Math;

use crate::parser::{Parsable, ParsableGenerics};

use rust_decimal_macros::dec;

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynom {
    pub factors: Vec<Math>,
    pub operators: Vec<Operator>,
    #[cfg(feature = "step-tracking")]
    pub step: Option<Step>,
}

impl Polynom {
    pub fn unpack(&self) -> Math {
        if self.factors.len() == 1 {
            return self.factors[0].clone();
        }
        Math::Polynom(self.clone())
    }

    pub fn as_fraction(&self) -> Fraction {
        Fraction {
            whole: None,
            numerator: Box::new(Math::Polynom(self.clone())),
            denominator: Box::new("1".parse_math().expect("error parsing 1 as math")),
        }
    }

    pub fn morph_double_operator(&self) -> Math {
        //TODO this is a hack
        let ret = self
            .to_tex()
            .replace("++", "+")
            .replace("--", "+")
            .replace("+-", "-")
            .replace("-+", "-");
        if ret != self.to_tex() {
            return ret
                .parse_math()
                .expect("an error happened while morphing double operators");
        }
        self.unpack()
    }

    pub fn to_vector(&self) -> Vector {
        let mut factors: Vec<Math> = self
            .operators
            .iter()
            .zip(self.factors.iter().skip(1))
            .map(Math::morph_operator)
            .collect();

        factors.insert(0, self.factors.get(0).cloned().unwrap_or_default());

        Vector {
            factors,
            #[cfg(feature = "step-tracking")]
            step: None,
        }
    }
}

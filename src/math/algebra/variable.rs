use fancy_regex::Regex;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::default;

use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};

use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::operator::Operator;
use crate::math::Math;

use crate::parser::{Parsable, ParsableGenerics, ParsableGenericsAsVariable, Parser};

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Variable {
    pub value: Decimal,
    pub suffix: String,
    pub exponent: Option<Box<Math>>,
    #[cfg(feature = "step-tracking")]
    pub step: Option<Step>,
}

fn ascii_score(s: &str) -> u32 {
    let mut score = 0;
    for (i, c) in s.chars().enumerate() {
        score = c.to_digit(10).unwrap_or(1000) / (i + 1) as u32;
    }
    score
}

impl Variable {
    pub fn is_integer(&self) -> bool {
        self.value.abs() - self.value.abs().round() == dec!(0)
            && self.suffix.is_empty()
            && self.get_exponent().to_tex() == "1"
    }

    pub fn as_polynom(&self) -> Polynom {
        Polynom {
            factors: vec![Math::Variable(self.clone())],
            operators: vec![],
            #[cfg(feature = "step-tracking")]
            step: None,
        }
    }

    pub fn as_fraction(&self) -> Fraction {
        Fraction {
            numerator: Box::new(Math::Variable(self.clone())),
            denominator: Box::new(1.parse_math().expect("error parsing 1 as math")),
        }
    }

    pub fn split_operator(&self) -> (Operator, Math) {
        if self.value < dec!(0) {
            return (
                Operator::Algebra(AlgebraOperator::Subtraction),
                self.negative(),
            );
        }
        (
            Operator::Algebra(AlgebraOperator::Addition),
            Math::Variable(self.clone()),
        )
    }

    pub fn sort_score(&self) -> u32 {
        u32::MAX - (ascii_score(&self.suffix) + ascii_score(&self.get_exponent().to_tex()))
    }

    pub fn add_sub_base(&self) -> String {
        let mut x = self.clone();
        x.value = dec!(1);
        x.to_tex()
    }
}

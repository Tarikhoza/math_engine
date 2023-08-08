use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};
use crate::math::operator::Operator;
use crate::math::Math;

use crate::parser::{Parsable, ParsableGenerics, ParsableGenericsAsVariable, Parser};

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

use fancy_regex::Regex;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::default;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Variable {
    pub value: Decimal,
    pub suffix: String,
    pub exponent: Option<Box<Math>>,
    #[cfg(feature = "step-tracking")]
    pub step: Option<Step>,
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
            whole: None,
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

    pub fn add_sub_base(&self) -> String {
        if self.value.is_zero() {
            return String::from("");
        }
        let mut x = self.clone();
        x.value = dec!(1);
        x.to_tex()
    }

    pub fn sorting_score(&self) -> i64 {
        let mut suffix_score: i64 = 0;
        if !self.suffix.is_empty() {
            for (i, c) in self.suffix.chars().rev().enumerate() {
                suffix_score += (c as i64).pow((i + 1) as u32);
            }
            if self.value.is_sign_negative() {
                suffix_score += 255;
            }
        } else {
            suffix_score = u16::MAX as i64;
        }
        if self.exponent.is_some() {
            if self.get_exponent().to_tex() != "1" {
                suffix_score -= self.get_exponent().sorting_score() - 200;
            }
        }

        suffix_score
    }
}

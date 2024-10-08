use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::operations::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};
use crate::math::Math;

use crate::parser::Parsable;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Variable {
    pub value: Decimal,
    pub suffix: String,
    pub exponent: Option<Box<Math>>,
}

impl Variable {
    pub fn is_integer(&self) -> bool {
        self.value.abs() - self.value.abs().round() == dec!(0)
            && self.suffix.is_empty()
            && self.get_exponent().to_tex() == "1"
    }

    pub fn split_operator(&self) -> (AlgebraOperator, Math) {
        if self.value < dec!(0) {
            return (AlgebraOperator::Subtraction, self.negative());
        }
        (AlgebraOperator::Addition, Math::Variable(self.clone()))
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
        if self.exponent.is_some() && self.get_exponent().to_tex() != "1" {
            suffix_score -= self.get_exponent().sorting_score() - 200;
        }

        suffix_score
    }
}

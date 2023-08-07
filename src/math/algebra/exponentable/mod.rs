pub mod braces;
pub mod function;
pub mod variable;

use crate::math::AlgebraOperations;
use crate::math::Math;
use crate::math::Variable;
use crate::parser::{Parsable, ParsableGenerics};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub trait Exponentable {
    fn get_exponent(&self) -> Math;
    fn without_exponent(&self) -> Math;
    fn with_exponent(&self) -> Math;
    fn is_exponentiable(&self) -> bool;
    fn apply_exponent(&self) -> Math {
        if !self.is_exponentiable() {
            return self.with_exponent();
        }
        if let Math::Variable(exponent) = self.get_exponent() {
            if exponent.value.is_one() {
                return self.without_exponent();
            } else if exponent.value.is_zero() {
                return "1".parse_math().expect("cannot parse 1 as math");
            }
            if exponent.is_integer() {
                let orig = self.without_exponent().simplify();
                let mut value = orig.clone();
                if exponent.value.is_sign_positive() {
                    for i in 1..exponent.value.to_i64().unwrap() {
                        value = value.mul(&orig);
                    }
                }
                if exponent.value.is_sign_negative() {
                    todo!("implement negative exponent");
                    for i in 1..exponent.value.to_i64().unwrap() {
                        value = value.mul(&orig);
                    }
                }

                return value;
            }
        }
        self.with_exponent()
    }
}

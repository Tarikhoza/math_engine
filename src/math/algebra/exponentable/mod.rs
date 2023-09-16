pub mod braces;
pub mod function;
pub mod variable;

use crate::castable::Castable;
use crate::math::simplifiable::Simplifiable;
use crate::math::AlgebraOperations;
use crate::math::Math;
use crate::math::Variable;
use crate::parser::ParsablePrimitiveAsVariable;
use crate::parser::{Parsable, ParsablePrimitive};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub trait Exponentable {
    fn get_exponent(&self) -> Math;
    fn set_exponent(&self, exponent: Math) -> Math;
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
                return 1_i64.as_variable().as_math();
            }
            if exponent.is_integer() {
                let orig = self.without_exponent().simplify();
                let mut value = orig.clone();
                if exponent.value.is_sign_positive() {
                    let mut i = dec!(1);
                    while (i < exponent.value) {
                        value = value.mul(&orig);
                        i += dec!(1);
                    }
                } else if exponent.value.is_sign_negative() {
                    return self
                        .set_exponent(self.get_exponent().negative())
                        .as_fraction()
                        .inverse()
                        .simplify()
                        .as_math();
                }

                return value;
            }
        }
        self.with_exponent()
    }
}

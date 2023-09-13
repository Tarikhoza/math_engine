use crate::math::algebra::operations::Operations;
use crate::math::algebra::variable::Variable;
use crate::math::Math;

use crate::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable, Parser};

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[derive(Debug, Clone, PartialEq)]
pub struct Fraction {
    pub whole: Option<Decimal>,
    pub numerator: Box<Math>,
    pub denominator: Box<Math>,
}

impl Fraction {
    pub fn inverse(&self) -> Fraction {
        Fraction {
            whole: self.whole,
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }

    pub fn expand(&self, other: Math) -> Fraction {
        Fraction {
            whole: self.whole,
            numerator: Box::new(self.numerator.mul(&other)),
            denominator: Box::new(self.denominator.mul(&other)),
        }
    }

    pub fn split_whole(&self) -> (Variable, Fraction) {
        let whole: Variable = self.whole.unwrap_or(dec!(0)).as_variable();
        let mut fraction = self.clone();

        fraction.whole = None;
        (whole, fraction)
    }
}

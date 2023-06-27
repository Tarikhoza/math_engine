pub mod braces;
pub mod function;
pub mod variable;

use crate::math::AlgebraOperations;
use crate::math::Math;
use crate::math::Variable;
use crate::parser::Parsable;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub trait Exponentable {
    fn get_exponent(&self) -> Math;
    fn without_exponent(&self) -> Math;
    fn with_exponent(&self) -> Math;
    fn apply_exponent(&self) -> Math {
        if let Math::Variable(exponent) = self.get_exponent() {
            if exponent.is_integer() {
                let mut value = self.without_exponent().simplify();
                let orig = value.clone();
                if exponent.value.is_sign_positive() {
                    for i in 1..exponent
                        .value
                        .to_i64()
                        .expect("error converting dec to i64")
                    {
                        value = value.mul(&orig);

                        dbg!(value.to_tex());
                    }
                } else {
                    //TODO check for correctness
                    for i in 1..exponent
                        .value
                        .to_i64()
                        .expect("error converting dec to i64")
                    {
                        value = value.div(&orig)
                    }
                }
                return value;
            } else {
                //Turn into a fraction and apply exponent with root
                dbg!("Exponentable::apply_exponent");
                return self.with_exponent();
            }
        }
        self.with_exponent()
    }
}

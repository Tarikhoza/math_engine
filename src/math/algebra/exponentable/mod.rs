pub mod braces;
pub mod variable;

use crate::math::AlgebraOperations;
use crate::math::Math;
use crate::math::Variable;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub trait Exponentable {
    fn get_exponent(&self) -> Math;
    fn without_exponent(&self) -> Math;
    fn with_exponent(&self) -> Math;
    fn apply_exponent(&self) -> Math {
        if let Math::Variable(exponent) = self.get_exponent() {
            if exponent.is_integer() {
                let mut value = self.without_exponent();
                if exponent.value.is_sign_positive() {
                    for i in 1..exponent
                        .value
                        .to_i64()
                        .expect("error converting dec to i64")
                    {
                        value = value.mul(&value)
                    }
                } else {
                    //TODO check for correctness
                    for i in 1..exponent
                        .value
                        .to_i64()
                        .expect("error converting dec to i64")
                    {
                        value = value.div(&value)
                    }
                }
                return value;
            } else {
                //Turn into a fraction and use with apply exponent with root
                todo!()
            }
        }
        self.with_exponent()
    }
}

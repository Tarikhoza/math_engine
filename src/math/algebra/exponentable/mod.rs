pub mod braces;
pub mod function;
pub mod variable;

use crate::castable::Castable;
use crate::logging::env_info;
use crate::math::algebra::operations::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};
use crate::math::algebra::polynom::PolynomPart;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::ParsablePrimitiveAsVariable;
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
        // TODO remove double code
        match self.get_exponent() {
            Math::Variable(exponent) => {
                env_info(
                    "operations",
                    "apply_exponent found variable as exponent".into(),
                );
                if exponent.value.is_one() {
                    return self.without_exponent();
                } else if exponent.value.is_zero() {
                    return 1_i64.as_variable().as_math();
                }
                if exponent.is_integer() {
                    env_info("operations", "apply_exponent exponent is integer".into());
                    let orig = self.without_exponent().simplify();
                    let mut value = orig.clone();
                    if exponent.value.is_sign_positive() {
                        env_info("operations", "apply_exponent exponent is positive".into());
                        let mut i = dec!(1);
                        while i < exponent.value {
                            value = value.mul(&orig);
                            i += dec!(1);
                        }
                    } else if exponent.value.is_sign_negative() {
                        env_info("operations", "apply_exponent exponent is negative".into());
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
            Math::Polynom(poly) => {
                return match poly.parts.as_slice() {
                    [PolynomPart::Operator(AlgebraOperator::Subtraction), PolynomPart::Math(Math::Variable(exponent))]
                    | [PolynomPart::Math(Math::Variable(exponent))] => {
                        env_info(
                            "operations",
                            "apply_exponent found polynom as exponent".into(),
                        );
                        if exponent.value.is_one() {
                            return self.without_exponent();
                        } else if exponent.value.is_zero() {
                            return 1_i64.as_variable().as_math();
                        }
                        if exponent.is_integer() {
                            env_info("operations", "apply_exponent exponent is integer".into());
                            env_info("operations", "apply_exponent exponent is positive".into());
                            let orig = self.without_exponent().simplify();
                            let mut value = orig.clone();
                            if exponent.value.is_sign_positive() {
                                let mut i = dec!(1);
                                while i < exponent.value {
                                    value = value.mul(&orig);
                                    i += dec!(1);
                                }
                            } else if exponent.value.is_sign_negative() {
                                env_info(
                                    "operations",
                                    "apply_exponent exponent is negative".into(),
                                );
                                return self
                                    .set_exponent(self.get_exponent().negative())
                                    .as_fraction()
                                    .inverse()
                                    .simplify()
                                    .as_math();
                            }

                            return value;
                        }

                        env_info(
                            "operations",
                            "apply_exponent exponent is not an integer, nothing to do".into(),
                        );
                        return self.with_exponent();
                    }
                    _ => {
                        env_info(
                            "operations",
                            "apply_exponent exponent is not an integer, nothing to do".into(),
                        );

                        return self.with_exponent();
                    }
                };
            }
            _ => {
                env_info(
                    "operations",
                    "apply_exponent exponent is not an integer, nothing to do".into(),
                );

                return self.with_exponent();
            }
        }

        self.with_exponent()
    }
}

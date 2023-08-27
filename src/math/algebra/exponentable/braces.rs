use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::operations::Operations;
use crate::math::simplifiable::Simplifiable;
use crate::math::Braces;
use crate::math::Math;
use crate::math::Variable;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

impl Exponentable for Braces {
    fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1),
                suffix: String::new(),
                exponent: None,
                #[cfg(feature = "step-tracking")]
                step: None,
            }),
            Some(e) => *e.clone(),
        }
    }

    fn without_exponent(&self) -> Math {
        self.math.simplify()
    }

    fn with_exponent(&self) -> Math {
        Math::Braces(self.clone())
    }

    fn is_exponentiable(&self) -> bool {
        true
    }
}

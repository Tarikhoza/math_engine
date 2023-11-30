use crate::castable::Castable;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::simplifiable::Simplifiable;
use crate::math::Braces;
use crate::math::Math;
use crate::math::Variable;

use rust_decimal_macros::dec;

impl Exponentable for Braces {
    fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1),
                suffix: String::new(),
                exponent: None,
            }),
            Some(e) => *e.clone(),
        }
    }
    fn set_exponent(&self, exponent: Math) -> Math {
        let mut new_math = self.clone();
        new_math.exponent = Some(Box::new(exponent));
        new_math.as_math()
    }

    fn without_exponent(&self) -> Math {
        self.inner.simplify()
    }

    fn with_exponent(&self) -> Math {
        Math::Braces(self.clone())
    }

    fn is_exponentiable(&self) -> bool {
        true
    }
}

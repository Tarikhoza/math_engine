use crate::math::algebra::exponentable::Exponentable;
use crate::math::Function;
use crate::math::Math;
use crate::math::Variable;

use rust_decimal_macros::dec;

impl Exponentable for Function {
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
    fn set_exponent(&self, _exponent: Math) -> Math {
        todo!();
        //let mut new_math = self.clone();
        //new_math.exponent = Some(Box::new(_exponent));
        //        new_math.as_math()
    }

    fn without_exponent(&self) -> Math {
        let mut value = self.clone();
        value.exponent = None;
        Math::Function(value)
    }

    fn with_exponent(&self) -> Math {
        Math::Function(self.clone())
    }

    fn is_exponentiable(&self) -> bool {
        false
    }
}

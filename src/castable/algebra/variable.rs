use crate::castable::Castable;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::ParsablePrimitiveAsVariable;

impl Castable for Variable {
    fn as_math(&self) -> Math {
        Math::Variable(self.clone())
    }

    fn as_braces(&self) -> Braces {
        Braces {
            inner: Box::new(self.without_exponent()),
            exponent: Some(Box::new(self.get_exponent())),
        }
    }
}

use crate::castable::Castable;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::ParsablePrimitiveAsVariable;

impl Castable for Fraction {
    fn as_math(&self) -> Math {
        Math::Fraction(self.clone())
    }

    fn as_fraction(&self) -> Fraction {
        self.clone()
    }
}

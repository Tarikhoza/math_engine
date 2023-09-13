use crate::castable::Castable;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::ParsablePrimitiveAsVariable;

impl Castable for Polynom {
    fn as_math(&self) -> Math {
        Math::Polynom(self.clone())
    }

    fn as_polynom(&self) -> Polynom {
        self.clone()
    }
}

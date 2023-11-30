use crate::castable::Castable;
use crate::math::algebra::absolute::Absolute;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::root::Root;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::ParsablePrimitiveAsVariable;

impl Castable for Absolute {
    fn as_math(&self) -> Math {
        Math::Absolute(self.clone())
    }
}

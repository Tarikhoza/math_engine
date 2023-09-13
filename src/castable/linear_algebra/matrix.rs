use crate::castable::Castable;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::root::Root;
use crate::math::algebra::variable::Variable;
use crate::math::linear_algebra::vector::Vector;
use crate::math::Math;
use crate::parser::ParsablePrimitiveAsVariable;

impl Castable for Vector {
    fn as_math(&self) -> Math {
        Math::Vector(self.clone())
    }
}

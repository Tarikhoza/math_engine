use crate::castable::Castable;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::root::Root;
use crate::math::algebra::variable::Variable;
use crate::math::calculus::sum::Sum;
use crate::math::Math;
use crate::parser::ParsablePrimitiveAsVariable;

impl Castable for Sum {
    fn as_math(&self) -> Math {
        Math::Sum(self.clone())
    }
}

pub mod algebra;
pub mod calculus;
pub mod linear_algebra;
pub mod math;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::Math;
use crate::parser::ParsablePrimitiveAsVariable;

pub trait Castable {
    fn as_math(&self) -> Math;

    fn as_polynom(&self) -> Polynom {
        Polynom {
            parts: vec![self.as_math().as_polynom_part()],
        }
    }

    fn as_fraction(&self) -> Fraction {
        Fraction {
            whole: None,
            denominator: Box::new(1_i64.as_variable().as_math()),
            numerator: Box::new(self.as_math()),
        }
    }

    fn as_braces(&self) -> Braces {
        Braces {
            math: Box::new(self.as_math()),
            exponent: None,
        }
    }
}

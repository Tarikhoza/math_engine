use crate::math::algebra::fraction::Fraction;
use crate::math::operator::algebra::{Operations as AlgebraOperatons, Operator as AlgebraOperator};
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::Parsable;

impl AlgebraOperatons for Fraction {
    fn addition(&self, other: &Fraction) -> Math {
        todo!();
    }

    fn subtraction(&self, other: &Fraction) -> Math {
        todo!();
    }

    fn multiplication(&self, other: &Fraction) -> Math {
        todo!();
    }

    fn division(&self, other: &Fraction) -> Math {
        todo!();
    }

    fn add(&self, rhs: &Math) -> Math {
        todo!();
    }

    fn sub(&self, rhs: &Math) -> Math {
        todo!();
    }

    fn mul(&self, rhs: &Math) -> Math {
        todo!();
    }

    fn div(&self, rhs: &Math) -> Math {
        todo!();
    }

    fn negative(&self) -> Math {
        todo!();
    }

    fn simplify(&self) -> Math {
        todo!();
    }
    fn substitute(&self, suffix: String, math: Math) -> Math {
        todo!();
    }
}

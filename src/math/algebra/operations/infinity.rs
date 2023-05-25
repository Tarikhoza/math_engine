use crate::math::operator::algebra::{Operations as AlgebraOperatons, Operator as AlgebraOperator};
use crate::math::operator::Operator;
use crate::math::Infinity;
use crate::math::Math;
use crate::parser::Parsable;

impl AlgebraOperatons for Infinity {
    fn addition(&self, other: &Infinity) -> Math {
        todo!();
    }

    fn subtraction(&self, other: &Infinity) -> Math {
        todo!();
    }

    fn multiplication(&self, other: &Infinity) -> Math {
        todo!();
    }

    fn division(&self, other: &Infinity) -> Math {
        //TODO check what to do when infinity is divided
        todo!();
    }

    fn add(&self, rhs: &Math) -> Math {
        //        println!("{}+{}", self.to_tex(), _rhs.to_tex());
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

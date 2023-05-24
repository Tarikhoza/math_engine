use crate::math::operator::algebra::{Operations as AlgebraOperatons, Operator as AlgebraOperator};
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::Parsable;

impl AlgebraOperatons for Infinity {
    fn addition(&self, other: &Infinity) -> Math {
        self.clone()
    }

    fn subtraction(&self, other: &Infinity) -> Math {
        self.clone()
    }

    fn multiplication(&self, other: &Infinity) -> Math {
        if self.minus && other.minus || self.minus && other.minus {
            return Infinity { minus: false };
        } else if self.minus && !other.minus || !self.minus && other.minus {
            return Infinity { minus: true };
        }
    }

    fn division(&self, other: &Infinity) -> Math {}

    fn add(&self, rhs: &Math) -> Math {
        //        println!("{}+{}", self.to_tex(), _rhs.to_tex());
        match rhs {
            Math::Polynom(p) => self.as_polynom().add(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.addition(v),
            _ => todo!(),
        }
    }
    fn sub(&self, rhs: &Math) -> Math {}
    fn mul(&self, rhs: &Math) -> Math {}

    fn div(&self, rhs: &Math) -> Math {}

    fn negative(&self) -> Math {}

    fn simplify(&self) -> Math {
        self.clone()
    }
}

use crate::castable::Castable;
use crate::math::algebra::operations::{
    Operations as AlgebraOperatons, Operator as AlgebraOperator,
};
use crate::math::algebra::polynom::Polynom;
use crate::math::operator::Operator;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::math::Root;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable};

impl Simplifiable for Root {
    fn simplify(&self) -> Math {
        todo!();
    }
}

impl AlgebraOperatons for Root {
    fn add_self(&self, other: &Root) -> Math {
        if self.to_tex() == other.to_tex() {
            Polynom {
                parts: vec![
                    2_i64.as_variable().as_math().as_polynom_part(),
                    Operator::Algebra(AlgebraOperator::InvMulti).as_polynom_part(),
                    self.clone().as_math().as_polynom_part(),
                ],
                #[cfg(feature = "step-tracking")]
                step: None,
            }
            .as_math()
        } else {
            Polynom {
                parts: vec![
                    self.as_math().as_polynom_part(),
                    Operator::Algebra(AlgebraOperator::Addition).as_polynom_part(),
                    other.as_math().as_polynom_part(),
                ],
                #[cfg(feature = "step-tracking")]
                step: None,
            }
            .as_math()
        }
    }

    fn sub_self(&self, other: &Root) -> Math {
        if self.to_tex() == other.to_tex() {
            0_i64.as_variable().as_math()
        } else {
            Polynom {
                parts: vec![
                    self.as_math().as_polynom_part(),
                    Operator::Algebra(AlgebraOperator::Subtraction).as_polynom_part(),
                    other.as_math().as_polynom_part(),
                ],
                #[cfg(feature = "step-tracking")]
                step: None,
            }
            .as_math()
        }
    }

    fn mul_self(&self, other: &Root) -> Math {
        todo!();
    }

    fn div_self(&self, other: &Root) -> Math {
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

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        todo!();
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        todo!();
    }
}

use crate::castable::Castable;
use crate::math::algebra::operations::{
    Operations as AlgebraOperatons, Operator as AlgebraOperator,
};
use crate::math::algebra::polynom::Polynom;
use crate::math::operator::Operator;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::math::Root;
use crate::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable};

impl Simplifiable for Root {
    fn simplify(&self) -> Math {
        todo!();
    }
}

impl AlgebraOperatons for Root {
    fn add_self(&self, other: &Root) -> Math {
        if self.to_tex() == other.to_tex() {
            Math::Polynom(Polynom {
                factors: vec![2_i64.as_variable().as_math(), Math::Root(self.clone())],
                operators: vec![Operator::Algebra(AlgebraOperator::InvMulti)],
                #[cfg(feature = "step-tracking")]
                step: None,
            })
        } else {
            Math::Polynom(Polynom {
                factors: vec![Math::Root(self.clone()), Math::Root(other.clone())],
                operators: vec![Operator::Algebra(AlgebraOperator::Addition)],
                #[cfg(feature = "step-tracking")]
                step: None,
            })
        }
    }

    fn sub_self(&self, other: &Root) -> Math {
        if self.to_tex() == other.to_tex() {
            0_i64.as_variable().as_math()
        } else {
            Math::Polynom(Polynom {
                factors: vec![Math::Root(self.clone()), Math::Root(other.clone())],
                operators: vec![Operator::Algebra(AlgebraOperator::Subtraction)],
                #[cfg(feature = "step-tracking")]
                step: None,
            })
        }
    }

    fn mul_self(&self, other: &Root) -> Math {
        todo!();
        if self.get_base().to_tex() == other.get_base().to_tex() {
            Math::Root(Root {
                math: Box::new(Math::Polynom(Polynom {
                    factors: vec![*self.math.clone(), *other.math.clone()],
                    operators: vec![Operator::Algebra(AlgebraOperator::Multiplication)],
                    #[cfg(feature = "step-tracking")]
                    step: None,
                })),
                base: self.base.clone(),
            })
        } else {
            todo!()
        }
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

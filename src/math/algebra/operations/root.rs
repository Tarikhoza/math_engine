use crate::castable::Castable;
use crate::math::algebra::operations::{
    Operations as AlgebraOperatons, Operator as AlgebraOperator,
};
use crate::math::algebra::polynom::Polynom;
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
    fn add_self(&self, other: &Self) -> Math {
        todo!()
    }

    fn sub_self(&self, other: &Self) -> Math {
        todo!()
    }

    fn div_self(&self, other: &Self) -> Math {
        todo!()
    }

    fn mul_self(&self, other: &Self) -> Math {
        todo!()
    }

    fn substitute(&self, suffix: &str, value: Math) -> Math {
        todo!()
    }

    fn add(&self, other: &Math) -> Math {
        todo!()
    }

    fn sub(&self, other: &Math) -> Math {
        todo!()
    }

    fn div(&self, other: &Math) -> Math {
        todo!()
    }

    fn mul(&self, other: &Math) -> Math {
        todo!()
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        todo!()
    }
}

use crate::math::algebra::operations::Operations as AlgebraOperatons;
use crate::math::simplifiable::Simplifiable;
use crate::math::Infinity;
use crate::math::Math;

impl Simplifiable for Infinity {
    fn simplify(&self) -> Math {
        todo!();
    }
}

impl AlgebraOperatons for Infinity {
    fn add_self(&self, other: &Infinity) -> Math {
        todo!();
    }

    fn sub_self(&self, other: &Infinity) -> Math {
        todo!();
    }

    fn mul_self(&self, other: &Infinity) -> Math {
        todo!();
    }

    fn div_self(&self, other: &Infinity) -> Math {
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

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        todo!();
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        todo!();
    }
}

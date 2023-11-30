use crate::castable::Castable;
use crate::math::algebra::operations::{Operations, Operator as AlgebraOperator};
use crate::math::algebra::polynom::Polynom;
use crate::math::calculus::sum::Sum;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable};

impl Simplifiable for Sum {
    fn simplify(&self) -> Math {
        let mut n = *self.start.clone();
        let end = self.end.clone().add(&1_i64.as_variable().as_math());

        let mut new_poly: Polynom = Polynom { parts: Vec::new() };

        while n.to_tex() != end.to_tex() {
            let i_n = self.inner.substitute(&self.iter, n.clone());
            if new_poly.parts.is_empty() {
                new_poly
                    .parts
                    .push(i_n.as_braces().as_math().as_polynom_part());
            } else {
                new_poly
                    .parts
                    .push(i_n.as_braces().as_math().as_polynom_part());
                new_poly
                    .parts
                    .push(AlgebraOperator::Addition.as_polynom_part());
            }
            n = n.add(&1_i64.as_variable().as_math()).simplify();
        }

        new_poly.as_math()
    }
}
impl Operations for Sum {
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

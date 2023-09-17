use crate::castable::Castable;
use crate::math::algebra::operations::{Operations, Operator as AlgebraOperator};
use crate::math::algebra::polynom::Polynom;
use crate::math::calculus::sum::Sum;
use crate::math::operator::Operator;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable};

impl Simplifiable for Sum {
    fn simplify(&self) -> Math {
        let mut n = *self.begining.clone();
        let end = self.end.clone().add(&1_i64.as_variable().as_math());

        let mut new_poly: Polynom = Polynom { parts: Vec::new() };

        while n.to_tex() != end.to_tex() {
            let i_n = self.math.substitute(&self.iter_suffix, n.clone());
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
                    .push(Operator::Algebra(AlgebraOperator::Addition).as_polynom_part());
            }
            n = n.add(&1_i64.as_variable().as_math()).simplify();
        }

        new_poly.unpack()
    }
}

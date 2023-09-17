use crate::castable::Castable;
use crate::math::algebra::operations::{Operations, Operator as AlgebraOperator};
use crate::math::algebra::polynom::Polynom;
use crate::math::calculus::factorial::Factorial;
use crate::math::operator::Operator;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable};

impl Simplifiable for Factorial {
    fn simplify(&self) -> Math {
        let mut n = 1_i64.as_variable().as_math();
        let end = self.math.clone().add(&1_i64.as_variable().as_math());

        let mut new_poly: Polynom = Polynom { parts: Vec::new() };

        while n.to_tex() != end.to_tex() {
            if new_poly.parts.is_empty() {
                new_poly
                    .parts
                    .push(n.as_braces().as_math().as_polynom_part());
            } else {
                new_poly
                    .parts
                    .push(n.as_braces().as_math().as_polynom_part());
                new_poly
                    .parts
                    .push(Operator::Algebra(AlgebraOperator::Multiplication).as_polynom_part())
            }
            n = n.add(&1_i64.as_variable().as_math()).simplify();
        }

        new_poly.unpack()
    }
}

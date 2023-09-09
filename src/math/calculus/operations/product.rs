use crate::math::algebra::operations::{Operations, Operator as AlgebraOperator};
use crate::math::algebra::polynom::Polynom;
use crate::math::calculus::product::Product;
use crate::math::operator::Operator;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable};

impl Simplifiable for Product {
    fn simplify(&self) -> Math {
        let mut n = *self.begining.clone();
        let end = self.end.clone().add(
            &1_i64
                .parse_math()
                .expect("failed parsing 1 as math for end"),
        );
        let mut factors: Vec<Math> = vec![];

        let mut new_poly: Polynom = Polynom {
            factors: Vec::new(),
            operators: Vec::new(),
        };

        while (n.to_tex() != end.to_tex()) {
            dbg!(self.math.to_tex());
            let i_n = self.math.substitute(&self.iter_suffix, n.clone());
            dbg!(i_n.to_tex());
            if new_poly.factors.is_empty() {
                new_poly.factors.push(i_n.in_brackets());
            } else {
                new_poly.push(
                    i_n.in_brackets(),
                    Operator::Algebra(AlgebraOperator::Multiplication),
                );
            }
            n = n
                .add(&1_i64.parse_math().expect("failed parsing 1 as math"))
                .simplify();
        }

        new_poly.unpack()
    }
}

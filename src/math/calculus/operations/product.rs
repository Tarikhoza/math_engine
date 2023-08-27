use crate::math::algebra::operations::{Operations, Operator as AlgebraOperator};
use crate::math::algebra::polynom::Polynom;
use crate::math::calculus::product::Product;
use crate::math::operator::Operator;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, ParsableGenericsAsVariable};

impl Simplifiable for Product {
    fn simplify(&self) -> Math {
        let mut n = *self.begining.clone();
        let end = self.end.clone().add(
            &1_i64
                .parse_math()
                .expect("failed parsing 1 as math for end"),
        );
        let mut factors: Vec<Math> = vec![];

        while (n.to_tex() != end.to_tex()) {
            dbg!(&self.math.to_tex(), &n.to_tex());
            dbg!(self.math.substitute(&self.iter_suffix, n.clone()).to_tex());
            factors.push(self.math.substitute(&self.iter_suffix, n.clone()));
            n = n
                .add(&1_i64.parse_math().expect("failed parsing 1 as math"))
                .simplify();
        }

        let operators: Vec<Operator> =
            vec![Operator::Algebra(AlgebraOperator::Multiplication); factors.len()];
        Math::Polynom(Polynom {
            factors,
            operators,
            #[cfg(feature = "step-tracking")]
            step: None,
        })
    }
}

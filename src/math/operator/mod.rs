use std::io::Empty;

use crate::math::algebra::operations::Operator as AlgebraOperator;
use crate::solver::step::DetailedOperator;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Operator {
    Algebra(AlgebraOperator),
    Detail(DetailedOperator),
    #[default]
    Empty,
}

impl Operator {
    pub fn morph(&self, other: &Operator) -> Operator {
        // TODO remove panic
        if self == &Operator::Algebra(AlgebraOperator::Division)
            || self == &Operator::Algebra(AlgebraOperator::Multiplication)
            || self == &Operator::Algebra(AlgebraOperator::InvMulti)
            || other == &Operator::Algebra(AlgebraOperator::Division)
            || other == &Operator::Algebra(AlgebraOperator::Multiplication)
            || other == &Operator::Algebra(AlgebraOperator::InvMulti)
        {
            panic!("cannot morph dot operators")
        }
        if self == &Operator::Algebra(AlgebraOperator::Subtraction)
            && other == &Operator::Algebra(AlgebraOperator::Addition)
            || other == &Operator::Algebra(AlgebraOperator::Subtraction)
                && self == &Operator::Algebra(AlgebraOperator::Addition)
        {
            return Operator::Algebra(AlgebraOperator::Subtraction);
        } else if self == &Operator::Empty {
            return other.clone();
        } else if other == &Operator::Empty {
            return self.clone();
        }
        Operator::Algebra(AlgebraOperator::Addition)
    }
}

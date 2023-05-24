pub mod algebra;
use crate::math::operator::algebra::Operator as AlgebraOperator;
use crate::solver::step::DetailedOperator;
use crate::math::Math;
use crate::parser::Parsable;
use std::default;
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Operator {
    Algebra(AlgebraOperator),
    Detail(DetailedOperator),
    #[default] 
    Empty
}


impl Operator{
    pub fn morph(&self, other:Operator) -> Operator{
        if self == &Operator::Algebra(AlgebraOperator::Subtraction) && other == Operator::Algebra(AlgebraOperator::Addition) || 
            other == Operator::Algebra(AlgebraOperator::Subtraction) && self == &Operator::Algebra(AlgebraOperator::Addition){
            return Operator::Algebra(AlgebraOperator::Subtraction)
        }
        Operator::Algebra(AlgebraOperator::Addition)
    }
}




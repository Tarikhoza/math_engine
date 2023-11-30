use crate::math::algebra::operations::Operator as AlgebraOperator;

impl AlgebraOperator {
    pub fn morph(&self, other: AlgebraOperator) -> AlgebraOperator {
        if self == &AlgebraOperator::Subtraction && other == AlgebraOperator::Addition
            || other == AlgebraOperator::Subtraction && self == &AlgebraOperator::Addition
        {
            return AlgebraOperator::Subtraction;
        }
        AlgebraOperator::Addition
    }
}

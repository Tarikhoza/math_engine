use crate::math::algebra::operations::Operations as AlgebraOperations;
use crate::math::Math;

pub trait Simplifiable {
    fn simplify(&self) -> Math;
}

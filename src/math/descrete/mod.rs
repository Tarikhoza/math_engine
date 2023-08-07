pub mod variable;
use crate::math::Variable;
pub trait Descrete {
    fn is_prime(&self) -> bool;
    fn primes_up_to(&self) -> Vec<Variable>;
    fn factorise_prime(&self) -> Vec<Variable>;
    fn lowest_common_denominator(&self, other: Variable) -> Variable;
    fn is_divisable(&self, other: &Variable) -> bool;
}

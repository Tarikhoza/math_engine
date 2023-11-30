pub mod braces;
pub mod fraction;
pub mod infinity;
pub mod polynom;
pub mod root;
pub mod undefined;
pub mod variable;

use crate::castable::Castable;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;

use crate::parser::ParsablePrimitiveAsVariable;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    InvMulti,
    AddSub,
}

pub trait Operations {
    fn add_self(&self, other: &Self) -> Math;
    fn sub_self(&self, other: &Self) -> Math;
    fn div_self(&self, other: &Self) -> Math;
    fn mul_self(&self, other: &Self) -> Math;

    fn negative(&self) -> Math {
        self.mul(&(-1_i64).as_variable().as_math())
    }
    fn substitute(&self, suffix: &str, value: Math) -> Math;

    fn add(&self, other: &Math) -> Math;
    fn sub(&self, other: &Math) -> Math;
    fn div(&self, other: &Math) -> Math;
    fn mul(&self, other: &Math) -> Math;
    fn add_sub(&self, other: &Math) -> (Math, Math) {
        (self.add(other), self.sub(other))
    }

    fn get_all_suffixes(&self) -> Vec<String>;
}

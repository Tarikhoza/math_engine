use crate::math::operator::equation::Operator as EquationOperator;
use crate::math::AlgebraOperations;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    pub factors: Vec<Math>,
    pub operators: Vec<EquationOperator>,
}

impl Equation {
    pub fn normalise_system(&self, other: &Equation) -> Equation {
        let mut new_eq = (*self).clone();
        let mut sum = new_eq.sum();
        while new_eq.factors.len() < other.factors.len() {
            new_eq.factors.push(sum.clone());
            new_eq.operators.push(EquationOperator::Equal);
        }
        new_eq
    }

    pub fn sum(&self) -> Math {
        let mut new_eq = (*self).clone();
        let mut sum = new_eq.factors[0].clone();
        for i in self.factors.iter().skip(1) {
            sum = sum.add(&i.negative());
        }
        sum
    }

    pub fn all_to_left(&self) -> Math {
        format!("{}=0", self.sum().to_tex())
            .parse_math()
            .expect("error parsing math for all_to_left")
    }

    pub fn decouple(&self) -> Math {
        self.factors
            .last()
            .expect("error decoupleing equation")
            .clone()
    }
}

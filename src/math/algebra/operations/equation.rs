use crate::math::algebra::equation::Equation;
//use crate::math::algebra::undefined::Undefined;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperators,
};

use crate::math::operator::equation::Operator as EquationOperator;
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, Parser};

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

impl AlgebraOperations for Equation {
    fn addition(&self, other: &Equation) -> Math {
        let mut factors: Vec<Math> = vec![];
        let first = self.normalise_system(other);
        let second = other.normalise_system(self);

        let mut operators: Vec<EquationOperator> = self.operators.clone();

        for i in 0..first.factors.len() {
            factors.push(first.factors[i].add(&second.factors[i]));
        }
        Math::Equation(Equation { factors, operators })
    }
    fn subtraction(&self, other: &Equation) -> Math {
        let mut factors: Vec<Math> = vec![];
        let first = self.normalise_system(other);
        let second = other.normalise_system(self);

        let mut operators: Vec<EquationOperator> = self.operators.clone();

        for i in 0..first.factors.len() {
            factors.push(first.factors[i].sub(&second.factors[i]));
        }
        Math::Equation(Equation { factors, operators })
    }

    fn multiplication(&self, other: &Equation) -> Math {
        todo!();
    }

    fn division(&self, _other: &Equation) -> Math {
        todo!();
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Equation(e) => self.addition(e),
            other => {
                let mut new_eq = self.clone();
                for i in new_eq.factors.iter_mut() {
                    i.add(other);
                }
                return Math::Equation(new_eq);
            }
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Equation(e) => self.subtraction(e),
            other => {
                let mut new_eq = self.clone();
                for i in new_eq.factors.iter_mut() {
                    i.sub(other);
                }
                return Math::Equation(new_eq);
            }
        }
    }
    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Equation(e) => todo!(),
            other => {
                let mut new_eq = self.clone();
                for i in new_eq.factors.iter_mut() {
                    i.mul(other);
                }
                return Math::Equation(new_eq);
            }
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Equation(e) => todo!(),
            other => {
                let mut new_eq = self.clone();
                for i in new_eq.factors.iter_mut() {
                    i.mul(other);
                }
                return Math::Equation(new_eq);
            }
        }
    }

    fn negative(&self) -> Math {
        self.mul(&"-1".parse_math().expect("Error parsing -1 as math"))
    }

    fn simplify(&self) -> Math {
        let mut new_eq = self.clone();
        for i in new_eq.factors.iter_mut() {
            i.simplify();
        }
        return Math::Equation(new_eq);
    }

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        let mut new_eq = self.clone();
        for i in new_eq.factors.iter_mut() {
            i.substitute(suffix, math.clone());
        }
        return Math::Equation(new_eq);
    }
}

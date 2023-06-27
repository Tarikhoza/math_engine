pub mod algebra;
pub mod descrete;
pub mod geometry;
pub mod linear_algebra;
pub mod operator;
pub mod trigonometry;

use fancy_regex::Regex;
use std::default;
use std::ops;

use crate::parser::{Parsable, Parser};

use crate::math::algebra::absolute::Absolute;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::equation::Equation;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::function::Function;
use crate::math::algebra::infinity::Infinity;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::root::Root;
use crate::math::algebra::undefined::Undefined;
use crate::math::algebra::variable::Variable;
use crate::math::linear_algebra::matrix::Matrix;
use crate::math::linear_algebra::vector::Vector;

use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};

use crate::math::operator::Operator;

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

#[derive(Debug, Clone, PartialEq)]
pub enum Math {
    Variable(Variable),
    Equation(Equation),
    Polynom(Polynom),
    Fraction(Fraction),
    Braces(Braces),
    Function(Function),
    Absolute(Absolute),
    Vector(Vector),
    Root(Root),
    Matrix(Matrix),
    Infinity(Infinity),
    Undefined(Undefined),
    Operator(Operator),
}

impl Default for Math {
    fn default() -> Self {
        Math::Variable(Variable::default())
    }
}

impl Math {
    pub fn sort_score(&self) -> u32 {
        match self {
            Math::Variable(v) => v.sort_score(),
            _ => u32::MAX,
        }
    }

    pub fn split_operator(&self) -> (Operator, Math) {
        match self {
            Math::Variable(s) => s.split_operator(),
            _ => (Operator::Algebra(AlgebraOperator::Addition), self.clone()),
        }
    }

    pub fn morph_operator(pair: (&Operator, &Math)) -> Math {
        match pair.0 {
            Operator::Algebra(AlgebraOperator::Subtraction) => pair.1.negative(),
            _ => pair.1.clone(),
        }
    }

    pub fn add_sub_change(&self, other: &Math) -> bool {
        let before = format!("{}+{}", self.to_tex(), other.to_tex());
        let after = (self.add(other)).to_tex();
        after != before
    }

    pub fn add_sub_base(&self) -> String {
        match &self {
            Math::Variable(v) => v.add_sub_base(),
            Math::Fraction(f) => format!(
                "\\frac{{{}}}{{{}}}",
                f.numerator.add_sub_base(),
                f.denominator.add_sub_base()
            ),
            _ => String::new(),
        }
    }

    #[cfg(feature = "step-tracking")]
    pub fn get_step(&self) -> Step {
        match self {
            Math::Variable(v) => v.step.clone().unwrap_or_default(),
            Math::Polynom(p) => p.step.clone().unwrap_or(
                Step::step(
                    self.clone(),
                    None,
                    Operator::Detail(DetailedOperator::Nothing),
                    String::from("Nothing to do"),
                )
                .unwrap_or_default(),
            ),
            s => Step::step(
                self.clone(),
                None,
                Operator::Detail(DetailedOperator::Nothing),
                format!("Not implemented yet for type: {}", stringify!(s)),
            )
            .unwrap_or_default(),
        }
    }

    pub fn as_polynom(&self) -> Polynom {
        match self {
            Math::Polynom(s) => s.clone(),
            _ => Polynom {
                factors: vec![self.clone()],
                operators: vec![],
                #[cfg(feature = "step-tracking")]
                step: None,
            },
        }
    }
}

fn or_zero(first: &Math, second: &Math) -> bool {
    if first.clone().to_tex() == "0" || second.clone().to_tex() == "0" {
        return true;
    }
    false
}

fn non_zero(first: &Math, second: &Math) -> Math {
    if first.to_tex() != "0" {
        return first.clone();
    }
    second.clone()
}

impl AlgebraOperations for Math {
    fn addition(&self, other: &Math) -> Math {
        self.add(other)
    }
    fn subtraction(&self, other: &Math) -> Math {
        self.sub(other)
    }
    fn multiplication(&self, other: &Math) -> Math {
        self.mul(other)
    }
    fn division(&self, other: &Math) -> Math {
        self.div(other)
    }
    fn add(&self, rhs: &Math) -> Math {
        if or_zero(self, rhs) {
            return non_zero(self, rhs);
        }
        match self {
            Math::Polynom(p) => p.add(rhs),
            Math::Variable(v) => v.add(rhs),
            Math::Braces(b) => b.add(rhs),
            Math::Fraction(f) => f.add(rhs),
            Math::Equation(e) => e.add(rhs),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn sub(&self, rhs: &Math) -> Math {
        if or_zero(self, rhs) {
            return non_zero(self, rhs);
        }
        match self {
            Math::Polynom(p) => p.sub(rhs),
            Math::Variable(v) => v.sub(rhs),
            Math::Braces(b) => b.sub(rhs),
            Math::Fraction(f) => f.sub(rhs),
            Math::Equation(e) => e.sub(rhs),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn mul(&self, rhs: &Math) -> Math {
        match self {
            Math::Polynom(p) => p.mul(rhs),
            Math::Variable(v) => v.mul(rhs),
            Math::Braces(b) => b.mul(rhs),
            Math::Fraction(f) => f.mul(rhs),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn div(&self, rhs: &Math) -> Math {
        match self {
            Math::Polynom(p) => p.div(rhs),
            Math::Variable(v) => v.div(rhs),
            Math::Fraction(f) => f.div(rhs),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn simplify(&self) -> Math {
        match self {
            Math::Polynom(p) => p.simplify(),
            Math::Braces(b) => b.math.simplify(),
            Math::Root(r) => r.take_root(),
            Math::Absolute(a) => a.simplify(),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            s => s.clone(),
        }
    }
    fn negative(&self) -> Math {
        match self {
            Math::Polynom(p) => p.negative(),
            Math::Braces(b) => b.math.negative(),
            Math::Variable(v) => v.negative(),
            Math::Equation(e) => e.negative(),
            Math::Fraction(f) => f.negative(),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            s => s.clone(),
        }
    }
    fn substitute(&self, suffix: &str, math: Math) -> Math {
        match self {
            Math::Variable(v) => v.substitute(suffix, math),
            Math::Polynom(p) => p.substitute(suffix, math),
            Math::Braces(b) => b.substitute(suffix, math),
            Math::Equation(e) => e.substitute(suffix, math),
            s => todo!(),
        }
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        match self {
            Math::Variable(v) => v.get_all_suffixes(),
            Math::Polynom(p) => p.get_all_suffixes(),
            Math::Braces(b) => b.get_all_suffixes(),
            Math::Equation(e) => e.get_all_suffixes(),
            Math::Fraction(f) => f.get_all_suffixes(),
            _ => todo!(),
        }
    }
}

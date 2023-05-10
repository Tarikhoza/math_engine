pub mod algebra;
pub mod linear_algebra;
pub mod operator;

use fancy_regex::Regex;
use std::default;
use std::ops;

use crate::parser::{Parsable, Parser};

use crate::math::algebra::braces::Braces;
use crate::math::algebra::infinity::Infinity;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::variable::Variable;
use crate::math::algebra::undefined::Undefined;
use crate::math::linear_algebra::matrix::Matrix;
use crate::math::linear_algebra::vector::Vector;

use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};

use crate::math::operator::Operator;

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

#[derive(Debug, Clone)]
pub enum Math {
    Variable(Variable),
    Polynom(Polynom),
    Braces(Braces),
    Vector(Vector),
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
            _ => String::new(),
        }
    }

    pub fn map_value(&self, suffix: &str, math: Math) -> Math {
        match self {
            Math::Variable(v) => v.map_value(suffix, math),
            Math::Polynom(p) => p.map_value(suffix, math),
            //            Math::Equation(e) => e.map_value(suffix, math),
            s => todo!(),
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
                .unwrap(),
            ),
            s => Step::step(
                    self.clone(),
                    None,
                    Operator::Detail(DetailedOperator::Nothing),
                    format!("Not implemented yet for type: {}", stringify!(s)),
                ).unwrap()

        }
    }

    pub fn as_polynom(&self) -> Polynom {
        match self {
            Math::Variable(s) => Polynom {
                factors: vec![self.clone()],
                operators: vec![],
                #[cfg(feature = "step-tracking")]
                step: None,
            },
            Math::Braces(s) => Polynom {
                factors: vec![self.clone()],
                operators: vec![],
                #[cfg(feature = "step-tracking")]
                step: None,
            },
            Math::Undefined(s) => Polynom {
               factors: vec![self.clone()],
               operators: vec![],
               #[cfg(feature = "step-tracking")]
               step: None,
            },
            Math::Infinity(s) => Polynom {
               factors: vec![self.clone()],
               operators: vec![],
               #[cfg(feature = "step-tracking")]
               step: None,
           },
            Math::Polynom(s) => s.clone(),
            _ => todo!(),
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
            //            Math::Fraction(f) => f.add(rhs),
            //           Math::Undefined(u) => Math::Undefined(Undefined {}),
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
            //            Math::Fraction(f) => f.sub(rhs),
            //          Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn mul(&self, rhs: &Math) -> Math {
        match self {
            Math::Polynom(p) => p.mul(rhs),
            Math::Variable(v) => v.mul(rhs),
            Math::Braces(b) => b.mul(rhs),
            //            Math::Fraction(f) => f * rhs,
            //         Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn div(&self, rhs: &Math) -> Math {
        match self {
            Math::Polynom(p) => p.div(rhs),
            //   Math::Braces(b)   => b*_rhs,
            Math::Variable(v) => v.div(rhs),
            //            Math::Fraction(f) => f.div(rhs),
            //            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn simplify(&self) -> Math {
        match self {
            Math::Polynom(p) => p.simplify(),
            Math::Braces(b) => b.math.simplify(),
            s => s.clone(),
        }
    }
    fn negative(&self) -> Math {
        match self {
            Math::Polynom(p) => p.negative(),
            Math::Braces(b) => b.math.negative(),
            Math::Variable(v) => v.negative(),
            s => s.clone(),
        }
    }
}

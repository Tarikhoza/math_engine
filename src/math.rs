use std::ops;

use fancy_regex::Regex;

use crate::braces::Braces;
use crate::equation::Equation;
use crate::fraction::Fraction;
use crate::matrix::Matrix;
use crate::operators::Operators;
use crate::parser::{Parsable, Parser};
use crate::polynom::Polynom;
use crate::undefined::Undefined;
use crate::variable::Variable;
use crate::vector::Vector;

#[derive(Debug, Clone)]
pub enum Math {
    Variable(Variable),
    Polynom(Polynom),
    Braces(Braces),
    Fraction(Fraction),
    Vector(Vector),
    Matrix(Matrix),
    Undefined(Undefined),
    Equation(Equation),
    Operators(Operators),
}

pub trait BasicOperations:
    ops::Add<Math> + ops::Sub<Math> + ops::Mul<Math> + ops::Div<Math>
{
    fn addition(&self, other: Self) -> Math;
    fn subtraction(&self, other: Self) -> Math;
    fn division(&self, other: Self) -> Math;
    fn multiplication(&self, other: Self) -> Math;
    fn simplify(&self) -> Math;
    fn negative(&self) -> Math;
}

impl Math {
    #[must_use]
    pub fn simplify(&self) -> Math {
        match self {
            Math::Polynom(p) => p.simplify(),
            Math::Braces(b) => b.math.simplify(),
            s => s.clone(),
        }
    }

    #[must_use]
    pub fn sort_score(&self) -> u32 {
        match self {
            Math::Variable(v) => v.sort_score(),
            _ => u32::MAX,
        }
    }

    #[must_use]
    pub fn negative(&self) -> Math {
        match self {
            Math::Variable(s) => s.negative(),
            Math::Polynom(s) => s.negative(),
            Math::Braces(s) => s.negative(),
            s => s.clone(),
        }
    }

    #[must_use]
    pub fn split_operator(&self) -> (Operators, Math) {
        match self {
            Math::Variable(s) => s.split_operator(),
            _ => (Operators::Addition, self.clone()),
        }
    }

    #[must_use]
    pub fn morph_operator(pair: (&Operators, &Math)) -> Math {
        match pair.0 {
            Operators::Subtraction => pair.1.negative(),
            _ => pair.1.clone(),
        }
    }

    #[must_use]
    pub fn operators_to_string(op: &Operators) -> String {
        op.to_tex()
    }

    #[must_use]
    pub fn string_to_operators(op: String) -> Operators {
        match op {
            x if x == *"+" => Operators::Addition,
            x if x == *"-" => Operators::Subtraction,
            x if x == *"*" => Operators::Multiplication,
            x if x == *"/" => Operators::Division,
            x if x == *" " => Operators::InvMulti,
            String { .. } => todo!(),
        }
    }

    #[must_use]
    pub fn add_sub_change(&self, other: &Math) -> bool {
        let before = format!("{}+{}", self.to_tex(), other.to_tex());
        let after = (self.clone() + other.clone()).to_tex();
        after != before
    }

    #[must_use]
    pub fn add_sub_bases(&self) -> Vec<String> {
        match &self {
            Math::Variable(v) => v.add_sub_bases(),
            _ => vec![String::new()],
        }
    }
    #[must_use]
    pub fn map_value(&self, suffix: &str, math: Math) -> Math {
        match self {
            Math::Variable(v) => v.map_value(suffix, math),
            Math::Polynom(p) => p.map_value(suffix, math),
            //            Math::Equation(e) => e.map_value(suffix, math),
            s => todo!(),
        }
    }
}

fn or_zero(first: &Math, second: &Math) -> bool {
    if first.clone().to_tex() == "0" || second.clone().to_tex() == "0" {
        return true;
    }
    false
}

fn non_zero(first: Math, second: Math) -> Math {
    if first.clone().to_tex() != "0" {
        return first;
    }
    return second;
}

impl ops::Add<Math> for Math {
    type Output = Math;
    fn add(self, rhs: Math) -> Math {
        if or_zero(&self, &rhs) {
            return non_zero(self, rhs);
        }
        match self {
            Math::Polynom(p) => p + rhs,
            Math::Variable(v) => v + rhs,
            Math::Braces(b) => b + rhs,
            Math::Fraction(f) => f + rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Math {
    type Output = Math;
    fn sub(self, rhs: Math) -> Math {
        if or_zero(&self, &rhs) {
            return non_zero(self, rhs);
        }
        match self {
            Math::Polynom(p) => p - rhs,
            Math::Variable(v) => v - rhs,
            Math::Braces(b) => b - rhs,
            Math::Fraction(f) => f - rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Math {
    type Output = Math;
    fn mul(self, rhs: Math) -> Math {
        match self {
            Math::Polynom(p) => p * rhs,
            Math::Variable(v) => v * rhs,
            Math::Braces(b) => b * rhs,
            Math::Fraction(f) => f * rhs,
            Math::Undefined(u) => Math::Undefined(u),
            Math::Operators(_) => todo!(),
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Math {
    type Output = Math;
    fn div(self, rhs: Math) -> Math {
        match self {
            //   Math::Polynom(p)  => p*_rhs,
            //   Math::Braces(b)   => b*_rhs,
            Math::Variable(v) => v / rhs,
            Math::Fraction(f) => f / rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

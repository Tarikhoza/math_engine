pub mod algebra;
pub mod calculus;
pub mod descrete;
pub mod geometry;
pub mod linear_algebra;
pub mod operator;
pub mod simplifiable;
pub mod trigonometry;

use crate::math::algebra::absolute::Absolute;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::function::Function;
use crate::math::algebra::infinity::Infinity;
use crate::math::algebra::operations::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::root::Root;
use crate::math::algebra::undefined::Undefined;
use crate::math::algebra::variable::Variable;
use crate::math::calculus::product::Product;
use crate::math::calculus::sum::Sum;
use crate::math::linear_algebra::matrix::Matrix;
use crate::math::linear_algebra::vector::Vector;
use crate::math::operator::Operator;
use crate::math::simplifiable::Simplifiable;
use crate::parser::{Parsable, ParsableGenerics, ParsableGenericsAsVariable, Parser};

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

use fancy_regex::Regex;
use std::default;
use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub enum Math {
    //Algebra
    Variable(Variable),
    Polynom(Polynom),
    Fraction(Fraction),
    Braces(Braces),
    Function(Function),
    Absolute(Absolute),
    Root(Root),
    Infinity(Infinity),
    Undefined(Undefined),
    //Linear Algebra
    Matrix(Matrix),
    Vector(Vector),
    //Calculus
    Sum(Sum),
    Product(Product),
}

impl Default for Math {
    fn default() -> Self {
        Math::Variable(Variable::default())
    }
}

impl Math {
    pub fn sorting_score(&self) -> i64 {
        match self {
            Math::Variable(v) => v.sorting_score(),
            _ => i64::MAX,
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

    pub fn equal_bruteforce(&self, other: Math) -> bool {
        let mut suffixes = self.get_all_suffixes();
        suffixes.extend(other.get_all_suffixes());
        suffixes.sort();
        suffixes.dedup();
        let mut rounds = 0;
        let mut eq = 0;
        for i in (-10000..10000).step_by(30) {
            rounds += 1;
            let mut new_a = self.clone();
            let mut new_b = other.clone();

            //       let mut substitutions: Vec<String> = vec![];

            for s in 0..suffixes.len() {
                new_a = new_a.substitute(
                    suffixes[s].as_ref(),
                    (s as i64 + i)
                        .parse_math()
                        .expect("failed parsing math for equal_bruteforce"),
                );
                new_b = new_b.substitute(
                    suffixes[s].as_ref(),
                    (s as i64 + i)
                        .parse_math()
                        .expect("failed parsing math for equal_bruteforce"),
                );
                //            substitutions.push(format!("{} = {}", suffixes[s], s as i64 + i));
            }
            //TODO remove reparsing
            if new_a
                .to_tex()
                .parse_math()
                .expect("failed parsing math for equal_bruteforce")
                .simplify()
                .to_tex()
                == new_b
                    .to_tex()
                    .parse_math()
                    .expect("failed parsing math for equal_bruteforce")
                    .simplify()
                    .to_tex()
            {
                eq += 1;
            }
        }
        eq == rounds
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

impl Simplifiable for Math {
    fn simplify(&self) -> Math {
        //TODO recursive simplify until no change
        match self {
            Math::Variable(v) => v.simplify(),
            Math::Polynom(p) => p.simplify(),
            Math::Braces(b) => b.simplify(),
            Math::Root(r) => Math::Root(r.clone()),
            Math::Absolute(a) => a.simplify(),
            Math::Fraction(a) => a.simplify(),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            Math::Infinity(i) => Math::Infinity(i.clone()),
            Math::Sum(s) => s.simplify(),
            Math::Product(p) => p.simplify(),
            _ => todo!(),
        }
    }
}

impl AlgebraOperations for Math {
    fn add_self(&self, other: &Math) -> Math {
        self.add(other)
    }

    fn sub_self(&self, other: &Math) -> Math {
        self.sub(other)
    }

    fn mul_self(&self, other: &Math) -> Math {
        self.mul(other)
    }

    fn div_self(&self, other: &Math) -> Math {
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
            Math::Infinity(i) => i.add(rhs),
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
            Math::Infinity(i) => i.sub(rhs),
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
            Math::Infinity(i) => i.mul(rhs),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match self {
            Math::Polynom(p) => p.div(rhs),
            Math::Variable(v) => v.div(rhs),
            Math::Fraction(f) => f.div(rhs),
            Math::Infinity(i) => i.div(rhs),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn negative(&self) -> Math {
        match self {
            Math::Polynom(p) => p.negative(),
            Math::Braces(b) => b.negative(),
            Math::Variable(v) => v.negative(),
            Math::Fraction(f) => f.negative(),
            //Math::Root(r) => r.negative(),
            //Math::Absolute(a) => a.negative(),
            //Math::Vector(v) => v.negative(),
            //Math::Matrix(m) => m.negative(),
            //Math::Undefined(u) => u.negative(),
            Math::Infinity(i) => i.negative(),
            _ => todo!(),
        }
    }

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        match self {
            Math::Variable(v) => v.substitute(suffix, math).as_polynom().unpack(),
            Math::Polynom(p) => p.substitute(suffix, math).as_polynom().unpack(),
            Math::Braces(b) => b.substitute(suffix, math).as_polynom().unpack(),
            Math::Fraction(f) => f.substitute(suffix, math).as_polynom().unpack(),
            Math::Infinity(i) => Math::Infinity(i.clone()).as_polynom().unpack(),
            //Math::Root(r) => r.substitute(suffix, math),
            //Math::Absolute(a) => a.substitute(suffix, math),
            //Math::Vector(v) => v.substitute(suffix, math),
            //Math::Matrix(m) => m.substitute(suffix, math),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        match self {
            Math::Variable(v) => v.get_all_suffixes(),
            Math::Polynom(p) => p.get_all_suffixes(),
            Math::Braces(b) => b.get_all_suffixes(),
            Math::Fraction(f) => f.get_all_suffixes(),
            _ => todo!(),
        }
    }
}

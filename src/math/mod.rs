pub mod algebra;
pub mod calculus;
pub mod descrete;
pub mod linear_algebra;
pub mod operator;
pub mod simplifiable;

use crate::castable::Castable;
use crate::logging::env_info;
use crate::math::algebra::absolute::Absolute;
use crate::math::algebra::braces::Braces;
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
use crate::math::calculus::factorial::Factorial;
use crate::math::calculus::product::Product;
use crate::math::calculus::sum::Sum;
use crate::math::linear_algebra::matrix::Matrix;
use crate::math::linear_algebra::vector::Vector;
use crate::math::simplifiable::Simplifiable;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable};

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
    Factorial(Factorial),
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

    pub fn split_operator(&self) -> (AlgebraOperator, Math) {
        match self {
            Math::Variable(s) => s.split_operator(),
            _ => (AlgebraOperator::Addition, self.clone()),
        }
    }

    pub fn morph_operator(pair: (&AlgebraOperator, &Math)) -> Math {
        match pair.0 {
            AlgebraOperator::Subtraction => pair.1.negative(),
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

    pub fn equal_bruteforce(&self, other: Math) -> bool {
        let mut suffixes = self.get_all_suffixes();
        suffixes.extend(other.get_all_suffixes());
        suffixes.sort();
        suffixes.dedup();
        for i in (-10000..10000).step_by(30) {
            let mut new_a = self.clone();
            let mut new_b = other.clone();
            for s in 0..suffixes.len() {
                new_a = new_a.substitute(&suffixes[s], (s as i64 + i).as_variable().as_math());
                new_b = new_b.substitute(&suffixes[s], (s as i64 + i).as_variable().as_math());
            }
            if new_a.simplify().to_tex() != new_b.simplify().to_tex() {
                return false;
            }
        }
        true
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
        env_info("operations", format!("math simplify {}", self.to_tex()));
        match self {
            Math::Variable(v) => v.simplify(),
            Math::Polynom(p) => p.simplify(),
            Math::Braces(b) => b.simplify(),
            Math::Root(r) => Math::Root(r.clone()),
            Math::Absolute(a) => a.simplify(),
            Math::Fraction(a) => a.simplify(),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            Math::Infinity(i) => Math::Infinity(i.clone()),
            Math::Sum(s) => s.simplify(),
            Math::Product(p) => p.simplify(),
            Math::Factorial(f) => f.simplify(),
            Math::Function(f) => {
                return Math::Function(f.clone());
            }
            Math::Matrix(f) => {
                return Math::Matrix(f.clone());
            }
            Math::Vector(f) => {
                return Math::Vector(f.clone());
            }
        }
    }
}

impl AlgebraOperations for Math {
    fn add_self(&self, other: &Math) -> Math {
        env_info("operations", format!("math add_self {}", self.to_tex()));
        self.add(other)
    }

    fn sub_self(&self, other: &Math) -> Math {
        env_info("operations", format!("math sub_self {}", self.to_tex()));
        self.sub(other)
    }

    fn mul_self(&self, other: &Math) -> Math {
        env_info("operations", format!("math mul_self {}", self.to_tex()));
        self.mul(other)
    }

    fn div_self(&self, other: &Math) -> Math {
        env_info("operations", format!("math div_self {}", self.to_tex()));
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
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
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
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
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
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match self {
            Math::Polynom(p) => p.div(rhs),
            Math::Variable(v) => v.div(rhs),
            Math::Fraction(f) => f.div(rhs),
            Math::Infinity(i) => i.div(rhs),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn negative(&self) -> Math {
        env_info("operations", format!("math negative {}", self.to_tex()));
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
        env_info("operations", format!("math substitute {}", self.to_tex()));
        match self {
            Math::Variable(v) => v.substitute(suffix, math).as_polynom().as_math(),
            Math::Polynom(p) => p.substitute(suffix, math).as_polynom().as_math(),
            Math::Braces(b) => b.substitute(suffix, math).as_polynom().as_math(),
            Math::Fraction(f) => f.substitute(suffix, math).as_polynom().as_math(),
            Math::Infinity(i) => Math::Infinity(i.clone()).as_polynom().as_math(),
            //Math::Root(r) => r.substitute(suffix, math),
            //Math::Absolute(a) => a.substitute(suffix, math),
            //Math::Vector(v) => v.substitute(suffix, math),
            //Math::Matrix(m) => m.substitute(suffix, math),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        let suf = match self {
            Math::Variable(v) => v.get_all_suffixes(),
            Math::Polynom(p) => p.get_all_suffixes(),
            Math::Braces(b) => b.get_all_suffixes(),
            Math::Fraction(f) => f.get_all_suffixes(),
            _ => Vec::new(),
        };

        env_info(
            "helper",
            format!("get_all_suffixes {} {:#?}", self.to_tex(), suf),
        );
        suf
    }
}

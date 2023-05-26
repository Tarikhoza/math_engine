use fancy_regex::Regex;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::default;

use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};

use crate::math::operator::Operator;

use crate::math::algebra::braces::Braces;
use crate::math::algebra::polynom::Polynom;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, Parser};

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

#[derive(Debug, Clone, Default)]
pub struct Variable {
    pub value: Decimal,
    pub suffix: String,
    pub exponent: Option<Box<Math>>,
    #[cfg(feature = "step-tracking")]
    pub step: Option<Step>,
}

fn ascii_score(s: &str) -> u32 {
    let mut score = 0;
    for (i, c) in s.chars().enumerate() {
        score = c.to_digit(10).unwrap_or(1000) / (i + 1) as u32;
    }
    score
}

impl Variable {
    pub fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1),
                suffix: String::new(),
                exponent: None,
                #[cfg(feature = "step-tracking")]
                step: None,
            }),
            Some(e) => *e.clone(),
        }
    }

    pub fn apply_exponent(&self) -> Math {
        todo!()
    }
    pub fn as_polynom(&self) -> Polynom {
        Polynom {
            factors: vec![Math::Variable(self.clone())],
            operators: vec![],
            #[cfg(feature = "step-tracking")]
            step: None,
        }
    }

    pub fn split_operator(&self) -> (Operator, Math) {
        if self.value < dec!(0) {
            return (
                Operator::Algebra(AlgebraOperator::Subtraction),
                self.negative(),
            );
        }
        (
            Operator::Algebra(AlgebraOperator::Addition),
            Math::Variable(self.clone()),
        )
    }

    pub fn sort_score(&self) -> u32 {
        u32::MAX - (ascii_score(&self.suffix) + ascii_score(&self.get_exponent().to_tex()))
    }

    pub fn add_sub_base(&self) -> String {
        let mut x = self.clone();
        x.value = dec!(1.0);
        x.to_tex()
    }

    pub fn lowest_common_denominator(&self, other: Variable) -> Variable {
        //TODO use generators for generating primes
        for i in self.factorise() {
            for j in other.factorise() {
                if i.value == j.value {
                    return i;
                }
            }
        }
        return 1.as_variable().expect("converting usize failed");
    }

    //descrete math functions
    pub fn is_prime(&self) -> bool {
        if self.value <= dec!(1) {
            return false;
        }
        let n = self.value.to_i64().expect("converting to i64 failed");
        for i in 2..=(n as f64).sqrt() as i64 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    pub fn primes_up_to(&self) -> Vec<Variable> {
        let mut primes: Vec<Variable> = vec![];
        for i in 2..self.value.to_i64().expect("converting to i64 failed") {
            let x = (i as usize).as_variable().expect("failed parsing math");
            if x.is_prime() {
                primes.push(x);
            }
        }
        primes
    }
    pub fn factorise(&self) -> Vec<Variable> {
        let mut math = self.clone();
        if math.is_prime() {
            return vec![math];
        }
        let mut factorised: Vec<Variable> = vec![];
        for i in math.primes_up_to() {
            if math.value.is_zero() {
                break;
            }
            while ((math.value % i.value).is_zero()) {
                math.value /= i.value;
                factorised.push(i.clone());
            }
        }
        factorised
    }
}

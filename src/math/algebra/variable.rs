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
use crate::parser::{Parsable, Parser};

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

    pub fn map_value(&self, suffix: &str, math: Math) -> Math {
        if self.suffix == suffix {
            Math::Polynom(Polynom {
                factors: vec![
                    Math::Variable(Variable {
                        value: self.value,
                        suffix: String::new(),
                        exponent: None,
                        #[cfg(feature = "step-tracking")]
                        step: Step::step(
                            Math::Variable(self.clone()),
                            Some(math.clone()),
                            Operator::Detail(DetailedOperator::MapTo),
                            String::from("Map to"),
                        ),
                    }),
                    Math::Braces(Braces {
                        math: Box::new(math),
                        exponent: Some(Box::new(self.get_exponent())),
                    }),
                ],
                operators: vec![Operator::Algebra(AlgebraOperator::Multiplication)],
                #[cfg(feature = "step-tracking")]
                step: None,
            })
        } else {
            Math::Variable(self.clone())
        }
    }
}

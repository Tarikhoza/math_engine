use fancy_regex::Regex;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::braces::Braces;
use crate::math::{BasicOperations, Math};
use crate::operators::Operators;
use crate::parser::{Parsable, Parser};
use crate::polynom::Polynom;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Decimal,
    pub suffix: String,
    pub exponent: Option<Box<Math>>,
}

fn ascii_score(s: &str) -> u32 {
    let mut score = 0;
    for (i, c) in s.chars().enumerate() {
        score = c.to_digit(10).unwrap_or(1000) / (i + 1) as u32;
    }
    score
}

impl Variable {
    #[must_use]
    pub fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1),
                suffix: String::new(),
                exponent: None,
            }),
            Some(e) => *e.clone(),
        }
    }

    #[must_use]
    pub fn apply_exponent(&self) -> Math {
        todo!()
    }

    #[must_use]
    pub fn as_polynom(&self) -> Polynom {
        Polynom {
            factors: vec![Math::Variable(self.clone())],
            operators: vec![],
        }
    }

    #[must_use]
    pub fn split_operator(&self) -> (Operators, Math) {
        if self.value < dec!(0) {
            return (Operators::Subtraction, self.negative());
        }
        (Operators::Addition, Math::Variable(self.clone()))
    }

    #[must_use]
    pub fn sort_score(&self) -> u32 {
        u32::MAX - (ascii_score(&self.suffix) + ascii_score(&self.get_exponent().to_tex()))
    }

    #[must_use]
    pub fn add_sub_bases(&self) -> Vec<String> {
        let mut x = self.clone();
        x.value = dec!(1.0);
        vec![x.to_tex()]
    }

    pub fn map_value(&self, suffix: &str, math: Math) -> Math {
        if self.suffix == suffix {
            Math::Polynom(Polynom {
                factors: vec![
                    Math::Variable(Variable {
                        value: self.value,
                        suffix: String::new(),
                        exponent: None,
                    }),
                    Math::Braces(Braces {
                        math: Box::new(math),
                        exponent: Some(Box::new(self.get_exponent())),
                    }),
                ],
                operators: vec![Operators::Multiplication],
            })
        } else {
            Math::Variable(self.clone())
        }
    }
}

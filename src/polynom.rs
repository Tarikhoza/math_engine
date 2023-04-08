use crate::fraction::Fraction;
use crate::math::{BasicOperations, Math};
use crate::operators::Operators;
use crate::parser::Parsable;
use crate::variable::Variable;
use crate::vector::Vector;
use rust_decimal_macros::dec;

#[derive(Debug, Clone)]
pub struct Polynom {
    pub factors: Vec<Math>,
    pub operators: Vec<Operators>,
}

impl Polynom {
    pub fn unpack(&self) -> Math {
        if self.factors.len() == 1 {
            return self.factors[0].clone();
        }
        Math::Polynom(self.clone())
    }

    #[must_use]
    pub fn to_vector(&self) -> Vector {
        let mut factors: Vec<Math> = self
            .operators
            .iter()
            .zip(self.factors.iter().skip(1))
            .map(|m| Math::morph_operator(m))
            .collect();

        factors.insert(0, self.factors.get(0).unwrap().to_owned());
        Vector { factors }
    }
    pub fn to_fraction(&self) -> Fraction {
        Fraction {
            denominator: Box::new(Math::Polynom(self.clone())),
            numerator: Box::new(Math::Variable(Variable {
                value: dec!(1),
                suffix: String::new(),
                exponent: None,
            })),
        }
    }

    pub fn map_value(&self, suffix: &str, math: Math) -> Math {
        let mut factors: Vec<Math> = vec![];
        let operators = self.operators.clone();

        for factor in self.factors.iter() {
            factors.push(factor.map_value(suffix, math.clone()));
        }

        Math::Polynom(Polynom { factors, operators })
    }
}

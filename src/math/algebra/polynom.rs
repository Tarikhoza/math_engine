use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};

use crate::math::operator::Operator;
use crate::math::Math;

use crate::math::algebra::variable::Variable;
use crate::math::linear_algebra::vector::Vector;
use crate::parser::Parsable;
use rust_decimal_macros::dec;

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

#[derive(Debug, Clone)]
pub struct Polynom {
    pub factors: Vec<Math>,
    pub operators: Vec<Operator>,
    #[cfg(feature = "step-tracking")]
    pub step: Option<Step>,
}

impl Polynom {
    pub fn unpack(&self) -> Math {
        if self.factors.len() == 1 {
            return self.factors[0].clone();
        }
        Math::Polynom(self.clone())
    }

    pub fn to_vector(&self) -> Vector {
        let mut factors: Vec<Math> = self
            .operators
            .iter()
            .zip(self.factors.iter().skip(1))
            .map(Math::morph_operator)
            .collect();

        factors.insert(0, self.factors.get(0).cloned().unwrap_or_default());
        Vector { factors, 
            #[cfg(feature = "step-tracking")]
            step:None 
        }
    }
    //   pub fn to_fraction(&self) -> Fraction {
    //       Fraction {
    //           denominator: Box::new(Math::Polynom(self.clone())),
    //           numerator: Box::new(Math::Variable(Variable {
    //               value: dec!(1),
    //               suffix: String::new(),
    //               exponent: None,
    //           })),
    //       }
    //   }

    pub fn map_value(&self, suffix: &str, math: Math) -> Math {
        let mut factors: Vec<Math> = vec![];
        let operators = self.operators.clone();

        for factor in self.factors.iter() {
            factors.push(factor.map_value(suffix, math.clone()));
        }

        #[cfg(feature = "step-tracking")]
        let step = Step::step(
            Math::Polynom(self.clone()),
            Some(math),
            Operator::Detail(crate::solver::step::DetailedOperator::MapTo),
            String::from("Map every member to value"),
        );
        Math::Polynom(Polynom {
            factors,
            operators,
            #[cfg(feature = "step-tracking")]
            step,
        })
    }
}

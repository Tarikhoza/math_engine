use crate::castable::Castable;
use crate::math::linear_algebra::vector::Vector;
use crate::math::operator::Operator;
use crate::math::Math;

use crate::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable};

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

#[derive(Debug, Clone, PartialEq)]
pub enum PolynomPart {
    Math(Math),
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polynom {
    pub parts: Vec<PolynomPart>,
    #[cfg(feature = "step-tracking")]
    pub step: Option<Step>,
}

impl Polynom {
    pub fn sort(&self) -> Polynom {
        let mut vec = self.clone().to_vector();

        for f in vec.factors.iter_mut() {
            if let Math::Polynom(poly) = f {
                *f = poly.sort().as_math()
            }
        }

        vec.factors.sort_by_key(|m| m.sorting_score());
        vec.add_all().as_polynom()
    }

    pub fn unpack(&self) -> Math {
        let maths = self.get_maths();
        if maths.len() == 1 {
            return maths[0].clone();
        }
        Math::Polynom(self.clone())
    }

    pub fn get_maths(&self) -> Vec<Math> {
        let mut maths: Vec<Math> = Vec::new();
        for part in self.parts.iter() {
            if let PolynomPart::Math(m) = part {
                maths.push(m.clone());
            }
        }
        maths
    }
    pub fn get_operators(&self) -> Vec<Operator> {
        let mut ops: Vec<Operator> = Vec::new();
        for part in self.parts.iter() {
            if let PolynomPart::Operator(op) = part {
                ops.push(op.clone());
            }
        }
        ops
    }

    pub fn morph_double_operator(&self) -> Math {
        //TODO this is a hack
        let ret = self
            .to_tex()
            .replace("++", "+")
            .replace("--", "+")
            .replace("+-", "-")
            .replace("-+", "-");
        if ret != self.to_tex() {
            return ret
                .parse_math()
                .expect("an error happened while morphing double operators");
        }
        self.unpack()
    }

    pub fn to_vector(&self) -> Vector {
        let mut operators = self.get_operators();
        operators.insert(0, Operator::Empty);
        let factors: Vec<Math> = operators
            .iter()
            .zip(self.get_maths().iter())
            .map(Math::morph_operator)
            .collect();

        Vector {
            factors,
            #[cfg(feature = "step-tracking")]
            step: None,
        }
    }
}

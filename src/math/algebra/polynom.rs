use crate::castable::Castable;
use crate::logging::env_info;
use crate::math::linear_algebra::vector::Vector;
use crate::math::Math;

use crate::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable};

use super::operations::{Operations as AlgebraOperations, Operator as AlgebraOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum PolynomPart {
    Math(Math),
    Operator(AlgebraOperator),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polynom {
    pub parts: Vec<PolynomPart>,
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

    pub fn to_vector(&self) -> Vector {
        let mut factors: Vec<Math> = Vec::new();
        let mut skip = false;
        let parts = self.morph_ops().parts;
        env_info("helper", format!("to_vector before:{:#?}", parts));
        if parts.len() == 1 {
            env_info("helper", "to_vector just one item in polynom".into());
            if let Some(PolynomPart::Math(m)) = &parts.first() {
                factors.push(m.clone());
                env_info("helper", format!("to_vector after:{:#?}", factors));
                return Vector { factors };
            } else {
                env_info(
                    "helper",
                    "to_vector it is an operator, this should never happen".into(),
                );
                panic!("to_vector this should never happen")
            }
        }
        for i in parts.windows(2) {
            if skip {
                skip = false;
                continue;
            }
            match i {
                [PolynomPart::Operator(AlgebraOperator::Addition), PolynomPart::Math(m)] => {
                    env_info("helper", format!("to_vector add +x: {}", m.to_tex()));
                    factors.push(m.clone());
                    skip = true;
                    continue;
                }
                [PolynomPart::Operator(AlgebraOperator::Subtraction), PolynomPart::Math(m)] => {
                    env_info("helper", format!("to_vector add -x: {}", m.to_tex()));
                    factors.push(m.negative());
                    skip = true;
                    continue;
                }
                [PolynomPart::Math(m), _] => {
                    env_info("helper", format!("to_vector add x: {}", m.to_tex()));
                    factors.push(m.clone())
                }
                a => {
                    env_info(
                        "helper",
                        format!("to_vector this should never happen {:#?}", a),
                    );
                    panic!("This should never happen")
                }
            }
        }

        env_info("helper", format!("to_vector after:{:#?}", factors));
        Vector { factors }
    }
}

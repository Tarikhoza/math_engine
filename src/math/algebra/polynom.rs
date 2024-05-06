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
    // groups operators and math components together
    pub fn split(&self) -> Vec<(Option<AlgebraOperator>, Math)> {
        if self
            .parts
            .contains(&PolynomPart::Operator(AlgebraOperator::Multiplication))
            || self
                .parts
                .contains(&PolynomPart::Operator(AlgebraOperator::Division))
        {
            panic!("before sorting you should use simplify_mul_div on the polynom to avoid this");
        }

        let mut pairs: Vec<(Option<AlgebraOperator>, Math)> = vec![];
        let mut skip = false;

        let mut operator_accumulator: Option<AlgebraOperator> = None;

        for pair in self.parts.windows(2) {
            if skip {
                skip = false;
                continue;
            }
            match pair {
                [PolynomPart::Math(m), _] => {
                    pairs.push((None, m.clone()));
                }
                [PolynomPart::Operator(o), PolynomPart::Math(m)] => {
                    if let Some(ref op_before) = operator_accumulator {
                        pairs.push((Some(op_before.morph(o.clone())), m.clone()));
                        operator_accumulator = None;
                    } else {
                        pairs.push((Some(o.clone()), m.clone()));
                    }
                    skip = true;
                }
                [PolynomPart::Operator(o), PolynomPart::Math(m)] => {
                    pairs.push((Some(o.clone()), m.clone()));
                    skip = true;
                }
                _ => todo!(),
            }
        }
        return pairs;
    }

    pub fn accumulate(&self) -> Polynom {
        // Pairs up operators and math and sorts them
        let mut pairs = self.split();
        pairs.sort_by(|a, b| {
            a.1.sorting_score()
                .partial_cmp(&b.1.sorting_score())
                .unwrap()
        });

        //add pairs up that can be added
        let mut new_pairs = vec![];

        let mut skip = false;
        for window in pairs.windows(2) {
            if skip {
                skip = false;
                continue;
            }

            let accumulated_math = Math::default();
            match window {
                [first, second] => {
                    println!("{} {}", first.1.to_tex(), second.1.to_tex());
                    if first.1.add_sub_base() == second.1.add_sub_base() {
                        match (first.0.clone(), second.0.clone()) {
                            (
                                Some(AlgebraOperator::Subtraction),
                                Some(AlgebraOperator::Subtraction),
                            ) => {}
                            (
                                Some(AlgebraOperator::Subtraction),
                                Some(AlgebraOperator::Addition),
                            ) => {}
                            (
                                Some(AlgebraOperator::Addition),
                                Some(AlgebraOperator::Subtraction),
                            ) => {}
                            (Some(AlgebraOperator::Addition), Some(AlgebraOperator::Addition)) => {}
                            _ => todo!(),
                        }
                    } else {
                        // If they cannot be combined, just push the first element to new_pairs
                        new_pairs.push(first.clone());
                    }
                }
                _ => {} // Handle the case where the window does not have exactly two elements
            }
        }

        let mut parts = vec![];
        let mut first = true;
        for (op, math) in new_pairs {
            if let Some(o) = op {
                if let AlgebraOperator::Subtraction = o {
                    parts.push(PolynomPart::Operator(o));
                } else if !first && AlgebraOperator::Addition == o {
                    parts.push(PolynomPart::Operator(o));
                }
            } else {
                if !first {
                    parts.push(PolynomPart::Operator(AlgebraOperator::Addition));
                }
            }
            parts.push(PolynomPart::Math(math));

            first = false;
        }

        Polynom { parts }
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

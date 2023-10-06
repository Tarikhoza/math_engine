use crate::castable::Castable;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::operations::Operator as AlgebraOperator;
use crate::math::algebra::variable::Variable;
use crate::math::linear_algebra::vector::Vector;
use crate::math::operator::Operator;
use crate::math::Math;

use crate::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable};

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

use super::exponentable::Exponentable;

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
        if maths.len() == 1 && self.parts.len() == 1 {
            return maths[0].clone();
        }
        Math::Polynom(self.clone())
    }

    pub fn flatten(&self) -> Polynom {
        let mut parts = Vec::new();
        for part in self.parts.iter() {
            match part {
                PolynomPart::Math(Math::Polynom(p)) => {
                    for i in p.flatten().parts.iter() {
                        parts.push(i.clone())
                    }
                }
                PolynomPart::Math(Math::Braces(b)) => {
                    let math;
                    let exponent;
                    if let Math::Polynom(p) = *b.math.clone() {
                        math = Box::new(p.flatten().as_math());
                    } else {
                        math = b.math.clone();
                    }
                    if let Math::Polynom(p) = b.get_exponent() {
                        exponent = Some(Box::new(p.flatten().as_math()));
                    } else {
                        exponent = Some(Box::new(b.get_exponent()));
                    }
                    parts.push(PolynomPart::Math(Braces { math, exponent }.as_math()))
                }
                PolynomPart::Math(Math::Variable(v)) => {
                    let exponent;
                    if let Math::Polynom(p) = v.get_exponent() {
                        exponent = Some(Box::new(p.flatten().as_math()));
                    } else {
                        exponent = Some(Box::new(v.get_exponent()));
                    }
                    parts.push(PolynomPart::Math(
                        Variable {
                            value: v.value,
                            suffix: v.suffix.clone(),
                            exponent,
                        }
                        .as_math(),
                    ))
                }
                PolynomPart::Math(Math::Fraction(f)) => {
                    let denominator;
                    let numerator;
                    if let Math::Polynom(p) = *f.denominator.clone() {
                        denominator = Box::new(p.flatten().as_math());
                    } else {
                        denominator = f.denominator.clone();
                    }
                    if let Math::Polynom(p) = *f.numerator.clone() {
                        numerator = Box::new(p.flatten().as_math());
                    } else {
                        numerator = f.numerator.clone();
                    }
                    parts.push(PolynomPart::Math(
                        Fraction {
                            whole: f.whole,
                            denominator,
                            numerator,
                        }
                        .as_math(),
                    ))
                }
                other => parts.push(other.clone()),
            }
        }
        Polynom { parts }
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

    pub fn morph_double_operator(&self) -> Polynom {
        println!("before morph_double_operator {}", self.to_tex());
        let mut parts: Vec<PolynomPart> = Vec::new();
        let mut operator = Operator::Empty;

        for part in self.parts.iter() {
            match part {
                PolynomPart::Math(math) => {
                    let (s_op, s_math) = math.split_operator();
                    if operator != Operator::Empty
                        && s_op == Operator::Algebra(AlgebraOperator::Subtraction)
                    {
                        println!(
                            "if: pushing {} to polyparts",
                            operator.morph(&s_op.clone()).to_tex()
                        );
                        parts.push(operator.morph(&s_op.clone()).as_polynom_part());
                        parts.push(s_math.as_polynom_part());
                    } else if operator != Operator::Empty {
                        parts.push(operator.as_polynom_part());
                        parts.push(s_math.as_polynom_part());
                        println!("else: pushing {} to polyparts", operator.clone().to_tex());
                    } else {
                        println!("pushing {} to polyparts", math.clone().to_tex());
                        parts.push(math.as_polynom_part());
                    }
                }
                PolynomPart::Operator(op) => {
                    operator = operator.morph(&op.clone());
                }
            }
        }
        println!(
            "after morph_double_operator {}",
            Polynom {
                parts: parts.clone()
            }
            .to_tex()
        );
        Polynom { parts }
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

use crate::fraction::Fraction;
use crate::math::{BasicOperations, Math};
use crate::operators::Operators;
use crate::parser::Parsable;
use crate::variable::Variable;
use crate::vector::Vector;
use rust_decimal_macros::dec;
use std::ops;

#[derive(Debug, Clone)]
pub struct Polynom {
    pub factors: Vec<Math>,
    pub operators: Vec<Operators>,
}

impl Polynom {
    //  Simplify helper functions
    //  PEMDAS
    //  P - Parentheses first
    #[must_use]
    pub fn simplify_par(&self) -> Polynom {
        if self.factors.len() <= 1 {
            return Polynom {
                factors: self.factors.clone(),
                operators: self.operators.clone(),
            };
        }
        let mut factors: Vec<Math> = vec![];
        let mut operators: Vec<Operators> = vec![];
        //        let mut chan: bool = false;

        for (i, factor) in self.factors.iter().take(self.factors.len()).enumerate() {
            match factor {
                Math::Braces(b) => {
                    factors.push(b.simplify());
                }
                o => {
                    factors.push(o.clone());
                }
            }
            if i != self.factors.len() - 1 {
                operators.push(self.operators[i].clone());
            }
        }

        Polynom { factors, operators }
    }

    //  E - Exponents (ie Powers and Square Roots, etc.)
    #[must_use]
    pub fn simplify_exp(&self) -> Polynom {
        self.clone()
    }

    //  MD - Multiplication and Division (left-to-right)
    #[must_use]
    pub fn simplify_mul_div(&self) -> Polynom {
        if self.factors.len() <= 1
            || (!self.operators.contains(&Operators::Multiplication)
                && !self.operators.contains(&Operators::Division))
        {
            return Polynom {
                factors: self.factors.clone(),
                operators: self.operators.clone(),
            };
        }
        let mut factors: Vec<Math> = vec![];
        let mut operators: Vec<Operators> = vec![];

        let mut chan: bool = false;
        let mut skip: bool = false;

        for (i, _factor) in self.factors.iter().take(self.factors.len() - 1).enumerate() {
            if chan {
                operators.push(self.operators[i].clone());
                if skip {
                    skip = false;
                } else {
                    factors.push(self.factors[i].clone());
                }
                if i == self.factors.len() - 2 {
                    factors.push(self.factors[i + 1].clone());
                }
                continue;
            }
            match &self.operators[i] {
                Operators::Multiplication => {
                    let f = self.factors[i].clone() * self.factors[i + 1].clone();
                    if f.to_tex() != "0" {
                        factors.push(f);
                    }
                    chan = true;
                    skip = true;
                }
                Operators::Division => {
                    let f = self.factors[i].clone() / self.factors[i + 1].clone();
                    if f.to_tex() != "0" {
                        factors.push(f);
                    }
                    chan = true;
                    skip = true;
                }
                o => {
                    factors.push(self.factors[i].clone());
                    operators.push(o.clone());
                }
            }
        }
        let p = Polynom { factors, operators };
        if p.factors.len() > 1
            && (p.operators.contains(&Operators::Multiplication)
                || p.operators.contains(&Operators::Division))
        {
            return p.simplify_mul_div();
        }
        p
    }

    //  AS - Addition and Subtraction (left-to-right)

    #[must_use]
    pub fn simplify_add_sub(&self) -> Math {
        self.to_vector().to_based_matrix().add_all()
    }
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
}

impl ops::Add<Math> for Polynom {
    type Output = Math;
    fn add(self, rhs: Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.addition(p),
            Math::Variable(v) => self.addition(v.as_polynom()),
            Math::Braces(b) => self + b.simplify(),
            Math::Fraction(f) => self.to_fraction().addition(f),
            Math::Undefined(u) => Math::Undefined(u),
            Math::Operators(_) => todo!(),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Polynom {
    type Output = Math;
    fn sub(self, rhs: Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.subtraction(p),
            Math::Variable(v) => self.subtraction(v.as_polynom()),
            Math::Braces(b) => self - b.simplify(),
            Math::Undefined(u) => Math::Undefined(u),
            Math::Operators(_) => todo!(),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Polynom {
    type Output = Math;
    fn mul(self, rhs: Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.multiplication(p),
            Math::Variable(v) => self.multiplication(v.as_polynom()),
            Math::Braces(b) => self * b.simplify(),
            Math::Undefined(u) => Math::Undefined(u),
            Math::Operators(_) => todo!(),
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Polynom {
    type Output = Math;
    fn div(self, _rhs: Math) -> Math {
        todo!()
    }
}

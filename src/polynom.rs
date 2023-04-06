use std::ops;

use crate::math::{BasicOperations, Math, Operators};
use crate::parser::Parsable;

#[derive(Debug, Clone)]
pub struct Polynom {
    pub factors: Vec<Math>,
    pub operators: Vec<Operators>,
}

impl BasicOperations for Polynom {
    fn addition(&self, other: Polynom) -> Math {
        Math::Polynom(Polynom {
            factors: {
                let mut factors = self.factors.clone();
                factors.extend(other.factors);
                factors
            },
            operators: {
                let mut operators = self.operators.clone();
                operators.push(Operators::Addition);
                operators.extend(other.operators.iter().cloned());
                operators
            },
        })
    }
    fn subtraction(&self, other: Polynom) -> Math {
        Math::Polynom(Polynom {
            factors: {
                let mut factors = self.factors.clone();
                factors.extend(other.factors);
                factors
            },
            operators: {
                let mut operators = self.operators.clone();
                operators.push(Operators::Subtraction);
                operators.extend(other.operators.iter().cloned());
                operators
            },
        })
    }

    fn multiplication(&self, other: Polynom) -> Math {
        let mut factors: Vec<Math> = vec![];
        for i in self.factors.iter() {
            for j in other.factors.iter() {
                factors.push((i.clone()) * (j.clone()));
            }
        }
        let len = factors.len();
        Math::Polynom(Polynom {
            factors,
            operators: vec![Operators::InvMulti],
        })
    }

    fn division(&self, other: Polynom) -> Math {
        todo!()
    }

    fn negative(&self) -> Math {
        let mut factors = vec![];
        for factor in self.factors.iter() {
            factors.push(factor.clone().negative());
        }
        Math::Polynom(Polynom {
            factors,
            operators: self.operators.clone(),
        })
    }

    //  PEMDAS
    fn simplify(&self) -> Math {
        return self
            .simplify_par()
            .simplify_exp()
            .simplify_mul_div()
            .sort()
            .simplify_add_sub()
            .unpack();
    }
}

//simplify helper functions
//  PEMDAS
impl Polynom {
    //  P - Parentheses first
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
        let p = Polynom { factors, operators };
        p
    }

    //  E - Exponents (ie Powers and Square Roots, etc.)
    pub fn simplify_exp(&self) -> Polynom {
        self.clone()
    }

    //  MD - Multiplication and Division (left-to-right)
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
        println!("{}", p.to_tex());
        if p.factors.len() > 1
            && (p.operators.contains(&Operators::Multiplication)
                || p.operators.contains(&Operators::Division))
        {
            return p.simplify_mul_div();
        }
        p
    }

    //  AS - Addition and Subtraction (left-to-right)

    pub fn sort(&self) -> Polynom {
        let mut old_factors = self.factors.clone();
        let mut old_operators = self.operators.clone();
        let first = old_factors.get(0).unwrap().split_operator();
        old_operators.insert(1, first.0);
        old_factors[0] = first.1;
        let mut to_sort: Vec<(&Math, &Operators)> =
            old_factors.iter().zip(old_operators.iter()).collect();
        to_sort.sort_by(|a, b| a.0.sort_score().cmp(&b.0.sort_score()));
        let factors = to_sort.iter().map(|x| x.0.clone()).collect();
        let operators = to_sort.iter().map(|x| x.1.clone()).collect();

        Polynom { factors, operators }
    }

    pub fn simplify_add_sub(&self) -> Polynom {
        if self.factors.len() <= 1
            || (!self.operators.contains(&Operators::Addition)
                && !self.operators.contains(&Operators::Subtraction))
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
                Operators::Addition => {
                    if self.factors[i].add_sub_change(&self.factors[i + 1]) {
                        factors.push(self.factors[i].clone() + self.factors[i + 1].clone());
                        chan = true;
                        skip = true;
                    } else {
                        factors.push(self.factors[i].clone());
                        operators.push(self.operators[i].clone());
                    }
                }
                Operators::Subtraction => {
                    if self.factors[i].add_sub_change(&self.factors[i + 1]) {
                        factors.push(self.factors[i].clone() - self.factors[i + 1].clone());
                        chan = true;
                        skip = true;
                    } else {
                        factors.push(self.factors[i].clone());
                        operators.push(self.operators[i].clone());
                    }
                }
                o => {
                    factors.push(self.factors[i].clone());
                    operators.push(o.clone());
                }
            }
        }
        let p = Polynom { factors, operators };
        p
    }

    pub fn unpack(&self) -> Math {
        if self.factors.len() == 1 {
            return self.factors[0].clone();
        }
        Math::Polynom(self.clone())
    }
}

impl Polynom {
    pub fn new(tex: &str) -> Math {
        Polynom::from_tex(tex.to_string())
    }
}

impl Parsable for Polynom {
    fn to_tex(&self) -> String {
        if self.factors.len() != 0 {
            if self.factors.len() <= 1 && self.factors.len() != self.operators.len() + 1 {
                return self.factors[0].to_tex();
            }
            let mut temp = self.factors[0].to_tex();
            for (i, factor) in self.factors.iter().skip(1).enumerate() {
                temp = format!(
                    "{}{}{}",
                    temp,
                    Math::operators_to_string(self.operators[i].clone()),
                    factor.to_tex()
                )
            }
            return temp;
        }
        "".to_string()
    }

    fn from_tex(tex: String) -> Math {
        Math::from_tex(tex)
    }

    fn on_begining(tex: String) -> Option<String> {
        None
    }
}

impl ops::Add<Math> for Polynom {
    type Output = Math;
    fn add(self, _rhs: Math) -> Math {
        match _rhs {
            Math::Polynom(p) => self.addition(p),
            Math::Variable(v) => self.addition(v.as_polynom()),
            Math::Braces(b) => self + b.simplify(),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Polynom {
    type Output = Math;
    fn sub(self, _rhs: Math) -> Math {
        match _rhs {
            Math::Polynom(p) => self.subtraction(p),
            Math::Variable(v) => self.subtraction(v.as_polynom()),
            Math::Braces(b) => self - b.simplify(),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Polynom {
    type Output = Math;
    fn mul(self, _rhs: Math) -> Math {
        match _rhs {
            Math::Polynom(p) => self.multiplication(p),
            Math::Variable(v) => self.multiplication(v.as_polynom()),
            Math::Braces(b) => self * b.simplify(),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Polynom {
    type Output = Math;
    fn div(self, _rhs: Math) -> Math {
        match _rhs {
            //           Math::Polynom(p)   => self.division(p),
            //           Math::Variable(v)  => self.division(v.as_polynom()),
            //           Math::Braces(b)    => self / b.simplify(),
            //           Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

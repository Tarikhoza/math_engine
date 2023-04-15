use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::undefined::Undefined;
use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperators,
};
use crate::math::Math;
use crate::parser::{Parsable, Parser};

impl AlgebraOperations for Polynom {
    fn addition(&self, other: &Polynom) -> Math {
        Math::Polynom(Polynom {
            factors: {
                let mut factors = self.factors.clone();
                factors.extend(other.factors.clone());
                factors
            },
            operators: {
                let mut operators = self.operators.clone();
                operators.push(AlgebraOperators::Addition);
                operators.extend(other.operators.iter().cloned());
                operators
            },
        })
    }
    fn subtraction(&self, other: &Polynom) -> Math {
        Math::Polynom(Polynom {
            factors: {
                let mut factors = self.factors.clone();
                factors.extend(other.factors.clone());
                factors
            },
            operators: {
                let mut operators = self.operators.clone();
                operators.push(AlgebraOperators::Subtraction);
                operators.extend(other.operators.iter().cloned());
                operators
            },
        })
    }

    fn multiplication(&self, other: &Polynom) -> Math {
        let mut factors: Vec<Math> = vec![];
        for i in self.factors.iter() {
            for j in other.factors.iter() {
                factors.push(i.mul(&j));
            }
        }
        let _len = factors.len();
        Math::Polynom(Polynom {
            factors,
            operators: vec![AlgebraOperators::InvMulti],
        })
    }

    #[must_use]
    fn division(&self, _other: &Polynom) -> Math {
        todo!()
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.addition(p),
            Math::Variable(v) => self.addition(&v.as_polynom()),
            Math::Braces(b) => self.add(&b.simplify()),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.subtraction(&p),
            Math::Variable(v) => self.subtraction(&v.as_polynom()),
            Math::Braces(b) => self.sub(&b.simplify()),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.multiplication(&p),
            Math::Variable(v) => self.multiplication(&v.as_polynom()),
            Math::Braces(b) => self.mul(&b.simplify()),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn div(&self, _rhs: &Math) -> Math {
        todo!()
    }

    #[must_use]
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
    #[must_use]
    fn simplify(&self) -> Math {
        self.simplify_par()
            .simplify_exp()
            .simplify_mul_div()
            .simplify_add_sub()
    }
}

//simplify helper functions
//  PEMDAS
impl Polynom {
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
        let mut operators: Vec<AlgebraOperators> = vec![];
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
            || (!self.operators.contains(&AlgebraOperators::Multiplication)
                && !self.operators.contains(&AlgebraOperators::Division))
        {
            return self.clone();
        }
        let mut factors: Vec<Math> = vec![];
        let mut operators: Vec<AlgebraOperators> = vec![];

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
                AlgebraOperators::Multiplication => {
                    let f = self.factors[i].mul(&self.factors[i + 1]);
                    if f.to_tex() != "0" {
                        factors.push(f);
                    }
                    chan = true;
                    skip = true;
                }
                AlgebraOperators::Division => {
                    let f = self.factors[i].div(&self.factors[i + 1]);
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
            && (p.operators.contains(&AlgebraOperators::Multiplication)
                || p.operators.contains(&AlgebraOperators::Division))
        {
            return p.simplify_mul_div();
        }
        p
    }

    //  AS - Addition and Subtraction (left-to-right)

    #[must_use]
    pub fn simplify_add_sub(&self) -> Math {
        println!("{}", self.to_vector().to_tex());
        println!("{}", self.to_vector().to_based_matrix().to_tex());
        self.to_vector().to_based_matrix().add_all()
    }
}

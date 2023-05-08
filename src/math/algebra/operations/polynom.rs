use crate::math::algebra::polynom::Polynom;
//use crate::math::algebra::undefined::Undefined;
use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperators,
};
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::{Parsable, Parser};

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

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
                operators.push(Operator::Algebra(AlgebraOperators::Addition));
                operators.extend(other.operators.iter().cloned());
                operators
            },
            #[cfg(feature = "step-tracking")]
            step: Step::step(
                Math::Polynom(self.clone()),
                Some(Math::Polynom(other.clone())),
                Operator::Algebra(AlgebraOperators::Addition),
                String::from("Grouping two polynoms"),
            ),
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
                operators.push(Operator::Algebra(AlgebraOperators::Subtraction));
                operators.extend(other.operators.iter().cloned());
                operators
            },
            #[cfg(feature = "step-tracking")]
            step: Step::step(
                Math::Polynom(self.clone()),
                Some(Math::Polynom(other.clone())),
                Operator::Algebra(AlgebraOperators::Subtraction),
                String::from("Grouping two polynoms"),
            ),
        })
    }

    fn multiplication(&self, other: &Polynom) -> Math {
        let mut factors: Vec<Math> = vec![];

        #[cfg(feature = "step-tracking")]
        let mut steps: Vec<Step> = vec![];

        for i in self.factors.iter() {
            for j in other.factors.iter() {
                let product = i.mul(j);
                #[cfg(feature = "step-tracking")]
                steps.push(product.get_step());
                factors.push(i.mul(j));
            }
        }
        let len = factors.len();
        Math::Polynom(Polynom {
            factors,
            operators: vec![Operator::Algebra(AlgebraOperators::Addition); len - 1],
            #[cfg(feature = "step-tracking")]
            step: Step::steps(
                Math::Polynom(self.clone()),
                Some(Math::Polynom(other.clone())),
                Operator::Algebra(AlgebraOperators::Multiplication),
                String::from("Multiping two polynoms"),
                steps,
            ),
        })
    }

    fn division(&self, _other: &Polynom) -> Math {
        todo!()
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.addition(p),
            Math::Variable(v) => self.addition(&v.as_polynom()),
            Math::Braces(b) => self.add(&b.simplify()),
//            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.subtraction(p),
            Math::Variable(v) => self.subtraction(&v.as_polynom()),
            Math::Braces(b) => self.sub(&b.simplify()),
//            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.multiplication(p),
            Math::Variable(v) => self.multiplication(&v.as_polynom()),
            Math::Braces(b) => self.mul(&b.simplify()),
//            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn div(&self, _rhs: &Math) -> Math {
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
            #[cfg(feature = "step-tracking")]
            step: Step::step(
                Math::Polynom(self.clone()),
                None,
                Operator::Detail(DetailedOperator::Negate),
                String::from("Negate polynom"),
            ),
        })
    }

    //  PEMDAS
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
    pub fn simplify_par(&self) -> Polynom {
        if self.factors.len() <= 1 {
            return Polynom {
                factors: self.factors.clone(),
                operators: self.operators.clone(),
                #[cfg(feature = "step-tracking")]
                step: None,
            };
        }
        let mut factors: Vec<Math> = vec![];
        let mut operators: Vec<Operator> = vec![];
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

        Polynom {
            factors,
            operators,
            #[cfg(feature = "step-tracking")]
            step: None,
        }
    }

    //  E - Exponents (ie Powers and Square Roots, etc.)
    pub fn simplify_exp(&self) -> Polynom {
        self.clone()
    }

    //  MD - Multiplication and Division (left-to-right)
    pub fn simplify_mul_div(&self) -> Polynom {
        if self.factors.len() <= 1
            || (!self
                .operators
                .contains(&Operator::Algebra(AlgebraOperators::Multiplication))
                && !self
                    .operators
                    .contains(&Operator::Algebra(AlgebraOperators::Division)))
        {
            return self.clone();
        }
        let mut factors: Vec<Math> = vec![];
        let mut operators: Vec<Operator> = vec![];

        #[cfg(feature = "step-tracking")]
        let mut steps: Vec<Step> = vec![];

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
                Operator::Algebra(AlgebraOperators::Multiplication) => {
                    let f = self.factors[i].mul(&self.factors[i + 1]);
                    #[cfg(feature = "step-tracking")]
                    steps.push(f.get_step());
                    if f.to_tex() != "0" {
                        factors.push(f);
                    }
                    chan = true;
                    skip = true;
                }
                Operator::Algebra(AlgebraOperators::Division) => {
                    let f = self.factors[i].div(&self.factors[i + 1]);
                    #[cfg(feature = "step-tracking")]
                    steps.push(f.get_step());
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
        let p = Polynom {
            factors,
            operators,
            #[cfg(feature = "step-tracking")]
            step: Step::steps(
                Math::Polynom(self.clone()),
                None,
                Operator::Detail(DetailedOperator::SimplifyMultiplicationDivision),
                String::from("Multiping every element in polynom"),
                steps,
            ),
        };

        if p.factors.len() > 1
            && (p
                .operators
                .contains(&Operator::Algebra(AlgebraOperators::Multiplication))
                || p.operators
                    .contains(&Operator::Algebra(AlgebraOperators::Division)))
        {
            return p.simplify_mul_div();
        }
        p
    }

    //  AS - Addition and Subtraction (left-to-right)

    pub fn simplify_add_sub(&self) -> Math {
        Math::Polynom(self.to_vector().to_based_matrix().add_all().as_polynom())
    }
}

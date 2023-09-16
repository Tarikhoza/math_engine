use crate::castable::Castable;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::operations::{
    Operations as AlgebraOperations, Operator as AlgebraOperators,
};
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::undefined::Undefined;
use crate::math::operator::Operator;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::{Parsable, Parser};

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

impl Simplifiable for Polynom {
    //  PEMDAS
    fn simplify(&self) -> Math {
        self.simplify_par()
            .simplify_exp()
            .simplify_mul_div()
            .simplify_add_sub()
            .as_math()
        //    .morph_double_operator()
    }
}
impl AlgebraOperations for Polynom {
    fn add_self(&self, other: &Polynom) -> Math {
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
    fn sub_self(&self, other: &Polynom) -> Math {
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

    fn mul_self(&self, other: &Polynom) -> Math {
        let mut factors: Vec<Math> = vec![];
        #[cfg(feature = "step-tracking")]
        let mut steps: Vec<Step> = vec![];
        for i in self.to_vector().factors.iter() {
            for j in other.to_vector().factors.iter() {
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

    fn div_self(&self, _other: &Polynom) -> Math {
        todo!("you have to implement division between polynomes")
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.add_self(p),
            Math::Variable(v) => self.add_self(&v.as_polynom()),
            Math::Braces(b) => self.add(&b.simplify()),
            Math::Fraction(f) => self.as_fraction().add_self(f),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.sub_self(p),
            Math::Variable(v) => self.sub_self(&v.as_polynom()),
            Math::Braces(b) => self.sub(&b.simplify()),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            Math::Fraction(f) => self.as_fraction().sub_self(f),
            _ => todo!(),
        }
    }
    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.mul_self(p),
            Math::Variable(v) => self.mul_self(&v.as_polynom()),
            Math::Braces(b) => self.mul(&b.simplify()),
            Math::Fraction(f) => self.as_fraction().mul_self(f),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!("did not implement mul with polynom"),
        }
    }
    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.as_fraction().div_self(f),
            _ => todo!("did not implement div with polynom"),
        }
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

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        let mut new_poly = Polynom {
            factors: vec![],
            operators: vec![],
        };
        let mut operators = self.operators.clone();
        operators.push(Operator::Algebra(AlgebraOperators::Addition));
        for (factor, operator) in self.factors.iter().zip(&operators) {
            new_poly.push(factor.substitute(suffix, math.clone()), operator.clone());
        }
        new_poly.unpack()
    }
    fn get_all_suffixes(&self) -> Vec<String> {
        let mut suf: Vec<String> = vec![];
        for i in self.factors.iter() {
            suf.extend(i.get_all_suffixes())
        }
        suf.sort();
        suf.dedup();
        suf
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

        for (i, factor) in self.factors.iter().take(self.factors.len()).enumerate() {
            match factor.as_polynom().unpack() {
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

    //  E - Exponents (Powers and Square Roots, etc.)
    pub fn simplify_exp(&self) -> Polynom {
        let mut factors: Vec<Math> = vec![];
        for i in self.factors.iter() {
            match i.as_polynom().unpack() {
                Math::Braces(b) => factors.push(b.apply_exponent()),
                Math::Variable(v) => factors.push(v.apply_exponent()),
                s => factors.push(s.clone()),
            }
        }
        Polynom {
            factors,
            operators: self.operators.clone(),
            #[cfg(feature = "step-tracking")]
            step: None,
        }
    }

    //  MD - Multiplication and Division (left-to-right)
    pub fn simplify_mul_div(&self) -> Polynom {
        if self.factors.len() <= 1
            || (!self
                .operators
                .contains(&Operator::Algebra(AlgebraOperators::Multiplication))
                && !self
                    .operators
                    .contains(&Operator::Algebra(AlgebraOperators::InvMulti))
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

        for (i, _factor) in self.factors.iter().take(self.factors.len() - 1).enumerate() {
            match &self.operators[i] {
                Operator::Algebra(AlgebraOperators::Multiplication)
                | Operator::Algebra(AlgebraOperators::InvMulti) => {
                    let f = self.factors[i].mul(&self.factors[i + 1]);
                    #[cfg(feature = "step-tracking")]
                    steps.push(f.get_step());
                    if f.to_tex() != "0" {
                        factors.push(f);
                    }
                    factors.extend_from_slice(self.factors.get(i + 2..).unwrap_or(&[]));
                    operators.extend_from_slice(self.operators.get(i + 1..).unwrap_or(&[]));
                    break;
                }
                Operator::Algebra(AlgebraOperators::Division) => {
                    let f = self.factors[i].div(&self.factors[i + 1]);
                    #[cfg(feature = "step-tracking")]
                    steps.push(f.get_step());
                    if f.to_tex() != "0" {
                        factors.push(f);
                    }
                    factors.extend_from_slice(self.factors.get(i + 2..).unwrap_or(&[]));
                    operators.extend_from_slice(self.operators.get(i + 1..).unwrap_or(&[]));
                    break;
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
                    .contains(&Operator::Algebra(AlgebraOperators::Multiplication))
                || p.operators
                    .contains(&Operator::Algebra(AlgebraOperators::Division)))
        {
            return p.simplify_mul_div();
        }
        p
    }

    //  AS - Addition and Subtraction (left-to-right)

    pub fn simplify_add_sub(&self) -> Polynom {
        let mut vec = self
            .to_vector()
            .to_based_matrix()
            .add_all()
            .as_polynom()
            .to_vector();

        vec.factors.sort_by_key(|m| m.sorting_score());
        vec.add_all().as_polynom()
    }
}

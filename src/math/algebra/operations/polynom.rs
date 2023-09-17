use crate::castable::Castable;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::operations::{
    Operations as AlgebraOperations, Operator as AlgebraOperators,
};
use crate::math::algebra::polynom::{Polynom, PolynomPart};
use crate::math::algebra::undefined::Undefined;
use crate::math::operator::Operator;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::Parsable;

#[cfg(feature = "step-tracking")]
use crate::solver::step::{DetailedOperator, Step};

impl Simplifiable for Polynom {
    //  PEMDAS
    fn simplify(&self) -> Math {
        self.simplify_par()
            .simplify_exp()
            .simplify_mul_div()
            .simplify_add_sub()
            .unpack()
            .as_polynom()
            .morph_double_operator()
    }
}
impl AlgebraOperations for Polynom {
    fn add_self(&self, other: &Polynom) -> Math {
        Math::Polynom(Polynom {
            parts: {
                let mut parts = self.parts.clone();
                parts.push(Operator::Algebra(AlgebraOperators::Addition).as_polynom_part());
                parts.extend(other.parts.clone());
                parts
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
            parts: {
                let mut parts = self.parts.clone();
                parts.push(Operator::Algebra(AlgebraOperators::Subtraction).as_polynom_part());
                parts.extend(other.parts.clone());
                parts
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
        let mut parts: Vec<PolynomPart> = vec![];
        #[cfg(feature = "step-tracking")]
        let mut steps: Vec<Step> = vec![];
        for i in self.to_vector().factors.iter() {
            for j in other.to_vector().factors.iter() {
                let product = i.mul(j);
                #[cfg(feature = "step-tracking")]
                steps.push(product.get_step());
                parts.push(product.as_polynom_part());
                parts.push(Operator::Algebra(AlgebraOperators::Addition).as_polynom_part());
            }
        }
        parts.pop();

        Polynom {
            parts,
            #[cfg(feature = "step-tracking")]
            step: Step::steps(
                self.as_math(),
                Some(other.as_math()),
                Operator::Algebra(AlgebraOperators::Multiplication),
                String::from("Multiply two polynoms"),
                steps,
            ),
        }
        .as_math()
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
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.sub_self(p),
            Math::Variable(v) => self.sub_self(&v.as_polynom()),
            Math::Braces(b) => self.sub(&b.simplify()),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
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
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!("did not implement mul with polynom"),
        }
    }
    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.as_fraction().div_self(f),
            _ => todo!("did not implement div with polynom"),
        }
    }

    fn substitute(&self, suffix: &str, _math: Math) -> Math {
        let mut new_poly = self.clone();
        for part in new_poly.parts.iter_mut() {
            if let PolynomPart::Math(math) = part {
                *part = math.substitute(suffix, _math.clone()).as_polynom_part();
            }
        }
        new_poly.as_math()
    }
    fn get_all_suffixes(&self) -> Vec<String> {
        let mut suf: Vec<String> = vec![];
        for i in self.get_maths().iter() {
            suf.extend(i.get_all_suffixes())
        }
        suf.sort();
        suf.dedup();
        suf
    }
}

//  Simplify helper functions
//  PEMDAS
impl Polynom {
    //  P - Parentheses
    pub fn simplify_par(&self) -> Polynom {
        let mut parts: Vec<PolynomPart> = self.parts.clone();
        for part in parts.iter_mut() {
            if let PolynomPart::Math(Math::Braces(b)) = part {
                *part = b.simplify().as_polynom_part();
            }
        }

        Polynom {
            parts,
            #[cfg(feature = "step-tracking")]
            step: None,
        }
    }

    //  E - Exponents (Powers and Square Roots, etc.)
    pub fn simplify_exp(&self) -> Polynom {
        let mut parts: Vec<PolynomPart> = self.parts.clone();

        for part in parts.iter_mut() {
            match part {
                PolynomPart::Math(Math::Braces(b)) => {
                    *part = b.apply_exponent().as_polynom_part();
                }
                PolynomPart::Math(Math::Variable(v)) => {
                    *part = v.apply_exponent().as_polynom_part();
                }
                _ => {}
            }
        }

        Polynom {
            parts,
            #[cfg(feature = "step-tracking")]
            step: None,
        }
    }

    //  MD - Multiplication and Division (left-to-right)
    pub fn simplify_mul_div(&self) -> Polynom {
        let mut parts = Vec::new();
        let maths = self.get_maths();
        let operators = self.get_operators();

        if maths.len() == 1
            || !operators.contains(&Operator::Algebra(AlgebraOperators::Multiplication))
                && !operators.contains(&Operator::Algebra(AlgebraOperators::InvMulti))
                && !operators.contains(&Operator::Algebra(AlgebraOperators::Division))
        {
            return self.clone();
        }

        #[cfg(feature = "step-tracking")]
        let mut steps: Vec<Step> = vec![];

        for (i, window) in self.parts.windows(3).enumerate() {
            if let (
                Some(PolynomPart::Math(lhs)),
                Some(PolynomPart::Operator(op)),
                Some(PolynomPart::Math(rhs)),
            ) = (window.get(0), window.get(1), window.get(2))
            {
                match op {
                    Operator::Algebra(AlgebraOperators::Multiplication)
                    | Operator::Algebra(AlgebraOperators::InvMulti) => {
                        let part = lhs.mul(rhs);
                        #[cfg(feature = "step-tracking")]
                        steps.push(p.get_step());
                        if part.to_tex() != "0" {
                            parts.push(part.as_polynom_part());
                        }
                        parts.extend_from_slice(self.parts.get(i + 3..).unwrap_or(&[]));
                        break;
                    }
                    Operator::Algebra(AlgebraOperators::Division) => {
                        let part = lhs.div(rhs);
                        #[cfg(feature = "step-tracking")]
                        steps.push(f.get_step());

                        if part.to_tex() != "0" {
                            parts.push(part.as_polynom_part());
                        }
                        parts.extend_from_slice(self.parts.get(i + 3..).unwrap_or(&[]));
                        break;
                    }
                    _ => {
                        parts.push(lhs.as_polynom_part());
                        parts.push(op.as_polynom_part());
                    }
                }
            }
        }
        let p = Polynom {
            parts,
            #[cfg(feature = "step-tracking")]
            step: Step::steps(
                Math::Polynom(self.clone()),
                None,
                Operator::Detail(DetailedOperator::SimplifyMultiplicationDivision),
                String::from("Multiply every element in polynom"),
                steps,
            ),
        };

        let maths = p.get_maths();
        let operators = p.get_operators();

        if maths.len() >= 2
            || (!operators.contains(&Operator::Algebra(AlgebraOperators::Multiplication))
                && !operators.contains(&Operator::Algebra(AlgebraOperators::InvMulti))
                && !operators.contains(&Operator::Algebra(AlgebraOperators::Division)))
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

        vec.add_all().as_polynom().sort()
    }
}

use crate::castable::Castable;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::operations::Operations;
use crate::math::algebra::undefined::Undefined;
use crate::math::simplifiable::Simplifiable;

use crate::math::Math;

impl Simplifiable for Braces {
    fn simplify(&self) -> Math {
        let mut new = self.clone();
        new.exponent = Some(Box::new(self.get_exponent().simplify()));
        new.apply_exponent()
    }
}

impl Operations for Braces {
    fn add_self(&self, other: &Braces) -> Math {
        self.simplify().add(&other.simplify())
    }

    fn sub_self(&self, other: &Braces) -> Math {
        self.simplify().sub(&other.simplify())
    }

    fn mul_self(&self, other: &Braces) -> Math {
        self.simplify().mul(&other.simplify())
    }

    fn div_self(&self, other: &Braces) -> Math {
        self.simplify().div(&other.simplify())
    }

    fn negative(&self) -> Math {
        match &self.exponent {
            Some(_has_exp) => Math::Braces(Braces {
                math: Box::new(self.math.negative()),
                exponent: Some(Box::new(self.get_exponent())),
            }),
            _no_exp => Math::Braces(Braces {
                math: Box::new(self.math.negative()),
                exponent: None,
            }),
        }
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify().add(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.simplify().add(&Math::Variable(v.clone())),
            Math::Braces(b) => self.add_self(b),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify().sub(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.simplify().sub(&Math::Variable(v.clone())),
            Math::Braces(b) => self.sub_self(b),
            //           Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify().mul(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.simplify().mul(&Math::Variable(v.clone())),
            Math::Braces(b) => self.mul_self(b),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            Math::Fraction(f) => self.simplify().mul(&Math::Fraction(f.clone())),
            _ => todo!(),
        }
    }

    fn div(&self, _rhs: &Math) -> Math {
        todo!()
    }

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        let new_math = Box::new(
            self.math
                .substitute(suffix, math.clone())
                .as_polynom()
                .unpack(),
        );
        let new_exponent = self
            .get_exponent()
            .substitute(suffix, math)
            .as_polynom()
            .unpack();

        Math::Braces(Braces {
            math: new_math,
            exponent: Some(Box::new(new_exponent)),
        })
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        let mut suf: Vec<String> = vec![];
        suf.extend_from_slice(&self.math.get_all_suffixes());
        if let Some(exp) = self.exponent.clone() {
            suf.extend_from_slice(&exp.get_all_suffixes());
        }
        suf.sort();
        suf.dedup();
        suf
    }
}

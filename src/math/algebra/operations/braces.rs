use crate::castable::Castable;
use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::operations::Operations;
use crate::math::algebra::undefined::Undefined;
use crate::math::simplifiable::Simplifiable;
use crate::parser::Parsable;

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
                inner: Box::new(self.inner.negative()),
                exponent: Some(Box::new(self.get_exponent())),
            }),
            _no_exp => Math::Braces(Braces {
                inner: Box::new(self.inner.negative()),
                exponent: None,
            }),
        }
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify().add(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.simplify().add(&Math::Variable(v.clone())),
            Math::Braces(b) => self.add_self(b),
            Math::Fraction(f) => self.as_fraction().add_self(f),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(
                "add not implemented for {} and {}",
                self.get_type(),
                rhs.get_type()
            ),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify().sub(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.simplify().sub(&Math::Variable(v.clone())),
            Math::Braces(b) => self.sub_self(b),
            Math::Fraction(f) => self.as_fraction().sub_self(f),
            _ => todo!(
                "sub not implemented for {} and {}",
                self.get_type(),
                rhs.get_type()
            ),
        }
    }

    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify().mul(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.simplify().mul(&Math::Variable(v.clone())),
            Math::Braces(b) => self.mul_self(b),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            Math::Fraction(f) => self.as_fraction().mul_self(f),
            _ => todo!(
                "mul not implemented for {} and {}",
                self.get_type(),
                rhs.get_type()
            ),
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.as_fraction().div_self(f),
            _ => todo!(
                "mul not implemented for {} and {}",
                self.get_type(),
                rhs.get_type()
            ),
        }
    }

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        let new_math = Box::new(
            self.inner
                .substitute(suffix, math.clone())
                .as_polynom()
                .as_math(),
        );
        let new_exponent = self
            .get_exponent()
            .substitute(suffix, math)
            .as_polynom()
            .as_math();

        Math::Braces(Braces {
            inner: new_math,
            exponent: Some(Box::new(new_exponent)),
        })
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        let mut suf: Vec<String> = vec![];
        suf.extend_from_slice(&self.inner.get_all_suffixes());
        if let Some(exp) = self.exponent.clone() {
            suf.extend_from_slice(&exp.get_all_suffixes());
        }
        suf.sort();
        suf.dedup();
        suf
    }
}

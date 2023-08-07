use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::variable::Variable;
use crate::math::descrete::Descrete;
use crate::math::operator::algebra::{Operations as AlgebraOperatons, Operator as AlgebraOperator};
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenericsAsVariable};

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

impl AlgebraOperatons for Fraction {
    fn addition(&self, other: &Fraction) -> Math {
        if self.denominator.to_tex() == other.denominator.to_tex() {
            if self.whole.is_none() || self.whole.unwrap_or(dec!(0)).is_zero() {
                return Math::Fraction(Fraction {
                    whole: None,
                    numerator: Box::new(self.numerator.add(&other.numerator)),
                    denominator: self.denominator.clone(),
                });
            } else {
                todo!("convert whole to fraction before adding fractions")
            }
        }

        Math::Fraction(self.expand(*other.denominator.clone()))
            .add(&Math::Fraction(other.expand(*self.denominator.clone())))
    }

    fn subtraction(&self, other: &Fraction) -> Math {
        if self.denominator.to_tex() == other.denominator.to_tex() {
            if self.whole.is_none() || self.whole.unwrap_or(dec!(0)).is_zero() {
                return Math::Fraction(Fraction {
                    whole: None,
                    numerator: Box::new(self.numerator.sub(&other.numerator)),
                    denominator: self.denominator.clone(),
                });
            } else {
                todo!("convert whole to fraction before subtraction fractions")
            }
        }
        Math::Fraction(self.expand(*other.denominator.clone()))
            .sub(&Math::Fraction(other.expand(*self.denominator.clone())))
    }

    fn multiplication(&self, other: &Fraction) -> Math {
        Math::Fraction(Fraction {
            whole: None,
            numerator: Box::new(self.numerator.mul(&other.numerator)),
            denominator: Box::new(self.denominator.mul(&other.denominator)),
        })
    }

    fn division(&self, other: &Fraction) -> Math {
        self.multiplication(&other.inverse())
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.addition(f),
            Math::Variable(v) => self.addition(&v.as_fraction()),
            Math::Polynom(p) => self.addition(&p.as_fraction()),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.subtraction(f),
            Math::Variable(v) => self.subtraction(&v.as_fraction()),
            Math::Polynom(p) => self.subtraction(&p.as_fraction()),
            _ => todo!(),
        }
    }

    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.multiplication(f),
            Math::Variable(v) => self.multiplication(&v.as_fraction()),
            Math::Polynom(p) => self.multiplication(&p.as_fraction()),
            _ => todo!(),
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.division(f),
            Math::Variable(v) => self.division(&v.as_fraction()),
            Math::Polynom(p) => self.division(&p.as_fraction()),
            _ => todo!(),
        }
    }

    fn negative(&self) -> Math {
        self.mul(&Math::Variable((-1 as i32).as_variable()))
    }

    fn simplify(&self) -> Math {
        if self.denominator.to_tex() == "1" {
            return *self.numerator.clone();
        }
        if self.denominator.to_tex() == self.numerator.to_tex() {
            return Math::Variable(1.as_variable());
        }
        if let Math::Variable(num) = *self.clone().numerator {
            if let Math::Variable(den) = *self.clone().denominator {
                let lcd = num.lowest_common_denominator(&den);
                if lcd.to_tex() != "1" {
                    return Fraction {
                        whole: self.whole.clone(),
                        numerator: Box::new(num.division(&lcd)),
                        denominator: Box::new(den.division(&lcd)),
                    }
                    .simplify();
                }
                if num.is_divisable(&den) {
                    return num.division(&den);
                }
            }
        }

        return Math::Fraction(self.clone());
    }

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        Math::Fraction(Fraction {
            whole: self.whole.clone(),
            numerator: Box::new(self.numerator.substitute(suffix.clone(), math.clone())),
            denominator: Box::new(self.denominator.substitute(suffix.clone(), math.clone())),
        })
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        let mut suf: Vec<String> = vec![];
        suf.extend(self.denominator.get_all_suffixes());
        suf.extend(self.numerator.get_all_suffixes());
        suf.sort();
        suf.dedup();
        suf
    }
}

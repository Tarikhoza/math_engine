use crate::castable::Castable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::operations::Operations as AlgebraOperatons;
use crate::math::descrete::Descrete;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable};

use rust_decimal_macros::dec;

impl Simplifiable for Fraction {
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
                        whole: self.whole,
                        numerator: Box::new(num.div_self(&lcd)),
                        denominator: Box::new(den.div_self(&lcd)),
                    }
                    .simplify();
                }
                if num.is_divisable(&den) {
                    return num.div_self(&den);
                }
            }
        }
        Fraction {
            whole: self.whole,
            numerator: Box::new(self.numerator.simplify()),
            denominator: Box::new(self.denominator.simplify()),
        }
        .as_math()
    }
}
impl AlgebraOperatons for Fraction {
    fn add_self(&self, other: &Fraction) -> Math {
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

    fn sub_self(&self, other: &Fraction) -> Math {
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

    fn mul_self(&self, other: &Fraction) -> Math {
        Math::Fraction(Fraction {
            whole: None,
            numerator: Box::new(self.numerator.mul(&other.numerator)),
            denominator: Box::new(self.denominator.mul(&other.denominator)),
        })
    }

    fn div_self(&self, other: &Fraction) -> Math {
        self.mul_self(&other.inverse())
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.add_self(f),
            Math::Variable(v) => self.add_self(&v.as_fraction()),
            Math::Polynom(p) => self.add_self(&p.as_fraction()),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.sub_self(f),
            Math::Variable(v) => self.sub_self(&v.as_fraction()),
            Math::Polynom(p) => self.sub_self(&p.as_fraction()),
            _ => todo!(),
        }
    }

    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.mul_self(f),
            Math::Variable(v) => self.mul_self(&v.as_fraction()),
            Math::Polynom(p) => self.mul_self(&p.as_fraction()),
            _ => todo!(),
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.div_self(f),
            Math::Variable(v) => self.div_self(&v.as_fraction()),
            Math::Polynom(p) => self.div_self(&p.as_fraction()),
            _ => todo!(),
        }
    }

    fn negative(&self) -> Math {
        self.mul(&Math::Variable((-1).as_variable()))
    }

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        Math::Fraction(Fraction {
            whole: self.whole,
            numerator: Box::new(self.numerator.substitute(suffix, math.clone())),
            denominator: Box::new(self.denominator.substitute(suffix, math)),
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

use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::operator::algebra::{Operations as AlgebraOperatons, Operator as AlgebraOperator};
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenericsAsVariable};

impl AlgebraOperatons for Fraction {
    fn addition(&self, other: &Fraction) -> Math {
        if self.denominator.to_tex() == other.denominator.to_tex() {
            return Math::Fraction(Fraction {
                numerator: Box::new(self.numerator.add(&other.numerator)),
                denominator: self.denominator.clone(),
            });
        }

        Math::Fraction(self.expand(*other.denominator.clone()))
            .add(&Math::Fraction(other.expand(*self.denominator.clone())))
    }

    fn subtraction(&self, other: &Fraction) -> Math {
        if self.denominator.to_tex() == other.denominator.to_tex() {
            return Math::Fraction(Fraction {
                numerator: Box::new(self.numerator.sub(&other.numerator)),
                denominator: self.denominator.clone(),
            });
        }
        Math::Fraction(self.expand(*other.denominator.clone()))
            .sub(&Math::Fraction(other.expand(*self.denominator.clone())))
    }

    fn multiplication(&self, other: &Fraction) -> Math {
        Math::Fraction(Fraction {
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
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.subtraction(f),
            _ => todo!(),
        }
    }

    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.multiplication(f),
            _ => todo!(),
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.division(f),
            _ => todo!(),
        }
    }

    fn negative(&self) -> Math {
        self.mul(&Math::Variable((-1 as i32).as_variable()))
    }

    fn simplify(&self) -> Math {
        todo!();
    }
    fn substitute(&self, suffix: String, math: Math) -> Math {
        Math::Fraction(Fraction {
            numerator: Box::new(self.numerator.substitute(suffix.clone(), math.clone())),
            denominator: Box::new(self.denominator.substitute(suffix.clone(), math.clone())),
        })
    }
}

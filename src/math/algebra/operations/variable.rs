use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::infinity::Infinity;
use crate::math::algebra::operations::{
    Operations as AlgebraOperatons, Operator as AlgebraOperator,
};
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::undefined::Undefined;
use crate::math::algebra::variable::Variable;
use crate::math::Math;

use crate::castable::Castable;
use crate::math::simplifiable::Simplifiable;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable};

use rust_decimal_macros::dec;

impl Simplifiable for Variable {
    fn simplify(&self) -> Math {
        let mut new = self.clone();
        if let Some(exp) = self.exponent.clone() {
            new.exponent = Some(Box::new(exp.simplify()));
        }
        if new.value.is_sign_negative() {
            let (sign, var) = new.split_operator();
            return Math::Polynom(Polynom {
                parts: vec![sign.as_polynom_part(), var.as_math().as_polynom_part()],
            });
        }
        new.apply_exponent()
    }
}

impl AlgebraOperatons for Variable {
    fn add_self(&self, other: &Variable) -> Math {
        //if suffix and exponent are the same
        if self.suffix == other.suffix
            && self.get_exponent().to_tex() == other.get_exponent().to_tex()
        {
            let ret = Variable {
                value: self.value + other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent())),
            };
            if ret.value.is_sign_negative() {
                let (sign, var) = ret.split_operator();
                return Math::Polynom(Polynom {
                    parts: vec![sign.as_polynom_part(), var.as_math().as_polynom_part()],
                });
            } else {
                return Math::Variable(ret);
            }
        }
        Math::Polynom(Polynom {
            parts: vec![
                self.as_math().as_polynom_part(),
                AlgebraOperator::Addition.as_polynom_part(),
                other.as_math().as_polynom_part(),
            ],
        })
    }

    fn sub_self(&self, other: &Variable) -> Math {
        //if suffix and exponent are the same
        if self.suffix == other.suffix
            && self.get_exponent().to_tex() == other.get_exponent().to_tex()
        {
            let ret = Variable {
                value: self.value - other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent())),
            };

            if ret.value.is_sign_negative() {
                let (sign, var) = ret.split_operator();
                return Math::Polynom(Polynom {
                    parts: vec![sign.as_polynom_part(), var.as_math().as_polynom_part()],
                });
            } else {
                return Math::Variable(ret);
            }
        }
        Math::Polynom(Polynom {
            parts: vec![
                self.as_math().as_polynom_part(),
                AlgebraOperator::Subtraction.as_polynom_part(),
                other.as_math().as_polynom_part(),
            ],
        })
    }

    fn mul_self(&self, other: &Variable) -> Math {
        //TODO clean up repetition and maybe rewrite
        //if suffix are empty
        if self.suffix == *"" && other.suffix == *"" {
            if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: None,
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(other.get_exponent())),
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(self.get_exponent().mul(&other.get_exponent()))),
                });
            }
        }
        //if one suffix is empty and one is some
        else if (self.suffix == *"" && other.suffix != *"")
            || (self.suffix != *"" && other.suffix == *"")
        {
            if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: None,
                });
            }
            if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent())),
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(other.get_exponent())),
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent().mul(&other.get_exponent()))),
                });
            }
        }
        //if both suffix are some
        else if self.suffix == other.suffix && self.suffix != String::new() {
            return Math::Variable(Variable {
                value: self.value * other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(
                    (self.get_exponent().add(&other.get_exponent())).simplify(),
                )),
            });
        }
        let left = self.split_operator();
        let right = other.split_operator();
        let sign = left.0.morph(right.0);

        if sign == AlgebraOperator::Subtraction {
            return Polynom {
                parts: vec![
                    self.negative().as_polynom_part(),
                    AlgebraOperator::InvMulti.as_polynom_part(),
                    other.as_math().as_polynom_part(),
                ],
            }
            .as_math();
        }
        Math::Polynom(Polynom {
            parts: vec![
                self.as_math().as_polynom_part(),
                AlgebraOperator::InvMulti.as_polynom_part(),
                other.as_math().as_polynom_part(),
            ],
        })
    }
    fn div_self(&self, other: &Variable) -> Math {
        //Handle 0/0 and 0/x
        if self.value.is_zero() && other.value.is_zero() {
            return Math::Undefined(Undefined {});
        }

        //Handle x/0
        if !self.value.is_zero() && other.value.is_zero() {
            return Math::Infinity(Infinity {});
        }

        //TODO clean up repetition and maybe rewrite
        //if suffix are empty
        if self.suffix == *"" && other.suffix == *"" {
            if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: None,
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(other.get_exponent())),
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(self.get_exponent().div(&other.get_exponent()))),
                });
            }
        }
        //if one suffix is empty and the second is some
        else if (self.suffix == *"" && other.suffix != *"")
            || (self.suffix != *"" && other.suffix == *"")
        {
            if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: None,
                });
            }
            if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent())),
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(other.get_exponent())),
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent().div(&other.get_exponent()))),
                });
            }
        }
        //if one suffix is some and the second is empty
        else if self.suffix == other.suffix && self.suffix != *"" {
            return Math::Variable(Variable {
                value: self.value / other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent().sub(&other.get_exponent()))),
            });
        }
        Math::Polynom(Polynom {
            parts: vec![
                self.as_math().as_polynom_part(),
                AlgebraOperator::Division.as_polynom_part(),
                other.as_math().as_polynom_part(),
            ],
        })
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.as_polynom().add(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.add_self(v),
            Math::Braces(b) => self.add(&b.simplify()),
            Math::Fraction(f) => self.as_fraction().add_self(f),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.as_polynom().sub(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.sub_self(v),
            Math::Braces(b) => self.sub(&b.simplify()),
            Math::Fraction(f) => self.as_fraction().sub_self(f),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.as_polynom().mul(&Math::Polynom(p.clone())),
            Math::Braces(b) => self.mul(&b.simplify()),
            Math::Variable(v) => self.mul_self(v),
            Math::Fraction(f) => self.as_fraction().mul_self(f),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            //  Math::Polynom(p)  => self.as_polynom()*Math::Polynom(p),
            //
            Math::Fraction(f) => self.as_fraction().div_self(f),
            Math::Variable(v) => self.div_self(v),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn negative(&self) -> Math {
        Math::Variable(Variable {
            value: -self.value,
            suffix: self.suffix.clone(),
            exponent: Some(Box::new(self.get_exponent())),
        })
    }

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        if self.get_all_suffixes().contains(&suffix.to_string()) {
            if self.suffix == suffix {
                return Polynom {
                    parts: vec![
                        self.value.as_variable().as_math().as_polynom_part(),
                        AlgebraOperator::Multiplication.as_polynom_part(),
                        Math::Braces(Braces {
                            inner: Box::new(math.clone()),
                            exponent: Some(Box::new(
                                self.get_exponent()
                                    .substitute(suffix, math)
                                    .as_polynom()
                                    .as_math(),
                            )),
                        })
                        .as_polynom_part(),
                    ],
                }
                .as_math();
            } else {
                let mut ret = self.clone();
                ret.exponent = Some(Box::new(
                    self.get_exponent()
                        .substitute(suffix, math)
                        .as_polynom()
                        .as_math(),
                ));
                return ret.as_math();
            }
        }
        self.as_math()
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        let mut suffixes: Vec<String> = Vec::new();
        if !self.suffix.is_empty() {
            suffixes.push(self.suffix.clone());
        }
        if self.exponent.is_some() {
            let exp = self.get_exponent();
            if exp.to_tex() != "1" && exp.to_tex() != "1.0" {
                suffixes.extend_from_slice(&exp.get_all_suffixes())
            }
        }
        suffixes
    }
}

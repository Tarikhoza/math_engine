use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::infinity::Infinity;
use crate::math::algebra::operations::{
    Operations as AlgebraOperatons, Operator as AlgebraOperator,
};
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::undefined::Undefined;
use crate::math::algebra::variable::Variable;
use crate::math::operator::Operator;
use crate::math::Math;

use crate::math::simplifiable::Simplifiable;
use crate::parser::{Parsable, ParsableGenerics, ParsableGenericsAsVariable, Parser};
use crate::solver::step::{DetailedOperator, Step};

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

impl Simplifiable for Variable {
    fn simplify(&self) -> Math {
        self.apply_exponent()
    }
}

impl AlgebraOperatons for Variable {
    fn add_self(&self, other: &Variable) -> Math {
        //if suffix and exponent are the same
        if self.suffix == other.suffix
            && self.get_exponent().to_tex() == other.get_exponent().to_tex()
        {
            return Math::Variable(Variable {
                value: self.value + other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent())),
                #[cfg(feature = "step-tracking")]
                step: Step::step(
                    Math::Variable(self.clone()),
                    Some(Math::Variable(other.clone())),
                    Operator::Algebra(AlgebraOperator::Addition),
                    String::from("Addition of two variables"),
                ),
            });
        }
        Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other.clone())],
            operators: vec![Operator::Algebra(AlgebraOperator::Addition)],
            #[cfg(feature = "step-tracking")]
            step: None,
        })
    }

    fn sub_self(&self, other: &Variable) -> Math {
        //if suffix and exponent are the same
        if self.suffix == other.suffix
            && self.get_exponent().to_tex() == other.get_exponent().to_tex()
        {
            return Math::Variable(Variable {
                value: self.value - other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent())),
                #[cfg(feature = "step-tracking")]
                step: Step::step(
                    Math::Variable(self.clone()),
                    Some(Math::Variable(other.clone())),
                    Operator::Algebra(AlgebraOperator::Subtraction),
                    String::from("Subtraction of two variables"),
                ),
            });
        }
        Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other.clone())],
            operators: vec![Operator::Algebra(AlgebraOperator::Subtraction)],
            #[cfg(feature = "step-tracking")]
            step: None,
        })
    }

    fn mul_self(&self, other: &Variable) -> Math {
        #[cfg(feature = "step-tracking")]
        let mul_two_var = Step::step(
            Math::Variable(self.clone()),
            Some(Math::Variable(other.clone())),
            Operator::Algebra(AlgebraOperator::Multiplication),
            String::from("Multiplication of two variables"),
        );

        #[cfg(feature = "step-tracking")]
        let mul_two_var_one_suf = Step::step(
            Math::Variable(self.clone()),
            Some(Math::Variable(other.clone())),
            Operator::Algebra(AlgebraOperator::Multiplication),
            String::from("Multiplication of two variables(one without suffix, one with)"),
        );

        //if suffix are empty
        if self.suffix == *"" && other.suffix == *"" {
            if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: None,
                    #[cfg(feature = "step-tracking")]
                    step: mul_two_var,
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(other.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: mul_two_var,
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(self.get_exponent().mul(&other.get_exponent()))),
                    #[cfg(feature = "step-tracking")]
                    step: mul_two_var,
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
                    #[cfg(feature = "step-tracking")]
                    step: mul_two_var_one_suf,
                });
            }
            if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: mul_two_var_one_suf,
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(other.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: mul_two_var_one_suf,
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent().mul(&other.get_exponent()))),
                    #[cfg(feature = "step-tracking")]
                    step: mul_two_var_one_suf,
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
                #[cfg(feature = "step-tracking")]
                step: Step::step(
                    Math::Variable(self.clone()),
                    Some(Math::Variable(other.clone())),
                    Operator::Algebra(AlgebraOperator::Multiplication),
                    String::from("Multiplication of two variables(both with suffxes)"),
                ),
            });
        }
        let left = self.split_operator();
        let right = other.split_operator();
        let sign = left.0.morph(right.0);

        if sign == Operator::Algebra(AlgebraOperator::Subtraction) {
            return Math::Polynom(Polynom {
                factors: vec![self.negative(), Math::Variable(other.clone())],
                operators: vec![Operator::Algebra(AlgebraOperator::InvMulti)],
                #[cfg(feature = "step-tracking")]
                step: Step::step(
                    Math::Variable(self.clone()),
                    Some(Math::Variable(other.clone())),
                    Operator::Detail(DetailedOperator::GroupTogether),
                    String::from("Grouping two variables together"),
                ),
            });
        }
        Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other.clone())],
            operators: vec![Operator::Algebra(AlgebraOperator::InvMulti)],
            #[cfg(feature = "step-tracking")]
            step: Step::step(
                Math::Variable(self.clone()),
                Some(Math::Variable(other.clone())),
                Operator::Detail(DetailedOperator::GroupTogether),
                String::from("Grouping two variables together"),
            ),
        })
    }
    fn div_self(&self, other: &Variable) -> Math {
        //Handle 0/0 and 0/x
        if self.value.is_zero() {
            if other.value.is_zero() {
                return Math::Undefined(Undefined {});
            }
            return Math::default();
        }

        //Handle x/0
        if self.value != dec!(0) && other.value == dec!(0) {
            return Math::Infinity(Infinity { minus: false });
        }

        #[cfg(feature = "step-tracking")]
        let div_two_var = Step::step(
            Math::Variable(self.clone()),
            Some(Math::Variable(other.clone())),
            Operator::Algebra(AlgebraOperator::Division),
            String::from("Division of two variables"),
        );
        #[cfg(feature = "step-tracking")]
        let div_two_var_one_suf = Step::step(
            Math::Variable(self.clone()),
            Some(Math::Variable(other.clone())),
            Operator::Algebra(AlgebraOperator::Division),
            String::from("Division of two variables(one without suffix, one with)"),
        );
        //if suffix are empty
        if self.suffix == *"" && other.suffix == *"" {
            if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: None,
                    #[cfg(feature = "step-tracking")]
                    step: div_two_var,
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(other.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: div_two_var,
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(self.get_exponent().div(&other.get_exponent()))),
                    #[cfg(feature = "step-tracking")]
                    step: div_two_var,
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
                    #[cfg(feature = "step-tracking")]
                    step: div_two_var_one_suf,
                });
            }
            if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: div_two_var_one_suf,
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(other.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: div_two_var_one_suf,
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent().div(&other.get_exponent()))),
                    #[cfg(feature = "step-tracking")]
                    step: div_two_var_one_suf,
                });
            }
        }
        //if one suffix is some and the second is empty
        else if self.suffix == other.suffix && self.suffix != *"" {
            return Math::Variable(Variable {
                value: self.value / other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent().sub(&other.get_exponent()))),
                #[cfg(feature = "step-tracking")]
                step: div_two_var_one_suf,
            });
        }
        Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other.clone())],
            operators: vec![Operator::Algebra(AlgebraOperator::Division)],
            #[cfg(feature = "step-tracking")]
            step: None,
        })
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.as_polynom().add(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.add_self(v),
            Math::Braces(b) => self.add(&b.simplify()),
            Math::Fraction(f) => self.as_fraction().add_self(f),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.as_polynom().sub(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.sub_self(v),
            Math::Braces(b) => self.sub(&b.simplify()),
            Math::Fraction(f) => self.as_fraction().sub_self(f),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.as_polynom().mul(&Math::Polynom(p.clone())),
            Math::Braces(b) => self.mul(&b.simplify()),
            Math::Variable(v) => self.mul_self(v),
            Math::Fraction(f) => self.as_fraction().mul_self(f),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            //  Math::Polynom(p)  => self.as_polynom()*Math::Polynom(p),
            //
            Math::Fraction(f) => self.as_fraction().div_self(f),
            Math::Variable(v) => self.div_self(v),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn negative(&self) -> Math {
        match &self.exponent {
            Some(_e) => Math::Variable(Variable {
                value: -self.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent())),
                #[cfg(feature = "step-tracking")]
                step: Step::step(
                    Math::Variable(self.clone()),
                    None,
                    Operator::Detail(crate::solver::step::DetailedOperator::Negate),
                    String::from("Negate variable with *-1"),
                ),
            }),
            _no_exp => Math::Variable(Variable {
                value: -self.value,
                suffix: self.suffix.clone(),
                exponent: None,
                #[cfg(feature = "step-tracking")]
                step: Step::step(
                    Math::Variable(self.clone()),
                    None,
                    Operator::Detail(crate::solver::step::DetailedOperator::Negate),
                    String::from("Negate variable with *-1"),
                ),
            }),
        }
    }

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        if self.get_all_suffixes().contains(&suffix.to_string()) {
            return Math::Polynom(Polynom {
                factors: vec![
                    self.value
                        .to_f64()
                        .expect("failed pasing dec as f64")
                        .parse_math()
                        .expect("failed parsing dec as math"),
                    Math::Braces(Braces {
                        math: Box::new(math.clone()),
                        exponent: Some(Box::new(self.get_exponent().substitute(suffix, math))),
                    }),
                ],
                operators: vec![Operator::Algebra(AlgebraOperator::Multiplication)],
                #[cfg(feature = "step-tracking")]
                step: None,
            });
        }
        Math::Variable(self.clone())
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        let mut suffixes: Vec<String> = Vec::new();
        dbg!("{}", self.to_tex());
        if let Some(exp) = &self.exponent {
            suffixes.extend(exp.get_all_suffixes())
        }
        if !self.suffix.is_empty() {
            suffixes.push(self.suffix.clone());
        }

        suffixes
    }
}

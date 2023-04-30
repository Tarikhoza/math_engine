use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::undefined::Undefined;
use crate::math::algebra::variable::Variable;
use crate::math::operator::algebra::{Operations as AlgebraOperatons, Operator as AlgebraOperator};
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::Parsable;
use crate::solver::step::{DetailedOperator, Step};

impl AlgebraOperatons for Variable {
    fn addition(&self, other: &Variable) -> Math {
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

    fn subtraction(&self, other: &Variable) -> Math {
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

    fn multiplication(&self, other: &Variable) -> Math {
        //if suffix are empty
        if self.suffix == *"" && other.suffix == *"" {
            if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: None,
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Multiplication),
                        String::from("Multiplication of two variables"),
                    ),
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(other.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Multiplication),
                        String::from("Multiplication of two variables"),
                    ),
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(self.get_exponent().mul(&other.get_exponent()))),
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Multiplication),
                        String::from("Multiplication of two variables"),
                    ),
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
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Multiplication),
                        String::from(
                            "Multiplication of two variables(one without suffix, one with)",
                        ),
                    ),
                });
            }
            if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent())),

                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Multiplication),
                        String::from(
                            "Multiplication of two variables(one without suffix, one with)",
                        ),
                    ),
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(other.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Multiplication),
                        String::from(
                            "Multiplication of two variables(one without suffix, one with)",
                        ),
                    ),
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent().mul(&other.get_exponent()))),
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Multiplication),
                        String::from(
                            "Multiplication of two variables(one without suffix, one with)",
                        ),
                    ),
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

    fn division(&self, other: &Variable) -> Math {
        //if suffix are empty
        if self.suffix == *"" && other.suffix == *"" {
            if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: None,
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Division),
                        String::from("Division of two variables"),
                    ),
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(other.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Division),
                        String::from("Division of two variables"),
                    ),
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(self.get_exponent().div(&other.get_exponent()))),
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Division),
                        String::from("Division of two variables"),
                    ),
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
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Division),
                        String::from("Division of two variables(one without suffix, one with)"),
                    ),
                });
            }
            if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Division),
                        String::from("Division of two variables(one without suffix, one with)"),
                    ),
                });
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(other.get_exponent())),
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Division),
                        String::from("Division of two variables(one without suffix, one with)"),
                    ),
                });
            } else if self.get_exponent().to_tex() != "1" && other.get_exponent().to_tex() != "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: format!("{}{}", self.suffix, other.suffix),
                    exponent: Some(Box::new(self.get_exponent().div(&other.get_exponent()))),
                    #[cfg(feature = "step-tracking")]
                    step: Step::step(
                        Math::Variable(self.clone()),
                        Some(Math::Variable(other.clone())),
                        Operator::Algebra(AlgebraOperator::Division),
                        String::from("Division of two variables(one without suffix, one with)"),
                    ),
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
                step: Step::step(
                    Math::Variable(self.clone()),
                    Some(Math::Variable(other.clone())),
                    Operator::Algebra(AlgebraOperator::Division),
                    String::from("Division of two variables(one without suffix, one with)"),
                ),
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
        //        println!("{}+{}", self.to_tex(), _rhs.to_tex());
        match rhs {
            Math::Polynom(p) => self.as_polynom().add(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.addition(v),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn sub(&self, rhs: &Math) -> Math {
        //        println!("{}-{}", self.to_tex(), _rhs.to_tex());
        match rhs {
            Math::Polynom(p) => self.as_polynom().sub(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.subtraction(v),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }
    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.as_polynom().mul(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.multiplication(v),
            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            //  Math::Polynom(p)  => self.as_polynom()*Math::Polynom(p),
            Math::Variable(v) => self.division(v),
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

    fn simplify(&self) -> Math {
        //TODO: apply_exponent
        todo!()
    }
}

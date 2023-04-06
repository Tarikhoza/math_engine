use std::ops;

use fancy_regex::Regex;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::math::{BasicOperations, Math, Operators};
use crate::parser::{Parsable, Parser};
use crate::polynom::Polynom;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Decimal,
    pub suffix: String,
    pub exponent: Option<Box<Math>>,
}

fn ascii_score(s: &str) -> u32 {
    let mut score = 0;
    for (i, c) in s.chars().enumerate() {
        score = c.to_digit(10).unwrap_or(1000) / (i + 1) as u32;
    }
    score
}

impl BasicOperations for Variable {
    #[must_use]
    fn addition(&self, other: Variable) -> Math {
        //if suffix and exponent are the same
        if self.suffix == other.suffix
            && self.get_exponent().to_tex() == other.get_exponent().to_tex()
        {
            return Math::Variable(Variable {
                value: self.value + other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent())),
            });
        }
        Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other)],
            operators: vec![Operators::Addition],
        })
    }

    #[must_use]
    fn subtraction(&self, other: Variable) -> Math {
        //if suffix and exponent are the same
        if self.suffix == other.suffix
            && self.get_exponent().to_tex() == other.get_exponent().to_tex()
        {
            return Math::Variable(Variable {
                value: self.value - other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent())),
            });
        }
        Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other)],
            operators: vec![Operators::Subtraction],
        })
    }

    #[must_use]
    fn multiplication(&self, other: Variable) -> Math {
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
                    exponent: Some(Box::new(self.get_exponent() * other.get_exponent())),
                });
            }
        }
        //if one suffix is empty and the second is some
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
                    exponent: Some(Box::new(self.get_exponent() * other.get_exponent())),
                });
            }
        }
        //if both suffix are some
        else if self.suffix == other.suffix && self.suffix != *"" {
            return Math::Variable(Variable {
                value: self.value * other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(
                    (self.get_exponent() + other.get_exponent()).simplify(),
                )),
            });
        }

        Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other)],
            operators: vec![Operators::InvMulti],
        })
    }

    #[must_use]
    fn division(&self, other: Variable) -> Math {
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
                    exponent: Some(Box::new(self.get_exponent() / other.get_exponent())),
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
                    exponent: Some(Box::new(self.get_exponent() / other.get_exponent())),
                });
            }
        }
        //if one suffix is some and the second is empty
        else if self.suffix == other.suffix && self.suffix != *"" {
            return Math::Variable(Variable {
                value: self.value / other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent() - other.get_exponent())),
            });
        }

        Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other)],
            operators: vec![Operators::Division],
        })
    }

    #[must_use]
    fn negative(&self) -> Math {
        match &self.exponent {
            Some(_e) => Math::Variable(Variable {
                value: -self.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent())),
            }),
            _no_exp => Math::Variable(Variable {
                value: -self.value,
                suffix: self.suffix.clone(),
                exponent: None,
            }),
        }
    }

    #[must_use]
    fn simplify(&self) -> Math {
        //TODO: apply_exponent
        todo!()
    }
}

impl Variable {
    #[must_use]
    pub fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1),
                suffix: String::new(),
                exponent: None,
            }),
            Some(e) => *e.clone(),
        }
    }

    #[must_use]
    pub fn apply_exponent(&self) -> Math {
        todo!()
    }

    #[must_use]
    pub fn as_polynom(&self) -> Polynom {
        Polynom {
            factors: vec![Math::Variable(self.clone())],
            operators: vec![],
        }
    }

    #[must_use]
    pub fn split_operator(&self) -> (Operators, Math) {
        if self.value < dec!(0) {
            return (Operators::Subtraction, self.negative());
        }
        (Operators::Addition, Math::Variable(self.clone()))
    }
    #[must_use]
    pub fn sort_score(&self) -> u32 {
        u32::MAX - (ascii_score(&self.suffix) + ascii_score(&self.get_exponent().to_tex()))
    }
}

impl Parsable for Variable {
    #[must_use]
    fn to_tex(&self) -> String {
        let mut val = self.value.normalize().to_string();
        if (val == "1.0" || val == "1") && !self.suffix.is_empty() {
            val = String::new();
        }
        match &self.exponent {
            Some(has_exp) => match &has_exp.to_tex()[..] {
                "1.0" | "1" => format!("{}{}", val, self.suffix),
                exp => format!("{}{}^{{{}}}", val, self.suffix, exp),
            },

            _no_exp => format!("{}{}", val, self.suffix),
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(-?\d+(?=\w*)(\.\d+)?|\d*(?=\w+)(\.\d+)?)(\w*)(\^(\{.+))?")
                    .unwrap_or_else(|e| {
                        panic!("Failed to compile regex for braces: {e}");
                    });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");
        let mut value = captures.get(1).map_or("", |m| m.as_str()).to_string();
        let suffix = captures.get(4).map_or("", |m| m.as_str()).to_string();
        let exponent_str =
            Parser::extract_brace(&captures.get(6).map_or("", |m| m.as_str()), '{', '}');

        if value.is_empty() {
            value = "1.0".to_string();
        }

        let exponent: Option<Box<Math>> = if exponent_str.is_empty() {
            None
        } else {
            Some(Box::new(Parser::new(&exponent_str).parse()?))
        };

        Ok(Math::Variable(Variable {
            value: Decimal::from_str(&value).unwrap_or(dec!(1.0)),
            suffix,
            exponent,
        }))
    }

    #[must_use]
    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(-?\d+(?=\w?)(\.\d+)?|\d*(?=\w?)(\.\d+)?)(\w?)(\^\{(.+)\})?")
                    .unwrap_or_else(|e| {
                        panic!("Failed to compile regex for variable: {e}");
                    });
        }

        if let Ok(Some(f)) = RE.find(&tex) {
            let f_str = f.as_str().to_string();
            if !f_str.is_empty() {
                return Some(f_str);
            }
        }
        None
    }
}

impl ops::Add<Math> for Variable {
    type Output = Math;
    fn add(self, rhs: Math) -> Math {
        //        println!("{}+{}", self.to_tex(), _rhs.to_tex());
        match rhs {
            Math::Polynom(p) => self.as_polynom() + Math::Polynom(p),
            Math::Variable(v) => self.addition(v),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Variable {
    type Output = Math;
    fn sub(self, rhs: Math) -> Math {
        //        println!("{}-{}", self.to_tex(), _rhs.to_tex());
        match rhs {
            Math::Polynom(p) => self.as_polynom() - Math::Polynom(p),
            Math::Variable(v) => self.subtraction(v),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Variable {
    type Output = Math;
    fn mul(self, rhs: Math) -> Math {
        match rhs {
            Math::Variable(v) => self.multiplication(v),
            Math::Polynom(p) => self.as_polynom() * Math::Polynom(p),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Variable {
    type Output = Math;
    fn div(self, rhs: Math) -> Math {
        match rhs {
            //  Math::Polynom(p)  => self.as_polynom()*Math::Polynom(p),
            Math::Variable(v) => self.division(v),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

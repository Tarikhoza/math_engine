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

fn ascii_score(s: String) -> u32 {
    let mut score = 0;
    for (i, c) in s.chars().enumerate() {
        score = (c.to_digit(10).unwrap_or(1000) / (i + 1) as u32) as u32;
    }
    score
}

impl BasicOperations for Variable {
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
        return Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other).clone()],
            operators: vec![Operators::Addition],
        });
    }

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
        return Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other).clone()],
            operators: vec![Operators::Subtraction],
        });
    }

    fn multiplication(&self, other: Variable) -> Math {
        //if suffix are empty
        if self.suffix == "".to_string() && other.suffix == "".to_string() {
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
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value * other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(self.get_exponent() * other.get_exponent())),
                });
            }
        }
        //if one suffix is empty and the second is some
        else if (self.suffix == "".to_string() && other.suffix != "".to_string())
            || (self.suffix != "".to_string() && other.suffix == "".to_string())
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
        else if self.suffix == other.suffix && self.suffix != "".to_string() {
            return Math::Variable(Variable {
                value: self.value * other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(
                    (self.get_exponent() + other.get_exponent()).simplify(),
                )),
            });
        }

        return Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other.clone())],
            operators: vec![Operators::InvMulti],
        });
    }

    fn division(&self, other: Variable) -> Math {
        //if suffix are empty
        if self.suffix == "".to_string() && other.suffix == "".to_string() {
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
            } else if self.get_exponent().to_tex() == "1" && other.get_exponent().to_tex() == "1" {
                return Math::Variable(Variable {
                    value: self.value / other.value,
                    suffix: self.suffix.clone(),
                    exponent: Some(Box::new(self.get_exponent() / other.get_exponent())),
                });
            }
        }
        //if one suffix is empty and the second is some
        else if (self.suffix == "".to_string() && other.suffix != "".to_string())
            || (self.suffix != "".to_string() && other.suffix == "".to_string())
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
        else if self.suffix == other.suffix && self.suffix != "".to_string() {
            return Math::Variable(Variable {
                value: self.value / other.value,
                suffix: self.suffix.clone(),
                exponent: Some(Box::new(self.get_exponent() - other.get_exponent())),
            });
        }

        return Math::Polynom(Polynom {
            factors: vec![Math::Variable(self.clone()), Math::Variable(other.clone())],
            operators: vec![Operators::Division],
        });
    }

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

    fn simplify(&self) -> Math {
        //TODO: apply_exponent
        todo!()
    }
}

impl Variable {
    pub fn new(tex: &str) -> Math {
        Variable::from_tex(tex.to_string())
    }

    pub fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1),
                suffix: "".to_string(),
                exponent: None,
            }),
            Some(e) => *e.clone(),
        }
    }

    pub fn apply_exponent(&self) -> Math {
        todo!()
    }

    pub fn as_polynom(&self) -> Polynom {
        Polynom {
            factors: vec![Math::Variable(self.clone())],
            operators: vec![],
        }
    }

    pub fn split_operator(&self) -> (Operators, Math) {
        if self.value < dec!(0) {
            return (Operators::Subtraction, self.negative());
        }
        return (Operators::Addition, Math::Variable(self.clone()));
    }
    pub fn sort_score(&self) -> u32 {
        let score = u32::MAX
            - (ascii_score(self.suffix.clone()) + ascii_score(self.get_exponent().to_tex()));
        score
    }
}

impl Parsable for Variable {
    fn to_tex(&self) -> String {
        let mut val = self.value.normalize().to_string();
        if (val == "1.0" || val == "1") && !self.suffix.is_empty() {
            val = "".to_string();
        }
        match &self.exponent {
            Some(has_exp) => match &has_exp.to_tex()[..] {
                "1.0" | "1" => format!("{}{}", val, self.suffix),
                exp => format!("{}{}^{{{}}}", val, self.suffix, exp),
            },

            _no_exp => format!("{}{}", val, self.suffix),
        }
    }

    fn from_tex(tex: String) -> Math {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(-?\d+(?=\w*)(\.\d+)?|\d*(?=\w+)(\.\d+)?)(\w*)(\^(\{.+))?").unwrap();
        }
        let result = RE.captures(&tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");
        let mut value = captures
            .get(1)
            .map(|m| m.as_str())
            .unwrap_or("1.0")
            .to_string();
        let suffix = captures
            .get(4)
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();
        let exponent_str = Parser::extract_brace(
            captures
                .get(6)
                .map(|m| m.as_str())
                .unwrap_or("")
                .to_string(),
            '{',
            '}',
        );
        let exponent: Option<Box<Math>>;

        if value.is_empty() {
            value = "1.0".to_string();
        }

        if !exponent_str.is_empty() {
            exponent = Some(Box::new(Parser::new(&exponent_str).parse()));
        } else {
            exponent = None;
        }

        Math::Variable(Variable {
            value: Decimal::from_str(&value).unwrap(),
            suffix,
            exponent,
        })
    }

    fn on_begining(tex: String) -> Option<String> {
        let re: Regex =
            Regex::new(r"^(-?\d+(?=\w*)(\.\d+)?|\d*(?=\w+)(\.\d+)?)(\w*)(\^\{(.+)\})?").unwrap();
        let f = re.find(&tex).unwrap()?.as_str();
        if !f.is_empty() {
            return Some(f.to_string());
        }
        return None;
    }
}

impl ops::Add<Math> for Variable {
    type Output = Math;
    fn add(self, _rhs: Math) -> Math {
        //        println!("{}+{}", self.to_tex(), _rhs.to_tex());
        match _rhs {
            Math::Polynom(p) => self.as_polynom() + Math::Polynom(p),
            Math::Variable(v) => self.addition(v),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Variable {
    type Output = Math;
    fn sub(self, _rhs: Math) -> Math {
        //        println!("{}-{}", self.to_tex(), _rhs.to_tex());
        match _rhs {
            Math::Polynom(p) => self.as_polynom() - Math::Polynom(p),
            Math::Variable(v) => self.subtraction(v),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Variable {
    type Output = Math;
    fn mul(self, _rhs: Math) -> Math {
        match _rhs {
            Math::Variable(v) => self.multiplication(v),
            Math::Polynom(p) => self.as_polynom() * Math::Polynom(p),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Variable {
    type Output = Math;
    fn div(self, _rhs: Math) -> Math {
        match _rhs {
            //  Math::Polynom(p)  => self.as_polynom()*Math::Polynom(p),
            Math::Variable(v) => self.division(v),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

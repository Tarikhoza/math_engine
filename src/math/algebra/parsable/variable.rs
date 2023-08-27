use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::{Parsable, Parser};

use fancy_regex::Regex;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

impl Parsable for Variable {
    fn to_tex(&self) -> String {
        if self.value.is_zero() {
            return String::from("0");
        }
        let mut val = self.value.normalize().to_string();
        if (val == "1.0" || val == "1") && !self.suffix.is_empty() {
            val = String::new();
        } else if (val == "-1.0" || val == "-1") && !self.suffix.is_empty() {
            val = String::from("-");
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
                Regex::new(r"(-?)(\d+(?=\w*)(\.\d+)?|\d*(?=\w+)(\.\d+)?)(\w*)(\^(\{.+))?")
                    .unwrap_or_else(|e| {
                        panic!("Failed to compile regex for variable: {e}");
                    });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");
        let minus = captures.get(1).map_or("", |m| m.as_str()).to_string();
        let mut value = captures.get(2).map_or("", |m| m.as_str()).to_string();
        let suffix = captures.get(5).map_or("", |m| m.as_str()).to_string();
        let exponent_str =
            Parser::extract_brace(captures.get(7).map_or("", |m| m.as_str()), '{', '}')?;

        if value.is_empty() {
            value = "1.0".to_string();
        }
        value = format!("{}{}", minus, value);

        let exponent: Option<Box<Math>> = if exponent_str.is_empty() {
            None
        } else {
            Some(Box::new(Parser::new(&exponent_str).parse()?))
        };

        Ok(Math::Variable(Variable {
            value: Decimal::from_str(&value).unwrap_or(dec!(1.0)),
            suffix,
            exponent,
            #[cfg(feature = "step-tracking")]
            step: None,
        }))
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(-?)(\d+(?=\w*)(\.\d+)?|\d*(?=\w+)(\.\d+)?)(\w*)(\^(\{.+))?")
                    .unwrap_or_else(|e| {
                        panic!("Failed to compile regex for variable: {e}");
                    });
        }
        let mut len = 0;

        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");
        let minus = captures.get(1).map_or("", |m| m.as_str()).to_string();

        let mut value = captures.get(2).map_or("", |m| m.as_str()).to_string();
        let suffix = captures.get(5).map_or("", |m| m.as_str()).to_string();
        let exponent_str =
            Parser::extract_brace(captures.get(7).map_or("", |m| m.as_str()), '{', '}')?;

        len += minus.len();
        len += value.len();

        if value.is_empty() {
            value = "1".to_string();
        }
        value = format!("{}{}", minus, value);

        let exponent: Option<Box<Math>> = if exponent_str.is_empty() {
            None
        } else {
            len += exponent_str.len() + 3;
            Some(Box::new(Parser::new(&exponent_str).parse()?))
        };

        len += suffix.len();

        Ok((
            len,
            Math::Variable(Variable {
                value: Decimal::from_str(&value).unwrap_or(dec!(1.0)),
                suffix,
                exponent,
                #[cfg(feature = "step-tracking")]
                step: None,
            }),
        ))
    }

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

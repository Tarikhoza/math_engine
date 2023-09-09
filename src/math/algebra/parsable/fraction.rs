use crate::math::algebra::fraction::Fraction;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive, Parser};

use fancy_regex::Regex;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

impl Parsable for Fraction {
    fn to_tex(&self) -> String {
        if self.whole.is_some() {
            return format!(
                "{}\\frac{{{}}}{{{}}}",
                self.whole.expect("No whole found"),
                self.numerator.to_tex(),
                self.denominator.to_tex()
            );
        }
        format!(
            "\\frac{{{}}}{{{}}}",
            self.numerator.to_tex(),
            self.denominator.to_tex()
        )
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d*)(\\frac)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");

        let whole_str = captures.get(1).map_or("", |m| m.as_str());

        let whole: Option<Decimal> = if !whole_str.is_empty() {
            Some(Decimal::from_str(whole_str).expect("failed converting string to decimal"))
        } else {
            None
        };

        let numerator =
            Parser::extract_brace(captures.get(3).map_or("", |m| m.as_str()), '{', '}')?;
        let denominator = Parser::extract_brace(
            captures
                .get(3)
                .map_or("", |m| m.as_str())
                .get(numerator.len() + 2..)
                .expect("falied to execute regex"),
            '{',
            '}',
        )?;

        if numerator.is_empty() {
            return Err("While parsing numerator was empty");
        }
        if denominator.is_empty() {
            return Err("While parsing denominator was empty");
        }

        Ok(Math::Fraction(Fraction {
            whole,
            numerator: Box::new(numerator.parse_math()?),
            denominator: Box::new(denominator.parse_math()?),
        }))
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d*)(\\frac)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }

        let mut len = 5;

        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");

        let whole_str = captures.get(1).map_or("", |m| m.as_str());
        len += whole_str.len();

        let whole: Option<Decimal> = if !whole_str.is_empty() {
            Some(Decimal::from_str(whole_str).expect("failed converting string to decimal"))
        } else {
            None
        };

        let numerator =
            Parser::extract_brace(captures.get(3).map_or("", |m| m.as_str()), '{', '}')?;

        len += numerator.len() + 2;

        let denominator = Parser::extract_brace(
            captures
                .get(3)
                .map_or("", |m| m.as_str())
                .get(numerator.len() + 2..)
                .expect("falied to execute regex"),
            '{',
            '}',
        )?;

        len += denominator.len() + 2;

        if numerator.is_empty() {
            return Err("While parsing numerator was empty");
        }
        if denominator.is_empty() {
            return Err("While parsing denominator was empty");
        }

        Ok((
            len,
            Math::Fraction(Fraction {
                whole,
                numerator: Box::new(numerator.parse_math()?),
                denominator: Box::new(denominator.parse_math()?),
            }),
        ))
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\\frac)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
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

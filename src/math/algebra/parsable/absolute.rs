use crate::math::algebra::absolute::Absolute;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable, Parser};

use fancy_regex::Regex;
use rust_decimal_macros::dec;

impl Parsable for Absolute {
    fn to_tex(&self) -> String {
        format!("\\lvert{}\\rvert", self.math.to_tex())
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\\lvert(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");

        let math = Parser::extract_between(
            captures.get(0).map_or("", |m| m.as_str()),
            "\\lvert",
            "\\rvert",
        )?;
        Ok(Math::Absolute(Absolute {
            math: Box::new(math.parse_math()?),
        }))
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\\lvert(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");

        let math = Parser::extract_between(
            captures.get(0).map_or("", |m| m.as_str()),
            "\\lvert",
            "\\rvert",
        )?;

        let len = 12 + math.len();

        Ok((
            len,
            Math::Absolute(Absolute {
                math: Box::new(math.parse_math()?),
            }),
        ))
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\\lvert(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for absolute: {e}");
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

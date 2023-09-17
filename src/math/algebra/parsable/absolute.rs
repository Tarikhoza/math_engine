use crate::math::algebra::absolute::Absolute;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive, Parser};

use fancy_regex::Regex;

impl Parsable for Absolute {
    fn to_tex(&self) -> String {
        format!("\\lvert{}\\rvert", self.math.to_tex())
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\\lvert(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }
        let result = RE.captures(tex);
        if let Ok(Some(captures)) = result {
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
        } else {
            Err("Failed capturing input for absolute")
        }
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

use crate::math::algebra::root::Root;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable, Parser};

use fancy_regex::Regex;

impl Parsable for Root {
    fn to_tex(&self) -> String {
        if let Some(base) = self.base.clone() {
            format!("\\sqrt[{}]{{{}}}", base.to_tex(), self.math.to_tex())
        } else {
            format!("\\sqrt[]{{{}}}", self.math.to_tex())
        }
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        //TODO use strip_prefix instead of regex
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\\sqrt)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }

        let mut len = 5;

        let result = RE.captures(tex);
        if let Ok(Some(captures)) = result {
            let base = Parser::extract_brace(captures.get(2).map_or("", |m| m.as_str()), '[', ']')?;

            len += base.len() + 2;

            let math;
            if let Some(math_str) = captures
                .get(2)
                .map_or("", |m| m.as_str())
                .get(base.len() + 2..)
            {
                math = Parser::extract_brace(math_str, '{', '}')?;
            } else {
                return Err("Failed capturing math on root");
            }

            len += math.len() + 2;

            if base.is_empty() {
                return Ok((
                    len,
                    Math::Root(Root {
                        math: Box::new(math.parse_math()?),
                        base: Some(Box::new(Math::Variable(2.as_variable()))),
                    }),
                ));
            }
            Ok((
                len,
                Math::Root(Root {
                    math: Box::new(math.parse_math()?),
                    base: Some(Box::new(base.parse_math()?)),
                }),
            ))
        } else {
            Err("Failed capturing input for root")
        }
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\\sqrt)(.*)").unwrap_or_else(|e| {
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

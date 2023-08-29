use crate::math::algebra::root::Root;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, ParsableGenericsAsVariable, Parser};

use fancy_regex::Regex;

impl Parsable for Root {
    fn to_tex(&self) -> String {
        if self.base.is_none() {
            format!("\\sqrt[]{{{}}}", self.math.to_tex())
        } else {
            format!(
                "\\sqrt[{}]{{{}}}",
                self.base
                    .clone()
                    .expect("root base is nonexistent")
                    .to_tex(),
                self.math.to_tex()
            )
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\\sqrt)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");
        let base = Parser::extract_brace(captures.get(2).map_or("", |m| m.as_str()), '[', ']')?;
        let math = Parser::extract_brace(
            captures
                .get(2)
                .map_or("", |m| m.as_str())
                .get(base.len() + 2..)
                .expect("failed to execute regex"),
            '{',
            '}',
        )?;
        if base.is_empty() {
            return Ok(Math::Root(Root {
                math: Box::new(math.parse_math()?),
                base: Some(Box::new(Math::Variable(2.as_variable()))),
            }));
        }
        Ok(Math::Root(Root {
            math: Box::new(math.parse_math()?),
            base: Some(Box::new(base.parse_math()?)),
        }))
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
        let captures = result
            .expect("Error running regex")
            .expect("No match found");
        let base = Parser::extract_brace(captures.get(2).map_or("", |m| m.as_str()), '[', ']')?;

        len += base.len() + 2;

        let math = Parser::extract_brace(
            captures
                .get(2)
                .map_or("", |m| m.as_str())
                .get(base.len() + 2..)
                .expect("failed to execute regex"),
            '{',
            '}',
        )?;

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

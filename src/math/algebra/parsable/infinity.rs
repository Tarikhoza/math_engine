use crate::math::algebra::infinity::Infinity;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::{Parsable, Parser};

use fancy_regex::Regex;

impl Parsable for Infinity {
    fn to_tex(&self) -> String {
        if self.minus {
            return String::from(r"-\infty");
        }
        String::from(r"\infty")
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        Ok(Math::Variable(Variable::default()))
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^-?\\infty").unwrap_or_else(|e| {
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

use crate::math::calculus::factorial::Factorial;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive, Parser};
use fancy_regex::Regex;

impl Parsable for Factorial {
    fn to_tex(&self) -> String {
        format!("{}!", self.math.to_tex())
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        todo!()
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^!").unwrap_or_else(|e| {
                panic!("Failed to compile regex for factorial: {e}");
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

use crate::math::calculus::sum::Sum;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive, Parser};
//\sum_{n=1}^{\infty} 2^{-n}

impl Parsable for Sum {
    fn to_tex(&self) -> String {
        format!(
            "\\sum_{{{}={}}}^{{{}}}{}",
            self.iter_suffix,
            self.begining.to_tex(),
            self.end.to_tex(),
            self.math.to_tex()
        )
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        if let Some(tex) = tex.strip_prefix("\\sum_") {
            let mut len = 5;

            let down_str = Parser::extract_brace(tex, '{', '}')?;

            len += down_str.len() + 2;

            let iter_suffix;
            if let Some(suffix) = down_str.split('=').collect::<Vec<&str>>().first() {
                iter_suffix = suffix.to_string();
            } else {
                return Err("Failed extracting iter_suffix for sum");
            }

            let begining;

            if let Some(b) = down_str.split('=').collect::<Vec<&str>>().get(1) {
                begining = Box::new(b.parse_math()?)
            } else {
                return Err("Failed extracting begining for sum");
            }

            let up_str;
            if let Some(u) = tex.get(down_str.len() + 3..) {
                up_str = Parser::extract_brace(u, '{', '}')?;
            } else {
                return Err("Failed capturing upper part of sum");
            }

            len += up_str.len() + 3;

            let (math_len, math);
            if let Some(math_str) = tex.get(down_str.len() + 3 + up_str.len() + 2..) {
                (math_len, math) = Math::from_tex_len(math_str)?;
            } else {
                return Err("Failed extracting math from sum");
            }

            len += math_len;

            Ok((
                len,
                Math::Sum(Sum {
                    iter_suffix,
                    begining,
                    end: Box::new(up_str.parse_math()?),
                    math: Box::new(math),
                }),
            ))
        } else {
            Err("Failed capturing input for sum")
        }
    }

    fn on_begining(tex: String) -> Option<String> {
        if tex.strip_prefix("\\sum_").is_some() {
            return Some(tex);
        }
        None
    }
}

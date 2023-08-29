use crate::math::calculus::sum::Sum;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, Parser};
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

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        let my_tex = tex
            .strip_prefix("\\sum_")
            .expect("failed to strip prefix from sum");
        let down = Parser::extract_brace(my_tex, '{', '}')?;
        let up = Parser::extract_brace(
            my_tex
                .get(down.len() + 3..)
                .expect("failed extracting upper from sum"),
            '{',
            '}',
        )?;
        let math = my_tex
            .get(down.len() + 3 + up.len() + 2..)
            .expect("failed extracting math from sum");
        Ok(Math::Sum(Sum {
            iter_suffix: down
                .split('=')
                .collect::<Vec<&str>>()
                .get(0)
                .expect("failed extracting iter_suffix from sum")
                .to_string(),
            begining: Box::new(
                down.split('=')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .expect("failed extracting begining from sum")
                    .parse_math()?,
            ),
            end: Box::new(up.parse_math()?),
            math: Box::new(format!("({})", math).parse_math()?),
        }))
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        let my_tex = tex
            .strip_prefix("\\sum_")
            .expect("failed to strip prefix from sum");

        let mut len = 5;

        let down = Parser::extract_brace(my_tex, '{', '}')?;

        len += down.len() + 2;

        let up = Parser::extract_brace(
            my_tex
                .get(down.len() + 3..)
                .expect("failed extracting upper from sum"),
            '{',
            '}',
        )?;

        len += up.len() + 3;

        let (math_len, math) = Math::from_tex_len(
            my_tex
                .get(down.len() + 2 + up.len() + 3..)
                .expect("failed extracting math from sum"),
        )?;

        dbg!(len);
        dbg!(tex.get(0..len));
        len += math_len;
        dbg!(tex.get(0..len));

        Ok((
            len,
            Math::Sum(Sum {
                iter_suffix: down
                    .split('=')
                    .collect::<Vec<&str>>()
                    .get(0)
                    .expect("failed extracting iter_suffix from sum")
                    .to_string(),
                begining: Box::new(
                    down.split('=')
                        .collect::<Vec<&str>>()
                        .get(1)
                        .expect("failed extracting begining from sum")
                        .parse_math()?,
                ),
                end: Box::new(up.parse_math()?),
                math: Box::new(math),
            }),
        ))
    }

    fn on_begining(tex: String) -> Option<String> {
        if tex.strip_prefix("\\sum_").is_some() {
            return Some(tex);
        }
        None
    }
}

use crate::math::calculus::product::Product;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive, Parser};

impl Parsable for Product {
    fn to_tex(&self) -> String {
        format!(
            "\\prod_{{{}={}}}^{{{}}}{}",
            self.iter_suffix,
            self.begining.to_tex(),
            self.end.to_tex(),
            self.math.to_tex()
        )
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        let my_tex = tex
            .strip_prefix("\\prod_")
            .expect("failed to strip prefix from prod");

        let down = Parser::extract_brace(my_tex, '{', '}')?;

        let up = Parser::extract_brace(
            my_tex
                .get(down.len() + 3..)
                .expect("failed extracting upper from prod"),
            '{',
            '}',
        )?;

        let math = my_tex
            .get(down.len() + 3 + up.len() + 2..)
            .expect("failed extracting math from prod");

        Ok(Math::Product(Product {
            iter_suffix: down
                .split('=')
                .collect::<Vec<&str>>()
                .get(0)
                .expect("failed extracting iter_suffix from prod")
                .to_string(),
            begining: Box::new(
                down.split('=')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .expect("failed extracting begining from prod")
                    .parse_math()?,
            ),
            end: Box::new(up.parse_math()?),
            math: Box::new(math.parse_math()?),
        }))
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        let my_tex = tex
            .strip_prefix("\\prod_")
            .expect("failed to strip prefix from prod");

        let mut len = 6;

        let down = Parser::extract_brace(my_tex, '{', '}')?;

        len += down.len() + 2;

        let up = Parser::extract_brace(
            my_tex
                .get(down.len() + 3..)
                .expect("failed extracting upper from prod"),
            '{',
            '}',
        )?;

        len += up.len() + 3;

        let (math_len, math) = Math::from_tex_len(
            my_tex
                .get(down.len() + 3 + up.len() + 2..)
                .expect("failed extracting math from prod"),
        )?;

        len += math_len;

        Ok((
            len,
            Math::Product(Product {
                iter_suffix: down
                    .split('=')
                    .collect::<Vec<&str>>()
                    .get(0)
                    .expect("failed extracting iter_suffix from prod")
                    .to_string(),
                begining: Box::new(
                    down.split('=')
                        .collect::<Vec<&str>>()
                        .get(1)
                        .expect("failed extracting begining from prod")
                        .parse_math()?,
                ),
                end: Box::new(up.parse_math()?),
                math: Box::new(math),
            }),
        ))
    }

    fn on_begining(tex: String) -> Option<String> {
        if tex.strip_prefix("\\prod_").is_some() {
            return Some(tex);
        }
        None
    }
}

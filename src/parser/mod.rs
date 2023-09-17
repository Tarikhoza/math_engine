pub mod math;
pub mod operator;
pub mod primitive;

use crate::castable::Castable;
use crate::math::algebra::operations::Operator as AlgebraOperator;

use crate::math::algebra::polynom::PolynomPart;
use crate::math::algebra::{
    absolute::Absolute, braces::Braces, fraction::Fraction, function::Function, polynom::Polynom,
    root::Root, variable::Variable,
};

use crate::math::calculus::{factorial::Factorial, product::Product, sum::Sum};

use crate::math::operator::Operator;
use crate::math::Math;

pub struct Parser {
    input: String,
    pos: usize,
}

pub trait Parsable {
    fn on_begining(_tex: String) -> Option<String> {
        Some(String::new())
    }

    fn to_tex(&self) -> String {
        String::new()
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        let (_len, math) = Self::from_tex_len(tex)?;
        Ok(math)
    }
    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str>;

    fn parse(tex: &str) -> Option<(usize, Math)> {
        if let Some(t) = Self::on_begining(tex.replace(' ', "")) {
            let (len, math) = Self::from_tex_len(&t).unwrap_or((0, Math::default()));
            return Some((len, math));
        }
        None
    }
}

pub trait ParsablePrimitive {
    fn parse_math(&self) -> Result<Math, &'static str>;
}

pub trait ParsablePrimitiveAsVariable {
    fn as_variable(&self) -> Variable;
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        Parser {
            input: input.to_string().replace(' ', ""),
            pos: 0,
        }
    }

    pub fn extract_brace(tex: &str, open_c: char, close_c: char) -> Result<String, &'static str> {
        let mut pos = 1;
        let mut close = 0;
        let mut open = 1;
        if !tex.starts_with(open_c) {
            return Ok(String::new());
        }
        for c in tex.chars().skip(1) {
            match c {
                x if x == open_c => open += 1,
                x if x == close_c => close += 1,
                _ => {}
            }
            if open == close {
                return if let Some(inner) = tex.get(1..pos) {
                    Ok(inner.to_string())
                } else {
                    Err("extract_between: could not extract inner")
                };
            }
            pos += 1;
        }
        Err("extract_between: Brace never closed")
    }

    pub fn extract_between(tex: &str, open_s: &str, close_s: &str) -> Result<String, &'static str> {
        let mut pos = open_s.len();
        let mut close = 0;
        let mut open = 1;
        if tex.starts_with(open_s) {
            return Ok(String::new());
        }

        while pos < tex.len() {
            if let Some(current_tex) = tex.get(pos..) {
                match current_tex {
                    x if x.starts_with(open_s) => {
                        open += 1;
                        pos += open_s.len()
                    }
                    x if x.starts_with(close_s) => {
                        close += 1;
                        pos += close_s.len()
                    }
                    _ => pos += 1,
                }
            }
            if open == close {
                return if let Some(inner) = tex.get(open_s.len()..(pos - close_s.len())) {
                    Ok(inner.to_string())
                } else {
                    Err("Could not extract inner")
                };
            }
        }
        Err("Brace never closed")
    }

    pub fn parse(&mut self) -> Result<Math, &'static str> {
        Ok(Self::parse_len(self)?.1)
    }

    pub fn parse_len(&mut self) -> Result<(usize, Math), &'static str> {
        type ParseFn = fn(tex: &str) -> Option<(usize, Math)>;
        let to_parse: Vec<ParseFn> = vec![
            Sum::parse,
            Product::parse,
            Function::parse,
            Braces::parse,
            Fraction::parse,
            Root::parse,
            Absolute::parse,
            Variable::parse,
        ];

        let mut parts: Vec<PolynomPart> = vec![];

        let mut op_search: bool = false;

        'outer: while self.pos < self.input.len() {
            let remaining_input = self.input.get(self.pos..).unwrap_or("");
            if remaining_input.is_empty() {
                return Err("Error while parsing");
            }
            if op_search {
                //Search for suffixes like factorial etc.
                if Factorial::on_begining((*remaining_input).to_string()).is_some()
                    && !parts.is_empty()
                {
                    if let Some(last) = parts.pop() {
                        if let PolynomPart::Math(m) = last {
                            parts.push(Factorial { math: Box::new(m) }.as_math().as_polynom_part())
                        } else {
                            return Err("Error because operator before factorial");
                        }
                    } else {
                        return Err("Error while parsing factorial without Math in front");
                    }
                    self.pos += 1;
                    continue 'outer;
                }

                if let Some(tex) = Operator::on_begining((*remaining_input).to_string()) {
                    let o = Operator::from_tex(&tex)?;
                    self.pos += o.to_tex().len();
                    parts.push(o.as_polynom_part());
                } else {
                    parts.push(Operator::Algebra(AlgebraOperator::InvMulti).as_polynom_part());
                }
                op_search = false;
            } else {
                for parsing in to_parse.iter() {
                    if let Some(pair) = parsing(remaining_input) {
                        self.pos += pair.0;
                        parts.push(pair.1.as_polynom_part());
                        op_search = true;
                        continue 'outer;
                    }
                }

                println!(
                    "Invalid character at position {}: '{}' \n\n'{}'\n",
                    self.pos,
                    self.input.chars().nth(self.pos).unwrap_or(' '),
                    self.input.get(self.pos..).unwrap_or("")
                );

                return Err("While parsing found invalid character");
            }
        }

        Ok((
            self.pos,
            Polynom {
                parts,
                #[cfg(feature = "step-tracking")]
                step: None,
            }
            .unpack(),
        ))
    }
}

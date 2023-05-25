pub mod generics;
pub mod math;
pub mod operator;

use crate::math::algebra::braces::Braces;
use crate::math::operator::algebra::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};
use crate::math::operator::Operator;
//use crate::math::algebra::equation::Equation;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use rust_decimal_macros::dec;

pub struct Parser {
    input: String,
    pos: usize,
}

pub trait Parsable {
    fn on_begining(tex: String) -> Option<String> {
        Some(String::new())
    }
    fn to_tex(&self) -> String {
        String::new()
    }
    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        Variable::from_tex("0")
    }
    fn parse(tex: &str) -> Option<(usize, Math)> {
        if let Some(t) = Self::on_begining(tex.to_owned()) {
            let math = Self::from_tex(&t).unwrap_or(Math::Variable(Variable {
                value: dec!(0),
                suffix: String::new(),
                exponent: None,
                #[cfg(feature = "step-tracking")]
                step: None,
            }));
            let len = math.to_tex().len();
            return Some((len, math));
        }
        None
    }
}

pub trait ParsableGenerics {
    fn parse_math(&self) -> Result<Math, &'static str>;
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
                return Ok(tex
                    .get(1..pos)
                    .expect("while extracting brace something unexpected happened")
                    .to_string());
            }
            pos += 1;
        }
        Err("Brace never closed")
    }

    pub fn parse(&mut self) -> Result<Math, &'static str> {
        type ParseFn = fn(tex: &str) -> Option<(usize, Math)>;
        let to_parse: Vec<ParseFn> = vec![
            //            Equation::parse,
            //
            Braces::parse,
            Fraction::parse,
            Variable::parse,
        ];

        let mut factors: Vec<Math> = vec![];
        let mut operators: Vec<Operator> = vec![];
        let mut op_search: bool = false;

        'outer: while self.pos < self.input.len() {
            let remaining_input = &self.input.get(self.pos..).unwrap_or("");
            if remaining_input.is_empty() {
                return Err("Error while parsing");
            }
            if op_search {
                if let Some(tex) = Operator::on_begining((*remaining_input).to_string()) {
                    let o = Operator::from_tex(&tex)?;
                    self.pos += o.to_tex().len();
                    if let Math::Operator(o) = o {
                        operators.push(o);
                    }
                } else {
                    operators.push(Operator::Algebra(AlgebraOperator::InvMulti));
                }
                op_search = false;
            } else {
                for parsing in to_parse.iter() {
                    if let Some(pair) = parsing(remaining_input) {
                        self.pos += pair.0;
                        factors.push(pair.1);
                        op_search = true;
                        continue 'outer;
                    }
                }
                println!(
                    "Invalid character at position {}: '{}' ",
                    self.pos,
                    self.input.chars().nth(self.pos).unwrap_or(' ')
                );

                return Err("While parsing found invalid character");
            }
        }
        Ok(Polynom {
            factors,
            operators,
            #[cfg(feature = "step-tracking")]
            step: None,
        }
        .unpack())
    }
}

use crate::math::algebra::polynom::PolynomPart;
use crate::parser::{Parsable, Parser};
use crate::{
    lexer::Token,
    math::{
        algebra::{
            absolute::Absolute, braces::Braces, fraction::Fraction, function::Function,
            infinity::Infinity, polynom::Polynom, root::Root, undefined::Undefined,
        },
        calculus::product::Product,
        calculus::sum::Sum,
        Math,
    },
};

impl Parsable for Math {
    fn to_tex(&self) -> String {
        match self {
            Math::Variable(s) => s.to_tex(),
            Math::Absolute(s) => s.to_tex(),
            Math::Braces(s) => s.to_tex(),
            Math::Polynom(s) => s.to_tex(),
            Math::Fraction(s) => s.to_tex(),
            Math::Function(s) => s.to_tex(),
            Math::Root(s) => s.to_tex(),
            Math::Undefined(s) => s.to_tex(),
            Math::Infinity(s) => s.to_tex(),
            Math::Sum(s) => s.to_tex(),
            Math::Product(s) => s.to_tex(),
            Math::Factorial(s) => s.to_tex(),
            Math::Vector(s) => s.to_tex(),
            Math::Matrix(s) => s.to_tex(),
        }
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        if let Ok((math, len)) = Parser::new(token_stream).parse() {
            return Ok((math.as_polynom_part(), len));
        } else {
            return Err("Failed parsing math".to_string());
        }
    }

    fn get_type(&self) -> String {
        match self {
            Math::Variable(s) => format!("Math::{}", s.get_type()),
            Math::Absolute(s) => format!("Math::{}", s.get_type()),
            Math::Braces(s) => format!("Math::{}", s.get_type()),
            Math::Polynom(s) => format!("Math::{}", s.get_type()),
            Math::Fraction(s) => format!("Math::{}", s.get_type()),
            Math::Function(s) => format!("Math::{}", s.get_type()),
            Math::Root(s) => format!("Math::{}", s.get_type()),
            Math::Undefined(s) => format!("Math::{}", s.get_type()),
            Math::Infinity(s) => format!("Math::{}", s.get_type()),
            Math::Sum(s) => format!("Math::{}", s.get_type()),
            Math::Product(s) => format!("Math::{}", s.get_type()),
            Math::Factorial(s) => format!("Math::{}", s.get_type()),
            Math::Matrix(s) => format!("Math::{}", s.get_type()),
            Math::Vector(s) => format!("Math::{}", s.get_type()),
        }
    }
}

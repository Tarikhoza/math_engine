use crate::math::{
    algebra::{
        absolute::Absolute, braces::Braces, fraction::Fraction, function::Function,
        infinity::Infinity, polynom::Polynom, root::Root, undefined::Undefined,
    },
    calculus::product::Product,
    calculus::sum::Sum,
    Math,
};
use crate::parser::{Parsable, Parser};

impl Parsable for Math {
    fn to_tex(&self) -> String {
        match self {
            Math::Variable(s) => s.to_tex(),
            Math::Fraction(s) => s.to_tex(),
            Math::Braces(s) => s.to_tex(),
            Math::Polynom(s) => s.to_tex(),
            Math::Undefined(s) => s.to_tex(),
            Math::Infinity(s) => s.to_tex(),
            Math::Root(s) => s.to_tex(),
            Math::Function(s) => s.to_tex(),
            Math::Absolute(s) => s.to_tex(),
            Math::Sum(s) => s.to_tex(),
            Math::Product(s) => s.to_tex(),
            Math::Factorial(s) => s.to_tex(),
            Math::Matrix(s) => s.to_tex(),
            Math::Vector(s) => s.to_tex(),
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        Parser::new(tex).parse()
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        match tex.to_string() {
            x if Sum::on_begining(x.clone()).is_some() => Sum::from_tex_len(tex),
            x if Product::on_begining(x.clone()).is_some() => Product::from_tex_len(tex),
            x if Absolute::on_begining(x.clone()).is_some() => Absolute::from_tex_len(tex),
            x if Root::on_begining(x.clone()).is_some() => Root::from_tex_len(tex),
            x if Function::on_begining(x.clone()).is_some() => Function::from_tex_len(tex),
            x if Undefined::on_begining(x.clone()).is_some() => Undefined::from_tex_len(tex),
            x if Infinity::on_begining(x.clone()).is_some() => Infinity::from_tex_len(tex),
            x if Fraction::on_begining(x.clone()).is_some() => Fraction::from_tex_len(tex),
            x if Braces::on_begining(x.clone()).is_some() => Braces::from_tex_len(tex),
            _ => Polynom::from_tex_len(tex),
        }
    }

    fn on_begining(_tex: String) -> Option<String> {
        None
    }
}

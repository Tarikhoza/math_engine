use crate::math::algebra::polynom::{Polynom, PolynomPart};
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::{Parsable, Parser};

impl Parsable for Polynom {
    fn to_tex(&self) -> String {
        let mut tex = String::new();
        for part in self.parts.iter() {
            tex = format!("{}{}", tex, part.to_tex());
        }
        tex
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        Parser::new(tex).parse()
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        Parser::new(tex).parse_len()
    }

    fn on_begining(_tex: String) -> Option<String> {
        None
    }
}

impl Parsable for PolynomPart {
    fn to_tex(&self) -> String {
        match self {
            PolynomPart::Math(math) => math.to_tex(),
            PolynomPart::Operator(op) => op.to_tex(),
        }
    }

    fn from_tex_len(_tex: &str) -> Result<(usize, Math), &'static str> {
        unimplemented!();
    }

    fn on_begining(_tex: String) -> Option<String> {
        unimplemented!()
    }
}

impl Math {
    pub fn as_polynom_part(&self) -> PolynomPart {
        PolynomPart::Math(self.clone())
    }
}
impl Operator {
    pub fn as_polynom_part(&self) -> PolynomPart {
        PolynomPart::Operator(self.clone())
    }
}

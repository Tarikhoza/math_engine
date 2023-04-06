use crate::braces::Braces;
use crate::math::{Math, Operators};
use crate::polynom::Polynom;
use crate::variable::Variable;

pub struct Parser {
    input: String,
    pos: usize,
}

pub trait Parsable {
    fn on_begining(tex: String) -> Option<String>;
    fn to_tex(&self) -> String;
    fn from_tex(tex: &str) -> Result<Math, &'static str>;
}

impl Parser {
    #[must_use]
    pub fn new(input: &str) -> Parser {
        Parser {
            input: input.to_string().replace(' ', ""),
            pos: 0,
        }
    }
    #[must_use]
    pub fn extract_brace(tex: &str, open_c: char, close_c: char) -> String {
        let mut pos = 1;
        let mut close = 0;
        let mut open = 1;
        if !tex.starts_with(open_c) {
            return String::new();
        }
        for c in tex.chars().skip(1) {
            match c {
                x if x == open_c => open += 1,
                x if x == close_c => close += 1,
                _ => {}
            }
            if open == close {
                return tex[1..pos].to_string();
            }
            pos += 1;
        }
        String::new()
    }

    pub fn parse(&mut self) -> Result<Math, &'static str> {
        let mut factors: Vec<Math> = vec![];
        let mut operators: Vec<Operators> = vec![];
        let mut op_search: bool = false;

        while self.pos < self.input.len() {
            let remaining_input = &self.input.get(self.pos..).unwrap_or("");
            if remaining_input.is_empty() {
                return Err("Error while parsing {self.input}");
            }
            if op_search {
                if let Some(tex) = Operators::on_begining((*remaining_input).to_string()) {
                    let o = Operators::from_tex(&tex)?;
                    self.pos += tex.len();
                    if let Math::Operators(o) = o {
                        operators.push(o);
                    }
                } else {
                    operators.push(Operators::InvMulti);
                }
                op_search = false;
            } else {
                if let Some(tex) = Braces::on_begining((*remaining_input).to_string()) {
                    let b = Braces::from_tex(&tex)?;
                    self.pos += tex.len();
                    factors.push(b);
                } else if let Some(tex) = Variable::on_begining((*remaining_input).to_string()) {
                    let v = Variable::from_tex(&tex)?;
                    self.pos += tex.len();
                    factors.push(v);
                } else {
                    return Err(
                        "Invalid character at position {self.pos}: '{&self.input.chars().nth(self.pos).unwrap_or(' ')}' ",
                    );
                }
                op_search = true;
            }
        }
        Ok(Math::Polynom(Polynom { factors, operators }))
    }
}

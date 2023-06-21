use crate::math::operator::Operator as MainOperator;
use crate::math::Math;
use crate::parser::Parsable;
use fancy_regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
}

impl Parsable for Operator {
    fn to_tex(&self) -> String {
        match self {
            Operator::Equal => String::from("="),
            Operator::NotEqual => String::from("!="),
            Operator::Greater => String::from(">"),
            Operator::Less => String::from("<"),
            Operator::GreaterOrEqual => String::from(">="),
            Operator::LessOrEqual => String::from("<="),
        }
    }
    fn from_tex(op: &str) -> Result<Math, &'static str> {
        match op {
            x if x == "=" => Ok(Math::Operator(MainOperator::Equation(Operator::Equal))),
            x if x == "!=" => Ok(Math::Operator(MainOperator::Equation(Operator::NotEqual))),
            x if x == ">" => Ok(Math::Operator(MainOperator::Equation(Operator::Greater))),
            x if x == "<" => Ok(Math::Operator(MainOperator::Equation(Operator::Less))),
            x if x == ">=" => Ok(Math::Operator(MainOperator::Equation(
                Operator::GreaterOrEqual,
            ))),
            x if x == "<=" => Ok(Math::Operator(MainOperator::Equation(
                Operator::LessOrEqual,
            ))),
            _ => Err("Conversion from string to operator went wrong"),
        }
    }
    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(=|!=|\>|\<|>=|<=)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for operators: {e}");
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

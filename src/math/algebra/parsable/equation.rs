use crate::math::algebra::equation::Equation;
use crate::math::operator::equation::Operator as EquationOperator;
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics};

use fancy_regex::Regex;

impl Parsable for Equation {
    fn to_tex(&self) -> String {
        if !self.factors.is_empty() {
            if self.factors.len() <= 1 && self.factors.len() != self.operators.len() + 1 {
                return self.factors[0].to_tex();
            }
            let mut temp = self.factors[0].to_tex();
            for (i, factor) in self.factors.iter().skip(1).enumerate() {
                temp = format!(
                    "{}{}{}",
                    temp,
                    EquationOperator::to_tex(&self.operators[i]),
                    factor.to_tex()
                );
            }
            return temp;
        }
        String::new()
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(=|!=|\>|\<|>=|<=)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for equation detection: {e}");
            });
        }

        let result: Vec<String> = RE
            .find_iter(tex)
            .filter_map(|op| op.unwrap().as_str().parse().ok())
            .collect();

        let operators: Vec<EquationOperator> = result
            .iter()
            .map(|op| match EquationOperator::from_tex(op).unwrap() {
                Math::Operator(Operator::Equation(o)) => return o,
                _ => panic!("error parsing equation operators"),
            })
            .collect();

        let factors: Vec<Math> = RE
            .replace_all(tex, " ")
            .trim()
            .split(" ")
            .map(|f| match f {
                "" => Math::Operator(Operator::default()),
                other => return other.parse_math().expect("error parsing math of equation"),
            })
            .collect();

        if factors.contains(&Math::Operator(Operator::default()))
            || (factors.len() <= operators.len())
        {
            return Err("Equation has to many operators");
        }

        Ok(Math::Equation(Equation { factors, operators }))
    }
    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(=|!=|\>|\<|>=|<=)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for equation detection: {e}");
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

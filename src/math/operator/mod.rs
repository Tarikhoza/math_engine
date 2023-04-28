pub mod algebra;
use crate::math::operator::algebra::Operator as AlgebraOperator;
use crate::math::Math;
use crate::parser::Parsable;
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Algebra(AlgebraOperator),
}

impl Parsable for Operator {
    fn to_tex(&self) -> String {
        match self {
            Operator::Algebra(o) => o.to_tex(),
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        AlgebraOperator::from_tex(tex)
    }

    fn on_begining(tex: String) -> Option<String> {
        AlgebraOperator::on_begining(tex)
    }
 }

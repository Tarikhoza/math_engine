use crate::math::operator::Operator;
use crate::math::operator::algebra::Operator as AlgebraOperator;
use crate::solver::step::DetailedOperator;
use crate::math::Math;
use crate::parser::Parsable;

impl Parsable for Operator {
    fn to_tex(&self) -> String {
        match self {
            Operator::Algebra(o) => o.to_tex(),
            Operator::Detail(o)=> String::from("Detail"),
            Operator::Empty => String::from("Empty")
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        AlgebraOperator::from_tex(tex)
    }

    fn on_begining(tex: String) -> Option<String> {
        AlgebraOperator::on_begining(tex)
    }
 }

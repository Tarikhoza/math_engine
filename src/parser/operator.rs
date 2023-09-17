use crate::math::algebra::operations::Operator as AlgebraOperator;
use crate::math::operator::Operator;

impl Operator {
    pub fn to_tex(&self) -> String {
        match self {
            Operator::Algebra(o) => o.to_tex(),
            Operator::Detail(_o) => String::from("Detail"),
            Operator::Empty => String::from("Empty"),
        }
    }

    pub fn from_tex(tex: &str) -> Result<Operator, &'static str> {
        Ok(Operator::Algebra(AlgebraOperator::from_tex(tex)?))
    }

    pub fn on_begining(tex: String) -> Option<String> {
        AlgebraOperator::on_begining(tex)
    }
}

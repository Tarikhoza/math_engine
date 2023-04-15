use crate::math::algebra::polynom::Polynom;
use crate::math::operator::algebra::Operator as AlgebraOperator;
use crate::math::Math;
use crate::parser::Parsable;

impl Parsable for Polynom {
    #[must_use]
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
                    AlgebraOperator::to_tex(&self.operators[i]),
                    factor.to_tex()
                );
            }
            return temp;
        }
        String::new()
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        Math::from_tex(tex)
    }

    #[must_use]
    fn on_begining(_tex: String) -> Option<String> {
        None
    }
}

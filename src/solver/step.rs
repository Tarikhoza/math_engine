use crate::math::algebra::variable::Variable;
use crate::math::operator::algebra::Operator as AlgebraOperator;
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::Parsable;
use std::default;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Step {
    left: Box<Math>,
    right: Option<Box<Math>>,
    operation: Operator,
    details: String,
    sub_steps: Vec<Step>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DetailedOperator {
    Nothing,
    GetExponent,
    Negate,
    MapTo,
    Simplify,
    SimplifyParentences,
    SimplifyExponents,
    SimplifyMultiplicationDivision,
    SimplifyAddtionSubtraction,
    GroupTogether,
}

impl Step {
    pub fn step(
        left: Math,
        right: Option<Math>,
        operation: Operator,
        details: String,
    ) -> Option<Step> {
        if let Some(r) = right {
            Some(Step {
                left: Box::new(left),
                right: Some(Box::new(r)),
                operation,
                details,
                sub_steps: vec![],
            })
        } else {
            Some(Step {
                left: Box::new(left),
                right: None,
                operation,
                details,
                sub_steps: vec![],
            })
        }
    }

    pub fn steps(
        left: Math,
        right: Option<Math>,
        operation: Operator,
        details: String,
        sub_steps: Vec<Step>,
    ) -> Option<Step> {
        if let Some(r) = right {
            Some(Step {
                left: Box::new(left),
                right: Some(Box::new(r)),
                operation,
                details,
                sub_steps,
            })
        } else {
            Some(Step {
                left: Box::new(left),
                right: None,
                operation,
                details,
                sub_steps,
            })
        }
    }

    pub fn report(&self) -> Vec<String> {
        let mut rep = vec![];
        rep.push(format!(
            "{} {} and {}",
            self.details,
            self.left.to_tex(),
            self.right
                .clone()
                .unwrap_or(Box::new(Variable::from_tex("0").unwrap()))
                .to_tex()
        ));
        for step in self.sub_steps.iter() {
            rep.extend(step.report());
        }
        rep
    }
}

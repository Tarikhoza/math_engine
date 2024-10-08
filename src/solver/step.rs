use crate::castable::Castable;
use crate::math::operator::Operator;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable};

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
    pub fn new_step(
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
                .unwrap_or(Box::new(0_i64.as_variable().as_math()))
                .to_tex()
        ));
        for step in self.sub_steps.iter() {
            rep.extend(step.report());
        }
        rep
    }
}

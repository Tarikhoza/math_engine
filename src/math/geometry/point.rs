use crate::math::AlgebraOperations;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, ParsableGenericsAsVariable};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub coordinates: Vec<Math>,
}

impl Point {
    pub fn distance(&self, other: &Point) -> Math {
        let ret = Math::default();
        let mut exp = String::new();

        let first = self.normalise_system(&other);
        let second = other.normalise_system(&first);

        for i in 0..first.coordinates.len() {
            if exp.is_empty() {
                exp = format!(
                    "({}-{})^{{{}}}",
                    first.coordinates[i].to_tex(),
                    second.coordinates[i].to_tex(),
                    2
                );
            } else {
                exp = format!(
                    "{}+({}-{})^{{{}}}",
                    exp,
                    first.coordinates[i].to_tex(),
                    second.coordinates[i].to_tex(),
                    2
                );
            }
        }
        format!("\\sqrt[2]{{{}}}", exp)
            .parse_math()
            .unwrap()
            .simplify()
    }
    pub fn normalise_system(&self, other: &Point) -> Point {
        let mut normalised = (*self).clone();
        while (normalised.coordinates.len() < other.coordinates.len()) {
            normalised.coordinates.push(0.parse_math().unwrap());
        }
        normalised
    }
}

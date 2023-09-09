use crate::math::geometry::line::Line;
use crate::math::geometry::point::Point;
use crate::math::simplifiable::Simplifiable;
use crate::math::AlgebraOperations;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive};

pub struct Angle {
    pub line_a: Line,
    pub line_b: Line,
}

impl Angle {
    pub fn new(line_a: Line, line_b: Line) -> Self {
        if line_a.point_a != line_b.point_a {
            return Self {
                line_a: line_a.swap_points(),
                line_b,
            };
        } else {
            return Self {
                line_a: line_b.swap_points(),
                line_b,
            };
        }
        Self { line_a, line_b }
    }

    pub fn slope(&self) -> Math {
        let points = self.get_points();
        let mut exp: Vec<String> = vec![];
        for i in 0..points.1 .0.coordinates.len() {
            exp.push(format!(
                "({}-{})",
                points.1 .0.coordinates[i].to_tex(),
                points.1 .1.coordinates[i].to_tex()
            ));
        }
        exp.join("/")
            .parse_math()
            .expect("failed parsing math")
            .simplify()
    }

    pub fn get_points(&self) -> (Point, (Point, Point)) {
        (
            self.line_a.point_a.clone(),
            (self.line_a.point_b.clone(), self.line_b.point_b.clone()),
        )
    }

    //    pub fn degree(&self) -> Math {}
}

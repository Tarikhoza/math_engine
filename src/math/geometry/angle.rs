use crate::math::geometry::line::Line;
use crate::math::geometry::point::Point;
use crate::math::AlgebraOperations;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics};

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
        } else if line_a.point_b != line_b.point_b {
            return Self {
                line_a: line_a.swap_points(),
                line_b,
            };
        }
        return Self { line_a, line_b };
    }
    //    pub fn degree(&self) -> Math {}
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
        dbg!(exp.clone().join("/"));
        exp.join("/").parse_math().unwrap().simplify()
    }

    // return (joint_point, (other_point, other_point))
    pub fn get_points(&self) -> (Point, (Point, Point)) {
        return (
            self.line_a.point_a.clone(),
            (self.line_a.point_b.clone(), self.line_b.point_b.clone()),
        );
    }
}

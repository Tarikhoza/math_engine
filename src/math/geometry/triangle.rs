use crate::math::geometry::line::Line;
use crate::math::geometry::point::Point;

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub line_a: Line,
    pub line_b: Line,
    pub line_c: Line,
    pub height: Line,
}

impl Triangle {
    pub fn hypothenuse(&self) -> Line {
        todo!();
    }
    pub fn oposite(&self) -> Line {
        todo!();
    }
    pub fn adjecent(&self) -> Line {
        todo!();
    }
}

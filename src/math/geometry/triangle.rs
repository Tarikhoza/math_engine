use crate::math::geometry::line::Line;
use crate::math::geometry::point::Point;
use crate::math::Math;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub hypothenuse: Option<Line>,
    pub opposite: Option<Line>,
    pub adjecent: Option<Line>,
}

impl Triangle {
    pub fn construct_lines(&self) -> Triangle {
        let mut constructed_triangle = (*self).clone();
        let mut lines = vec![
            Line {
                point_a: constructed_triangle.a.clone(),
                point_b: constructed_triangle.b.clone(),
            },
            Line {
                point_a: constructed_triangle.b.clone(),
                point_b: constructed_triangle.c.clone(),
            },
            Line {
                point_a: constructed_triangle.c.clone(),
                point_b: constructed_triangle.a.clone(),
            },
        ];
        lines.sort_by_key(|line| line.length_as_dec());

        constructed_triangle.opposite = Some(lines[0].clone());
        constructed_triangle.adjecent = Some(lines[1].clone());
        constructed_triangle.hypothenuse = Some(lines[2].clone());
        constructed_triangle
    }

    pub fn from_points(point_a: Point, point_b: Point, point_c: Point) -> Triangle {
        Triangle {
            a: point_a,
            b: point_b,
            c: point_c,
            hypothenuse: None,
            opposite: None,
            adjecent: None,
        }
        .construct_lines()
    }

    pub fn right_from_angle(angle: Math) -> Triangle {
        todo!();

        let first_angle = angle;
        //       let second_angle: Decimal = 90.as_variable();
        //       let third_angle: Decimal = 180.as_variable().sub((first_angle.add(second_angle)));
    }
}

use crate::math::geometry::point::Point;
use crate::math::Math;
use crate::math::Variable;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[derive(Debug, Clone)]
pub struct Line {
    pub point_a: Point,
    pub point_b: Point,
}

impl Line {
    pub fn length(&self) -> Math {
        self.point_a.distance(&self.point_b)
    }

    pub fn length_as_dec(&self) -> Decimal {
        if let Math::Variable(v) = self.point_a.distance(&self.point_b) {
            return v.value;
        }
        dec!(0)
    }

    pub fn swap_points(&self) -> Line {
        Line {
            point_a: self.point_b.clone(),
            point_b: self.point_a.clone(),
        }
    }
}

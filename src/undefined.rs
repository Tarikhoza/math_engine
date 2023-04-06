use crate::math::Math;

use std::ops;
#[derive(Debug, Clone)]
pub struct Undefined {}

impl Undefined {
    #[must_use]
    pub fn to_tex(&self) -> String {
        "undefined".to_string()
    }
}

impl ops::Add<Math> for Undefined {
    type Output = Math;
    fn add(self, _rhs: Math) -> Math {
        Math::Undefined(Undefined {})
    }
}

impl ops::Sub<Math> for Undefined {
    type Output = Math;
    fn sub(self, _rhs: Math) -> Math {
        Math::Undefined(Undefined {})
    }
}

impl ops::Mul<Math> for Undefined {
    type Output = Math;
    fn mul(self, _rhs: Math) -> Math {
        Math::Undefined(Undefined {})
    }
}

impl ops::Div<Math> for Undefined {
    type Output = Math;
    fn div(self, _rhs: Math) -> Math {
        Math::Undefined(Undefined {})
    }
}

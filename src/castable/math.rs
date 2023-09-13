use crate::castable::Castable;
use crate::math::Math;

impl Castable for Math {
    fn as_math(&self) -> Math {
        self.clone()
    }
}

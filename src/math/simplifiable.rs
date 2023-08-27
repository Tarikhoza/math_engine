use crate::math::Math;
pub trait Simplifiable {
    fn simplify(&self) -> Math;
    fn simplify_force_number(&self) -> Math {
        todo!()
    }
}

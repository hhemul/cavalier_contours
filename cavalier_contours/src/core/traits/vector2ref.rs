use super::Real;

pub trait Vector2Ref {
    type Num: Real;
    fn x(&self) -> Self::Num;
    fn y(&self) -> Self::Num;
}

pub trait Vector2RefMut: Vector2Ref {
    fn x_mut(&mut self) -> &mut Self::Num;
    fn y_mut(&mut self) -> &mut Self::Num;
}

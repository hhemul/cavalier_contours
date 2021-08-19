use crate::core::traits::{Real, Vector2Ref};

pub trait PlineVertexRef {
    type Num: Real;
    type Vertex: Vector2Ref;
    fn x(&self) -> Self::Num;
    fn y(&self) -> Self::Num;
    fn bulge(&self) -> Self::Num;

    fn new(x: Self::Num, y: Self::Num) -> Self;
}

pub trait PlineVertexRefMut: PlineVertexRef {
    fn x_mut(&self) -> &mut Self::Num;
    fn y_mut(&self) -> &mut Self::Num;
    fn bulge_mut(&self) -> &mut Self::Num;
}

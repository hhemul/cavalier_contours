use crate::{core::traits::Real, polyline::PlineVertex};

pub trait IterablePlineVertexes<'a> {
    type Num: Real;
    type Iter: Iterator<Item = PlineVertex<Self::Num>>;
    fn iter_vertexes(&'a self) -> Self::Iter;
    fn vertex_count(&'a self) -> usize;
}

pub trait IterablePlineVertexesMut<'a> {
    type Num: Real + 'a;
    type Iter: Iterator<Item = &'a mut PlineVertex<Self::Num>> + 'a;
    fn iter_vertexes_mut(&'a mut self) -> Self::Iter;

    fn scale(&'a mut self, scale_factor: Self::Num) {
        for v in self.iter_vertexes_mut() {
            v.x = scale_factor * v.x;
            v.y = scale_factor * v.y;
        }
    }

    fn translate(&'a mut self, x: Self::Num, y: Self::Num) {
        for v in self.iter_vertexes_mut() {
            v.x = v.x + x;
            v.y = v.y + y;
        }
    }
}

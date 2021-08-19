use crate::core::traits::Real;

use super::{IterablePlineSegments, IterablePlineVertexes};

pub trait IterablePolyline<'a, T>:
    IterablePlineVertexes<'a, Num = T> + IterablePlineSegments<'a, Num = T>
where
    T: Real,
{
    fn is_closed(&self) -> bool;
}

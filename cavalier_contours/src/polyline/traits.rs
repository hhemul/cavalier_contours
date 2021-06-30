use std::{collections::binary_heap::Iter, iter::Copied, slice::Windows};

use crate::core::traits::{ControlFlow, Real};

use super::{PlineSegVisitor, PlineVertex, Polyline};

pub trait PlineVertexRef<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn bulge(&self) -> T;
}

impl<T> PlineVertexRef<T> for PlineVertex<T>
where
    T: Real,
{
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }

    fn bulge(&self) -> T {
        self.bulge
    }
}

pub trait PlineVertexRefMut<T> {
    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
    fn bulge_mut(&mut self) -> &mut T;
}

impl<T> PlineVertexRefMut<T> for PlineVertex<T>
where
    T: Real,
{
    fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    fn bulge_mut(&mut self) -> &mut T {
        &mut self.bulge
    }
}

pub trait PolylineRef {
    type Num: Real;
    type Vertex: PlineVertexRef<Self::Num> + Copy + Default;
    fn vertexes(&self) -> &[Self::Vertex];
    fn is_closed(&self) -> bool;
}

pub trait IterablePlineSegments<'a> {
    type Num: Real;
    type Vertex: PlineVertexRef<Self::Num> + 'a;
    type Iter: Iterator<Item = (Self::Vertex, Self::Vertex)>;
    fn iter_segments(&'a self) -> Self::Iter;
}

pub trait ContiguousPolylineRef {
    type Num: Real;
    type Vertex: PlineVertexRef<Self::Num>;
    fn vertexes(&self) -> &[Self::Vertex];
    fn is_closed(&self) -> bool;
}

pub trait ContiguousPolylineRefMut {
    type Num: Real;
    type Vertex: PlineVertexRef<Self::Num>;
    fn vertexes(&mut self) -> &mut [Self::Vertex];
    fn is_closed(&mut self) -> &mut bool;
}

pub trait IterablePolyline<'a> {
    type Num: Real;
    type Vertex: PlineVertexRef<Self::Num> + Copy + Default + 'a;
    type VertexIter: Iterator<Item = Self::Vertex>;
    type SegmentIter: Iterator<Item = (Self::Vertex, Self::Vertex)>;
    fn iter_vertexes(&'a self) -> Self::VertexIter;
    fn is_closed(&self) -> bool;
    fn iter_segments(&'a self) -> Self::SegmentIter;
}

impl<'a, T> IterablePolyline<'a> for T
where
    T: PolylineRef + 'a,
{
    type Num = T::Num;

    type Vertex = T::Vertex;

    type VertexIter = Copied<std::slice::Iter<'a, Self::Vertex>>;
    type SegmentIter = PlineSegIterator3<'a, Self>;

    fn iter_vertexes(&'a self) -> Self::VertexIter {
        self.vertexes().iter().copied()
    }

    fn is_closed(&self) -> bool {
        self.is_closed()
    }

    fn iter_segments(&'a self) -> Self::SegmentIter {
        PlineSegIterator3::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct PlineSegIterator3<'a, Pline>
where
    Pline: IterablePolyline<'a>,
{
    polyline: &'a Pline,
    prev_vertex: Pline::Vertex,
    iter: Pline::VertexIter,
    wrap_not_exhausted: bool,
}

impl<'a, Pline> PlineSegIterator3<'a, Pline>
where
    Pline: IterablePolyline<'a>,
{
    pub fn new(polyline: &'a Pline) -> PlineSegIterator3<'a, Pline> {
        let mut iter = polyline.iter_vertexes();
        let prev_vertex = iter.next().unwrap_or_default();
        let wrap_not_exhausted = polyline.is_closed();
        PlineSegIterator3 {
            polyline,
            prev_vertex,
            iter,
            wrap_not_exhausted,
        }
    }
}

impl<'a, Pline> Iterator for PlineSegIterator3<'a, Pline>
where
    Pline: IterablePolyline<'a>,
{
    type Item = (Pline::Vertex, Pline::Vertex);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v2) = self.iter.next() {
            let v1 = self.prev_vertex;
            self.prev_vertex = v2;
            Some((v1, v2))
        } else if self.wrap_not_exhausted {
            self.wrap_not_exhausted = false;
            if let Some(v) = self.polyline.iter_vertexes().next() {
                Some((self.prev_vertex, v))
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let windows_hint = self.iter.size_hint();
        if self.wrap_not_exhausted {
            (windows_hint.0 + 1, windows_hint.1.map(|h| h + 1))
        } else {
            windows_hint
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlineSegIterator2<'a, Num, Vertex, PlineRef>
where
    Num: Real,
    Vertex: PlineVertexRef<Num> + Copy,
    PlineRef: PolylineRef<Num = Num, Vertex = Vertex>,
{
    polyline: &'a PlineRef,
    vertex_windows: Windows<'a, Vertex>,
    wrap_not_exhausted: bool,
}

impl<'a, Num, Vertex, PlineRef> PlineSegIterator2<'a, Num, Vertex, PlineRef>
where
    Num: Real,
    Vertex: PlineVertexRef<Num> + Copy,
    PlineRef: PolylineRef<Num = Num, Vertex = Vertex>,
{
    fn new(polyline: &'a PlineRef) -> PlineSegIterator2<'a, Num, Vertex, PlineRef> {
        let vertex_windows = polyline.vertexes().windows(2);
        let wrap_not_exhausted = if polyline.vertexes().len() < 2 {
            false
        } else {
            polyline.is_closed()
        };
        PlineSegIterator2 {
            polyline,
            vertex_windows,
            wrap_not_exhausted,
        }
    }
}

impl<'a, Num, Vertex, PlineRef> Iterator for PlineSegIterator2<'a, Num, Vertex, PlineRef>
where
    Num: Real,
    Vertex: PlineVertexRef<Num> + Copy,
    PlineRef: PolylineRef<Num = Num, Vertex = Vertex>,
{
    type Item = (Vertex, Vertex);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&[v1, v2]) = self.vertex_windows.next() {
            Some((v1, v2))
        } else if self.wrap_not_exhausted {
            self.wrap_not_exhausted = false;
            let ln = self.polyline.vertexes().len();
            Some((
                self.polyline.vertexes()[ln - 1],
                self.polyline.vertexes()[0],
            ))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let windows_hint = self.vertex_windows.size_hint();
        if self.wrap_not_exhausted {
            (windows_hint.0 + 1, windows_hint.1.map(|h| h + 1))
        } else {
            windows_hint
        }
    }
}

impl<'a, T> IterablePlineSegments<'a> for T
where
    T: PolylineRef + 'a,
{
    type Num = T::Num;

    type Vertex = T::Vertex;

    type Iter = PlineSegIterator2<'a, Self::Num, Self::Vertex, T>;

    fn iter_segments(&'a self) -> Self::Iter {
        PlineSegIterator2::new(self)
    }
}

// pub trait VisitablePlineSegments {
//     type Num: Real;

//     fn visit_segments<V, C>(&self, visitor: &mut V) -> C
//     where
//         C: ControlFlow,
//         V: PlineSegVisitor<Self::Num, C>;
// }

// impl<'a, T, U, P> VisitablePlineSegments for T
// where
//     U: Real,
//     P: PlineVertexTrait<U>,
//     T: ContiguousPlineVertexes<Num = U, Vertex = P>,
// {
//     type Num = U;

//     fn visit_segments<V, C>(&self, visitor: &mut V) -> C
//     where
//         C: ControlFlow,
//         V: PlineSegVisitor<Self::Num, C>,
//     {
//         let mut iter = self.segments().windows(2);
//         while let Some(&[v1, v2]) = iter.next() {
//             try_cf!(visitor.visit_seg(v1, v2));
//         }

//         C::continuing()
//     }
// }

// pub trait IterablePlineVertexes<'a> {
//     type Num: Real + 'a;
//     type Iter: Iterator<Item = &'a PlineVertex<Self::Num>>;
//     fn vertexes_iter(&'a self) -> Self::Iter;
// }

// impl<'a, T> IterablePlineVertexes<'a> for T
// where
//     T: ContiguousPlineVertexes + 'a,
// {
//     type Num = T::Num;

//     type Iter = std::slice::Iter<'a, PlineVertex<Self::Num>>;

//     fn vertexes_iter(&'a self) -> Self::Iter {
//         self.segments().iter()
//     }
// }

// pub trait IterablePlineSegments<'a> {
//     type Num: Real + 'a;
//     type Iter: Iterator<Item = &'a (PlineVertex<Self::Num>, PlineVertex<Self::Num>)>;
//     fn segments_iter(&'a self) -> Self::Iter;
// }

// impl<'a, T> IterablePlineSegments<'a> for T
// where
//     T: ContiguousPlineVertexes + 'a,
// {
//     type Num = T::Num;

//     type Iter = std::slice::Iter<'a, (PlineVertex<Self::Num>, PlineVertex<Self::Num>)>;

//     fn segments_iter(&'a self) -> Self::Iter {}
// }

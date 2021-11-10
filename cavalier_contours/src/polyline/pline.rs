use super::{
    internal::{pline_boolean::polyline_boolean, pline_offset::parallel_offset},
    BooleanOp, BooleanResult, PlineBooleanOptions, PlineOffsetOptions, PlineVertex,
};
use crate::{core::traits::Real, polyline::PolylineRef};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ops::{Index, IndexMut},
    slice::Windows,
};

/// Polyline represented by a sequence of [PlineVertex](crate::polyline::PlineVertex) and a bool
/// indicating whether the polyline is open (last vertex is end of polyline) or closed (last vertex
/// forms segment with first vertex).
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone)]
pub struct Polyline<T = f64> {
    #[cfg_attr(feature = "serde", serde(rename = "vertexes"))]
    pub vertex_data: Vec<PlineVertex<T>>,
    pub is_closed: bool,
}

impl<T> Default for Polyline<T>
where
    T: Real,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Polyline<T>
where
    T: Real,
{
    /// Create a new empty [Polyline] with `is_closed` set to false.
    pub fn new() -> Self {
        Polyline {
            vertex_data: Vec::new(),
            is_closed: false,
        }
    }

    /// Create a new empty [Polyline] with `is_closed` set to true.
    pub fn new_closed() -> Self {
        Polyline {
            vertex_data: Vec::new(),
            is_closed: true,
        }
    }

    /// Visit all the polyline segments (represented as polyline vertex pairs, starting at indexes
    /// (0, 1)) with a function/closure.
    ///
    /// This is equivalent to [Polyline::iter_segments] but uses a visiting function rather than an
    /// iterator.
    pub fn visit_segments<F>(&self, visitor: &mut F)
    where
        F: FnMut(PlineVertex<T>, PlineVertex<T>) -> bool,
    {
        let ln = self.vertex_data.len();
        if ln < 2 {
            return;
        }

        let mut windows = self.vertex_data.windows(2);
        while let Some(&[v1, v2]) = windows.next() {
            if !visitor(v1, v2) {
                return;
            }
        }

        if self.is_closed {
            let v1 = self.vertex_data[ln - 1];
            let v2 = self.vertex_data[0];
            visitor(v1, v2);
        }
    }
    /// Perform a boolean `operation` between this polyline and another using default options.
    ///
    /// See [Polyline::boolean_opt] for more information.
    ///
    /// # Examples
    /// ```
    /// # use cavalier_contours::core::traits::*;
    /// # use cavalier_contours::polyline::*;
    /// # use cavalier_contours::pline_closed;
    /// let rectangle = pline_closed![
    ///     (-1.0, -2.0, 0.0),
    ///     (3.0, -2.0, 0.0),
    ///     (3.0, 2.0, 0.0),
    ///     (-1.0, 2.0, 0.0),
    /// ];
    /// let circle = pline_closed![(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)];
    /// let results = rectangle.boolean(&circle, BooleanOp::Not);
    /// // since the circle is inside the rectangle we get back 1 positive polyline and 1 negative
    /// // polyline where the positive polyline is the rectangle and the negative polyline is the
    /// // circle
    /// assert_eq!(results.pos_plines.len(), 1);
    /// assert_eq!(results.neg_plines.len(), 1);
    /// assert!(results.pos_plines[0].pline.area().fuzzy_eq(rectangle.area()));
    /// assert!(results.neg_plines[0].pline.area().fuzzy_eq(circle.area()));
    /// ```
    pub fn boolean(&self, other: &Polyline<T>, operation: BooleanOp) -> BooleanResult<T> {
        self.boolean_opt(other, operation, &Default::default())
    }

    /// Perform a boolean `operation` between this polyline and another with options provided.
    ///
    /// Returns the boolean result polylines and their associated slices that were stitched together
    /// end to end to form them.
    ///
    /// # Examples
    /// ```
    /// # use cavalier_contours::core::traits::*;
    /// # use cavalier_contours::polyline::*;
    /// # use cavalier_contours::pline_closed;
    /// let rectangle = pline_closed![
    ///     (-1.0, -2.0, 0.0),
    ///     (3.0, -2.0, 0.0),
    ///     (3.0, 2.0, 0.0),
    ///     (-1.0, 2.0, 0.0),
    /// ];
    /// let circle = pline_closed![(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)];
    /// let aabb_index = rectangle.create_approx_aabb_index().unwrap();
    /// let options = PlineBooleanOptions {
    ///     // passing in existing spatial index of the polyline segments for the first polyline
    ///     pline1_aabb_index: Some(&aabb_index),
    ///     ..Default::default()
    /// };
    /// let results = rectangle.boolean_opt(&circle, BooleanOp::Not, &options);
    /// // since the circle is inside the rectangle we get back 1 positive polyline and 1 negative
    /// // polyline where the positive polyline is the rectangle and the negative polyline is the
    /// // circle
    /// assert_eq!(results.pos_plines.len(), 1);
    /// assert_eq!(results.neg_plines.len(), 1);
    /// assert!(results.pos_plines[0].pline.area().fuzzy_eq(rectangle.area()));
    /// assert!(results.neg_plines[0].pline.area().fuzzy_eq(circle.area()));
    /// ```
    pub fn boolean_opt(
        &self,
        other: &Polyline<T>,
        operation: BooleanOp,
        options: &PlineBooleanOptions<T>,
    ) -> BooleanResult<T> {
        polyline_boolean(self, other, operation, options)
    }
}

/// Internal type used by [Polyline::remove_redundant].
impl<T> Index<usize> for Polyline<T> {
    type Output = PlineVertex<T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.vertex_data[index]
    }
}

impl<T> IndexMut<usize> for Polyline<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vertex_data[index]
    }
}

/// An iterator that traverses all segment vertex pairs.
#[derive(Debug, Clone)]
pub struct PlineSegIterator<'a, T>
where
    T: Real,
{
    polyline: &'a Polyline<T>,
    vertex_windows: Windows<'a, PlineVertex<T>>,
    wrap_not_exhausted: bool,
}

impl<'a, T> PlineSegIterator<'a, T>
where
    T: Real,
{
    pub fn new(polyline: &'a Polyline<T>) -> PlineSegIterator<'a, T> {
        let vertex_windows = polyline.vertex_data.windows(2);
        let wrap_not_exhausted = if polyline.vertex_data.len() < 2 {
            false
        } else {
            polyline.is_closed
        };
        PlineSegIterator {
            polyline,
            vertex_windows,
            wrap_not_exhausted,
        }
    }
}

impl<'a, T> Iterator for PlineSegIterator<'a, T>
where
    T: Real,
{
    type Item = (PlineVertex<T>, PlineVertex<T>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&[v1, v2]) = self.vertex_windows.next() {
            Some((v1, v2))
        } else if self.wrap_not_exhausted {
            self.wrap_not_exhausted = false;
            let vc = self.polyline.vertex_count();
            Some((self.polyline[vc - 1], self.polyline[0]))
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

use static_aabb2d_index::AABB;

use crate::{
    core::traits::Real,
    polyline::{arc_seg_bounding_box, seg_length, PlineVertex},
};
use num_traits::Zero;

pub trait IterablePlineSegments<'a> {
    type Num: Real;
    type Iter: Iterator<Item = (PlineVertex<Self::Num>, PlineVertex<Self::Num>)> + 'a;
    fn iter_segments(&'a self) -> Self::Iter;
    fn segment_count(&'a self) -> usize;

    fn extents(&'a self) -> Option<AABB<Self::Num>> {
        if self.segment_count() == 0 {
            return None;
        }

        let mut result = AABB::new(
            Self::Num::max_value(),
            Self::Num::max_value(),
            Self::Num::min_value(),
            Self::Num::min_value(),
        );

        for (v1, v2) in self.iter_segments() {
            if v1.bulge_is_zero() {
                // line segment, just look at end of line point
                if v2.x < result.min_x {
                    result.min_x = v2.x;
                } else if v2.x > result.max_x {
                    result.max_x = v2.x;
                }

                if v2.y < result.min_y {
                    result.min_y = v2.y;
                } else if v2.y > result.max_y {
                    result.max_y = v2.y;
                }

                continue;
            }
            // else arc segment
            let arc_extents = arc_seg_bounding_box(v1, v2);

            result.min_x = num_traits::real::Real::min(result.min_x, arc_extents.min_x);
            result.min_y = num_traits::real::Real::min(result.min_y, arc_extents.min_y);
            result.max_x = num_traits::real::Real::max(result.max_x, arc_extents.max_x);
            result.max_y = num_traits::real::Real::max(result.max_y, arc_extents.max_y);
        }

        Some(result)
    }

    /// Returns the total path length of the polyline.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cavalier_contours::polyline::*;
    /// # use cavalier_contours::core::traits::*;
    /// let mut polyline: Polyline = Polyline::new();
    /// // open polyline half circle
    /// polyline.add(0.0, 0.0, 1.0);
    /// polyline.add(2.0, 0.0, 1.0);
    /// assert!(polyline.path_length().fuzzy_eq(std::f64::consts::PI));
    /// // close into full circle
    /// polyline.set_is_closed(true);
    /// assert!(polyline.path_length().fuzzy_eq(2.0 * std::f64::consts::PI));
    /// ```
    #[inline]
    fn path_length(&'a self) -> Self::Num {
        self.iter_segments()
            .fold(Self::Num::zero(), |acc, (v1, v2)| acc + seg_length(v1, v2))
    }
}

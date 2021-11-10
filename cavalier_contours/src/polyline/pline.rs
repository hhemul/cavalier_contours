use super::PlineVertex;
use crate::core::traits::Real;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

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

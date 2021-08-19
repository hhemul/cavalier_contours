use crate::{
    core::traits::{FuzzyEq, Real},
    polyline::PlineVertex,
};

pub trait PolylineContiguousStorage {
    type Num: Real;
    /// Read only reference to the vertexes of the polyline as a contiguous slice.
    fn as_slice(&self) -> &[PlineVertex<Self::Num>];
    /// Returns true if the polyline is closed, false if it is open.
    fn is_closed(&self) -> bool;
    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

pub trait PolylineContiguousStorageMut: PolylineContiguousStorage {
    /// Mutable reference to the vertexes of the polyline as a contiguous slice.
    fn as_mut_slice(&mut self) -> &mut [PlineVertex<Self::Num>];
    /// Sets the polyline to be closed or not.
    fn set_is_closed(&mut self, is_closed: bool);

    /// Reserves capacity for at least `additional` more elements.
    fn reserve(&mut self, additional: usize);

    /// Add a vertex to the polyline by giving a [PlineVertex](crate::polyline::PlineVertex).
    fn add_vertex(&mut self, vertex: PlineVertex<Self::Num>);

    /// Add a vertex to the polyline by giving the `x`, `y`, and `bulge` values of the vertex.
    #[inline]
    fn add(&mut self, x: Self::Num, y: Self::Num, bulge: Self::Num) {
        self.add_vertex(PlineVertex::new(x, y, bulge))
    }

    /// Insert a new vertex into the polyline at position `index` by giving a
    /// [PlineVertex](crate::polyline::PlineVertex).
    fn insert_vertex(&mut self, index: usize, vertex: PlineVertex<Self::Num>);

    /// Set the vertex data at a given index of the polyline.
    #[inline]
    fn set_vertex(&mut self, index: usize, x: Self::Num, y: Self::Num, bulge: Self::Num) {
        let v = &mut self.as_mut_slice()[index];
        v.x = x;
        v.y = y;
        v.bulge = bulge;
    }

    /// Add a vertex if it's position is not fuzzy equal to the last vertex in the polyline.
    ///
    /// If the vertex position is fuzzy equal then just update the bulge of the last vertex with
    /// the bulge given.
    #[inline]
    fn add_or_replace(
        &mut self,
        x: Self::Num,
        y: Self::Num,
        bulge: Self::Num,
        pos_equal_eps: Self::Num,
    ) {
        let ln = self.len();
        if ln == 0 {
            self.add(x, y, bulge);
            return;
        }

        let last_vert = &mut self.as_mut_slice()[ln - 1];
        if last_vert.x.fuzzy_eq_eps(x, pos_equal_eps) && last_vert.y.fuzzy_eq_eps(y, pos_equal_eps)
        {
            last_vert.bulge = bulge;
            return;
        }

        self.add(x, y, bulge);
    }

    /// Add a vertex if it's position is not fuzzy equal to the last vertex in the polyline.
    ///
    /// If the vertex position is fuzzy equal then just update the bulge of the last vertex with
    /// the bulge given.
    #[inline]
    fn add_or_replace_vertex(&mut self, vertex: PlineVertex<Self::Num>, pos_equal_eps: Self::Num) {
        self.add_or_replace(vertex.x, vertex.y, vertex.bulge, pos_equal_eps)
    }

    /// Copy all vertexes from `other` to the end of this polyline.
    #[inline]
    fn extend<P>(&mut self, other: &P)
    where
        P: PolylineContiguousStorage<Num = Self::Num>,
    {
        self.reserve(other.len());
        other.as_slice().iter().for_each(|v| self.add_vertex(*v));
    }

    /// Add all vertexes to the end of this polyline.
    fn extend_vertexes<I>(&mut self, vertexes: I)
    where
        I: IntoIterator<Item = PlineVertex<Self::Num>>;

    /// Remove vertex at index.
    fn remove(&mut self, index: usize) -> PlineVertex<Self::Num>;

    /// Remove last vertex.
    #[inline]
    fn remove_last(&mut self) -> PlineVertex<Self::Num> {
        self.remove(self.len() - 1)
    }

    /// Clear all vertexes.
    fn clear(&mut self);

    /// Get the last vertex or None if there are no vertexes.
    #[inline]
    fn last(&self) -> Option<&PlineVertex<Self::Num>> {
        self.as_slice().last()
    }

    /// Get a mutable reference to the last vertex or None if there are no vertexes.
    #[inline]
    fn last_mut(&mut self) -> Option<&mut PlineVertex<Self::Num>> {
        self.as_mut_slice().last_mut()
    }

    /// Fuzzy equal comparison with another polyline using `fuzzy_epsilon` given.
    #[inline]
    fn fuzzy_eq_eps(&self, other: &Self, fuzzy_epsilon: Self::Num) -> bool {
        self.len() == other.len()
            && self
                .as_slice()
                .iter()
                .zip(other.as_slice())
                .all(|(v1, v2)| v1.fuzzy_eq_eps(*v2, fuzzy_epsilon))
    }

    /// Fuzzy equal comparison with another vertex using Self::Num::fuzzy_epsilon().
    #[inline]
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.fuzzy_eq_eps(other, Self::Num::fuzzy_epsilon())
    }

    /// Invert/reverse the direction of the polyline in place.
    ///
    /// This method works by simply reversing the order of the vertexes, shifting by 1 position all
    /// the vertexes, and inverting the sign of all the bulge values. E.g. after reversing the
    /// vertex the bulge at index 0 becomes negative bulge at index 1. The end result for a closed
    /// polyline is the direction will be changed from clockwise to counter clockwise or vice versa.
    fn invert_direction(&mut self) {
        let ln = self.len();
        if ln < 2 {
            return;
        }
        let is_closed = self.is_closed();

        let s = self.as_mut_slice();
        s.reverse();

        let first_bulge = s[0].bulge;
        for i in 1..ln {
            s[i - 1].bulge = -s[i].bulge;
        }

        if is_closed {
            s[ln - 1].bulge = -first_bulge;
        }
    }
}

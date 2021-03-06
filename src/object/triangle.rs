//! Defines [`Triangle`].

use num_traits::Float;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::vector::Vector;

/// A triangle
///
/// Defined by three Vector.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Triangle<F: Float, const N: usize> {
    points: Vector<Vector<F, N>, 3>,
}

impl<F: Float, const N: usize> Triangle<F, N> {
    /// Get the points of the triangle.
    pub const fn points(&self) -> &Vector<Vector<F, N>, 3> {
        &self.points
    }

    /// Get the points of the triangle as an array.
    pub const fn points_array(&self) -> &[Vector<F, N>; 3] {
        self.points.data()
    }
}

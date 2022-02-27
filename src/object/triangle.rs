use crate::vector::Vector;
use num_traits::Float;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub struct Triangle<F: Float, const N: usize> {
    vector: Vector<Vector<F, N>, 3>,
}

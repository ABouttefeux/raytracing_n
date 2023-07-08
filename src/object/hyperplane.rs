use num_traits::Float;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::Vector;

// H is the hyper dimention
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct HyperPlane<F: Float, const N: usize, const H: usize> {
    vecs: Vector<Vector<F, N>, H>,
    offset: Vector<F, N>,
}

impl<F: Float, const N: usize, const H: usize> HyperPlane<F, N, H> {
    pub fn new(vecs: Vector<Vector<F, N>, H>, offset: Vector<F, N>) -> Option<Self> {
        if H > N {
            None
        } else {
            todo!()
        }
    }
}

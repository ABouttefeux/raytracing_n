#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Direction<const N: usize> {
    index: usize,
}

impl<const N: usize> Direction<N> {
    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn new(index: usize) -> Option<Self> {
        if index < N {
            Some(Self { index })
        } else {
            None
        }
    }
}

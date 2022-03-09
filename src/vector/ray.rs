use std::ops::Div;

use num_traits::Float;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::utils::{Norm, Normed};
use crate::Vector;

/// Semi open line
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Ray<F, const N: usize>
where
    F: Float + Clone + std::iter::Sum,
    Vector<F, N>: Div<<Vector<F, N> as Norm>::Output, Output = Vector<F, N>>,
{
    starting_point: Vector<F, N>,
    direction: Normed<Vector<F, N>>,
}

impl<F, const N: usize> Ray<F, N>
where
    F: Float + Clone + std::iter::Sum,
    Vector<F, N>: Div<<Vector<F, N> as Norm>::Output, Output = Vector<F, N>>,
{
    pub fn new(starting_point: Vector<F, N>, direction: Normed<Vector<F, N>>) -> Self {
        Self {
            starting_point,
            direction,
        }
    }

    pub fn new_unormed_direction(
        starting_point: Vector<F, N>,
        direction: Vector<F, N>,
    ) -> Option<Self> {
        Some(Self {
            starting_point,
            direction: Normed::new(direction)?,
        })
    }

    pub fn starting_point(&self) -> &Vector<F, N> {
        &self.starting_point
    }

    pub fn direction(&self) -> &Normed<Vector<F, N>> {
        &self.direction
    }
}

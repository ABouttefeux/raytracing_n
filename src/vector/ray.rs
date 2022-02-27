use super::Vector;
use crate::utils::{Norm, Normed};
use num_traits::Float;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};
use std::ops::Div;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Ray<F, const N: usize>
where
    F: Float + Clone + std::iter::Sum,
    Vector<F, N>: Div<<Vector<F, N> as Norm>::Output, Output = Vector<F, N>>,
{
    offset: Vector<F, N>,
    direction: Normed<Vector<F, N>>,
}

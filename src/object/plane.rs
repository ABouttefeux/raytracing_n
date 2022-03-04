use std::error::Error;
use std::num::FpCategory;

use num_traits::Float;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::vector::Vector;

/// Error return by the [`Plane::new`] function.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum PlaneBuildingError {
    /// The two vectors are colinear.
    ///
    /// And therefore cannot define a plane.
    ColinearVector,
    /// A vector has a norm that is too small i.e. [`FpCategory::Zero`] or i.e. [`FpCategory::Subzero`]
    NormTooSmall,
    /// The norm is not clasify as normal, i.e. [`FpCategory::Infinite`]
    /// or [`FpCategory::Nan`]
    NotNormalNorm,
}

impl std::fmt::Display for PlaneBuildingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ColinearVector => write!(f, "the vectors are colinear"),
            Self::NormTooSmall => write!(f, "at least one vector has a norm that is too small"),
            Self::NotNormalNorm => write!(f, "the norm is infinite or NaN"),
        }
    }
}

impl Error for PlaneBuildingError {}

/// Define a plane in the space.
///
/// It is defined by two non null vectors that are not colinear.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Plane<F: Float, const N: usize> {
    vecs: [Vector<F, N>; 2],
}
impl<F: Float + std::iter::Sum, const N: usize> Plane<F, N> {
    /// Try creating a plane from two vectors.
    ///
    /// # Errors
    /// - [`PlaneBuildingError::ColinearVector`] if the two vectors are colinear.
    /// - [`PlaneBuildingError::NormTooSmall`] if the norm of at least one vector is too small.
    /// - [`PlaneBuildingError::NotNormalNorm`] if the norm of at least one vector is infinite or NaN.
    ///
    /// # Examples
    // TODO
    pub fn new(v1: Vector<F, N>, v2: Vector<F, N>) -> Result<Self, PlaneBuildingError> {
        match v1.norm().classify() {
            FpCategory::Subnormal | FpCategory::Zero => {
                return Err(PlaneBuildingError::NormTooSmall)
            }
            FpCategory::Infinite | FpCategory::Nan => {
                return Err(PlaneBuildingError::NotNormalNorm)
            }
            FpCategory::Normal => {}
        }

        match v2.norm().classify() {
            FpCategory::Subnormal | FpCategory::Zero => {
                return Err(PlaneBuildingError::NormTooSmall)
            }
            FpCategory::Infinite | FpCategory::Nan => {
                return Err(PlaneBuildingError::NotNormalNorm)
            }
            FpCategory::Normal => {}
        }

        if v1.are_colinear(v2) {
            Err(PlaneBuildingError::ColinearVector)
        } else {
            Ok(Self { vecs: [v1, v2] })
        }
    }
}

impl<F: Float, const N: usize> Plane<F, N> {
    /// Return the two vectors defining the plane.
    pub fn vectors(&self) -> &[Vector<F, N>; 2] {
        &self.vecs
    }
}

//TODO

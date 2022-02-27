use crate::vector::Vector;
use num_traits::Float;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::num::FpCategory;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum PlaneBuildingError {
    /// the two vectors are colinear
    ColinearVector,
    /// A vector has a norm that is too small i.e. [`FpCategory::Zero`] or i.e. [`FpCategory::Subzero`]
    NormTooSmall,
    /// The norm is not clasify as normal, i.e. [`FpCategory::Infinite`]
    // [`FpCategory::Nan`]
    NotNormalNorm,
}

impl core::fmt::Display for PlaneBuildingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ColinearVector => write!(f, "the vectors are colinear"),
            Self::NormTooSmall => write!(f, "at least one vector has a norm that is too small"),
            Self::NotNormalNorm => write!(f, "the norm is infinite or NaN"),
        }
    }
}

impl Error for PlaneBuildingError {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub struct Plane<F: Float, const N: usize> {
    vecs: [Vector<F, N>; 2],
}
impl<F: Float + std::iter::Sum, const N: usize> Plane<F, N> {
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
    pub fn vectors(&self) -> &[Vector<F, N>; 2] {
        &self.vecs
    }
}

//TODO

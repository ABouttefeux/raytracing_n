use std::error::Error;
use std::num::FpCategory;
use std::ops::Div;

use num_traits::Float;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::utils::Norm;
use crate::{Ray, Vector};

/// Error return by the [`OriginPlane::new`] function.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum PlaneBuildingError {
    /// The two vectors are colinear.
    ///
    /// And therefore cannot define a plane.
    ColinearVector,
    /// A vector has a norm that is too small i.e. [`FpCategory::Zero`] or i.e. [`FpCategory::Subzero`]
    NormTooSmall,
    /// The norm is not classify as normal, i.e. [`FpCategory::Infinite`]
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
pub struct OriginPlane<F: Float, const N: usize> {
    vecs: [Vector<F, N>; 2],
}
impl<F: Float + std::iter::Sum, const N: usize> OriginPlane<F, N> {
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

impl<F: Float, const N: usize> OriginPlane<F, N> {
    /// Return the two vectors defining the plane.
    pub const fn vectors(&self) -> &[Vector<F, N>; 2] {
        &self.vecs
    }
}

// TODO

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
//#[derive(Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Plane<F: Float, const N: usize> {
    data: PlaneEnum<F, N>,
}

impl<F: Float, const N: usize> Plane<F, N> {
    pub const fn new_tree_points(vecs: Vector<Vector<F, N>, 3>) -> Self {
        Self {
            data: PlaneEnum::ThreePoints(vecs),
        }
    }

    pub const fn new_origin_plane_with_offset(
        origin_plane: OriginPlane<F, N>,
        offset: Vector<F, N>,
    ) -> Self {
        Self {
            data: PlaneEnum::OriginPlanePlaneWithOffset(OriginPlanePlaneWithOffset::new(
                origin_plane,
                offset,
            )),
        }
    }

    pub const fn new_origin_plane_with_offset_struct(
        plant_with_offset: OriginPlanePlaneWithOffset<F, N>,
    ) -> Self {
        Self {
            data: PlaneEnum::OriginPlanePlaneWithOffset(plant_with_offset),
        }
    }

    pub fn tree_points(self) -> Vector<Vector<F, N>, 3> {
        self.data.tree_points()
    }

    pub fn origin_plane_with_offset(self) -> OriginPlanePlaneWithOffset<F, N> {
        self.data.origin_plane_with_offset()
    }
}

impl<F, const N: usize> Plane<F, N>
where
    F: Float + Clone + std::iter::Sum,
    Vector<F, N>: Div<<Vector<F, N> as Norm>::Output, Output = Vector<F, N>>,
{
    pub fn intersection(&self, ray: &Ray<F, N>) -> Option<Vector<F, N>> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
enum PlaneEnum<F: Float, const N: usize> {
    ThreePoints(Vector<Vector<F, N>, 3>),
    OriginPlanePlaneWithOffset(OriginPlanePlaneWithOffset<F, N>),
}

impl<F: Float, const N: usize> PlaneEnum<F, N> {
    pub fn tree_points(self) -> Vector<Vector<F, N>, 3> {
        match self {
            Self::ThreePoints(tree_points) => tree_points,
            Self::OriginPlanePlaneWithOffset(origin_plane_with_offset) => todo!(),
        }
    }

    pub fn origin_plane_with_offset(self) -> OriginPlanePlaneWithOffset<F, N> {
        match self {
            Self::ThreePoints(tree_points) => todo!(),
            Self::OriginPlanePlaneWithOffset(origin_plane_with_offset) => origin_plane_with_offset,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct OriginPlanePlaneWithOffset<F: Float, const N: usize> {
    pub plane: OriginPlane<F, N>,
    pub offset: Vector<F, N>,
}

impl<F: Float, const N: usize> OriginPlanePlaneWithOffset<F, N> {
    pub const fn new(plane: OriginPlane<F, N>, offset: Vector<F, N>) -> Self {
        Self { plane, offset }
    }

    pub const fn plane(&self) -> &OriginPlane<F, N> {
        &self.plane
    }

    pub const fn offset(&self) -> &Vector<F, N> {
        &self.offset
    }

    pub fn plane_mut(&mut self) -> &mut OriginPlane<F, N> {
        &mut self.plane
    }

    pub fn offset_mut(&mut self) -> &mut Vector<F, N> {
        &mut self.offset
    }

    pub const fn deconstruct(self) -> (OriginPlane<F, N>, Vector<F, N>) {
        (self.plane, self.offset)
    }
}

// impl<F: Float + Default, const N: usize> Default for PlaneWithOffsetEnum<F, N> {
//     fn default() -> Self {
//         Self::TwoVecWithOffset(Vector::default(), Vector::default())
//     }
// }

//! Modules that defines the [`Camera`] structures and everithin that is needed for it.

use std::ops::SubAssign;

use num_traits::{Float, FloatConst};
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::angle::Angle;
use crate::object::Plane;
use crate::transformation::Transformable;
use crate::vector::Vector;

/// The camera defines wher will be projecting the rays and rendering the sreen.
///
/// It contains many informations like the directon of observation...
/// TODO
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Camera<F: Float + FloatConst, const N: usize> {
    position: Vector<F, N>,
    //observation: OrientedPlane<F, N>,
    observation_direction: Vector<F, N>,
    observation_angle: Angle<F>,
    //------
    fov: [Angle<F>; 2],
    min_dist: F,
    focal_distance: F,
    fog: F,
    foccus: Foccus<F, N>, // TODO better name
}

impl<F: Float + FloatConst, const N: usize> Camera<F, N> {
    /// Get the position of the camera.
    pub fn position(&self) -> &Vector<F, N> {
        &self.position
    }

    /// Get a mutable reference to the position of the camera.
    pub fn position_mut(&mut self) -> &mut Vector<F, N> {
        &mut self.position
    }
}

impl<F: Float + FloatConst, const N: usize> Transformable<F, N> for Camera<F, N> {
    fn position(&self) -> &Vector<F, N> {
        self.position()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
enum Foccus<F: Float, const N: usize> {
    RefPoint(Vector<F, N>),
    Distance(F),
}

// impl<F: Float + Default, const N: usize> Default for Foccus<F, N> {
//     fn default() -> Self {
//         Self::Distance(F::default())
//     }
// }

impl<F: Float + SubAssign + std::iter::Sum + FloatConst, const N: usize> Foccus<F, N> {
    pub fn distance(&self, camera: &Camera<F, N>) -> F {
        match self {
            Self::Distance(d) => *d,
            Self::RefPoint(pt) => (*camera.position() - *pt).norm(),
        }
    }
}

/// A plane that is oriented in the space.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct OrientedPlane<F: Float + FloatConst, const N: usize> {
    plane: Plane<F, N>,
    angle: Angle<F>,
}

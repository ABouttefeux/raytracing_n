//! Defines the traits [`Transformation`] and [`Transformable`]
//! and some transformation.
// TODO

use std::ops::AddAssign;

use num_traits::{Float, FloatConst};
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::angle::Angle;
use crate::object::OriginePlane;
use crate::vector::Vector;

pub trait Transformation<F: Float, T: Transformable<F, N> + ?Sized, const N: usize> {
    /// Error type of the transformation
    type Error;

    /// Apply a transformation to an object
    /// # Errors
    /// Return an error if the transformation fail
    fn apply(&self, object: T) -> Result<T, Self::Error>
    where
        T: Sized;

    /// Apply a transformation to an object
    /// # Errors
    /// Return an error if the transformation fail
    fn apply_boxed(&self, object: T) -> Result<Box<T>, Self::Error>;
}

pub trait Transformable<F: Float, const N: usize> {
    /// Get the position of the object.
    fn position(&self) -> &Vector<F, N>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Translation<F: Float, const N: usize> {
    /// vector of the translation.
    vec: Vector<F, N>,
}

impl<F: Float, const N: usize> Translation<F, N> {
    /// Create a new translation from a vector
    pub fn new(vec: Vector<F, N>) -> Self {
        Self { vec }
    }

    /// Get the vector of the translation.
    pub fn vec(&self) -> &Vector<F, N> {
        &self.vec
    }

    /// Get the mutable vector of the translation.
    pub fn vec_mut(&mut self) -> &mut Vector<F, N> {
        &mut self.vec
    }
}

impl<F: Float + AddAssign, const N: usize> Transformation<F, Vector<F, N>, N>
    for Translation<F, N>
{
    /// A translation can never fail.
    type Error = crate::utils::Never;

    fn apply(&self, object: Vector<F, N>) -> Result<Vector<F, N>, Self::Error> {
        Ok(object + self.vec)
    }

    fn apply_boxed(&self, object: Vector<F, N>) -> Result<Box<Vector<F, N>>, Self::Error> {
        self.apply(object).map(Box::new)
    }
}

/// Defines a rotation
///
/// angle is the angle of the rotation.
/// center gives around wich point the rotation is done.
/// plane give the plane the rotation is done in.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Rotation<F: Float + FloatConst, const N: usize> {
    /// angle of the rotation.
    angle: Angle<F>,
    /// center of the rotation.
    center: Vector<F, N>, // TODO
    /// plane of rotation.
    plane: OriginePlane<F, N>, //TODO
}

impl<F: Float + FloatConst, const N: usize> Rotation<F, N> {
    /// - `angle` is the angle of the rotation.
    /// - `center` gives around wich point the rotation is done.
    /// - `plane` gives the plane the rotation is done in.
    pub fn new(angle: Angle<F>, center: Vector<F, N>, plane: OriginePlane<F, N>) -> Self {
        Self {
            angle,
            center,
            plane,
        }
    }

    /// Get the angle of the rotation.
    pub fn angle(&self) -> &Angle<F> {
        &self.angle
    }

    /// Get a mutable reference to the angle of the rotation.
    pub fn angle_mut(&mut self) -> &mut Angle<F> {
        &mut self.angle
    }

    /// Get the plane of the rotation.
    pub fn plane(&self) -> &OriginePlane<F, N> {
        &self.plane
    }

    /// Get a mutable reference to the plane of the rotation.
    pub fn plane_mut(&mut self) -> &mut OriginePlane<F, N> {
        &mut self.plane
    }

    /// Get the center of the rotation.
    pub fn center(&self) -> &Vector<F, N> {
        &self.center
    }

    /// Get a mutable reference to the center of the rotation.
    pub fn center_mut(&mut self) -> &mut Vector<F, N> {
        &mut self.center
    }
}

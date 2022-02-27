use crate::angle::Angle;
use crate::object::Plane;
use crate::vector::Vector;
use num_traits::Float;
use std::ops::AddAssign;

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
    fn position(&self) -> &Vector<F, N>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub struct Translation<F: Float, const N: usize> {
    vec: Vector<F, N>,
}

impl<F: Float, const N: usize> Translation<F, N> {
    pub fn new(vec: Vector<F, N>) -> Self {
        Self { vec }
    }

    pub fn vec(&self) -> &Vector<F, N> {
        &self.vec
    }

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub struct Rotation<F: Float, const N: usize> {
    angle: Angle<F>,
    plane: Plane<F, N>, //TODO
}

impl<F: Float, const N: usize> Rotation<F, N> {
    pub fn new(angle: Angle<F>, plane: Plane<F, N>) -> Self {
        Self { angle, plane }
    }

    pub fn angle(&self) -> &Angle<F> {
        &self.angle
    }

    pub fn angle_mut(&mut self) -> &mut Angle<F> {
        &mut self.angle
    }

    pub fn plane(&self) -> &Plane<F, N> {
        &self.plane
    }

    pub fn plane_mut(&mut self) -> &mut Plane<F, N> {
        &mut self.plane
    }
}

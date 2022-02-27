use crate::angle::Angle;
use crate::object::Plane;
use crate::transformation::Transformable;
use crate::vector::Vector;
use num_traits::Float;
use std::ops::SubAssign;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Camera<F: Float, const N: usize> {
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

impl<F: Float, const N: usize> Camera<F, N> {
    pub fn position(&self) -> &Vector<F, N> {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut Vector<F, N> {
        &mut self.position
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Foccus<F: Float, const N: usize> {
    RefPoint(Vector<F, N>),
    Distance(F),
}

// impl<F: Float + Default, const N: usize> Default for Foccus<F, N> {
//     fn default() -> Self {
//         Self::Distance(F::default())
//     }
// }

impl<F: Float + SubAssign + std::iter::Sum, const N: usize> Foccus<F, N> {
    pub fn distance(&self, camera: &Camera<F, N>) -> F {
        match self {
            Self::Distance(d) => *d,
            Self::RefPoint(pt) => (*camera.position() - *pt).norm(),
        }
    }
}

// TODO
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct OrientedPlane<F: Float, const N: usize> {
    plane: Plane<F, N>,
    angle: Angle<F>,
}

impl<F: Float, const N: usize> Transformable<F, N> for Camera<F, N> {
    fn position(&self) -> &Vector<F, N> {
        self.position()
    }
}

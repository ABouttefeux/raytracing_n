//! Defines the object exhibiting in the space used for rendering.

use num_traits::{Float, FloatConst};

mod camera;
mod hyperplane;
mod plane;
mod triangle;

pub use camera::*;
pub use hyperplane::*;
pub use plane::*;
pub use triangle::*;

pub trait Object {}

impl<F: Float, const N: usize> Object for OriginPlane<F, N> {}
impl<F: Float, const N: usize> Object for Triangle<F, N> {}
impl<F: Float + FloatConst, const N: usize> Object for Camera<F, N> {}

pub trait Render: Object {}

impl<F: Float, const N: usize> Render for OriginPlane<F, N> {}
impl<F: Float, const N: usize> Render for Triangle<F, N> {}

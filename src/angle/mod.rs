//! Defines the class [`Angle`]

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{Float, FloatConst, Zero};
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::utils::number::n_2;

/// Get the euclidean reminder
///
/// # Examples
///
/// ```
/// # use raytracing_n::angle::rem_euclidean;
/// # use approx::assert_abs_diff_eq;
/// #
/// assert_abs_diff_eq!(rem_euclidean(2_f64, 1_f64), 0_f64);
/// assert_abs_diff_eq!(rem_euclidean(2.5_f64, 1_f64), 0.5_f64);
/// assert_abs_diff_eq!(rem_euclidean(-2.5_f64, 1_f64), 0.5_f64);
/// assert_abs_diff_eq!(rem_euclidean(2.25_f64, 0.5_f64), 0.25_f64);
///
/// assert!(rem_euclidean(1_f64, 0_f64).is_nan());
/// ```
pub fn rem_euclidean<F: Float>(num: F, div: F) -> F {
    let r = num % div;
    if r < F::zero() {
        r + div.abs()
    } else {
        r
    }
}

/// Represent a angle in radiant.
///
/// It is represented between [0, 2PI[`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Angle<F: Float + FloatConst> {
    radiant: F,
}

impl<F: Float + FloatConst> Angle<F> {
    /// Creat the angle taking the "euclidean reminder" if necessary.
    ///
    /// # Example
    /// ```
    /// # use raytracing_n::Angle;
    /// # use approx::assert_abs_diff_eq;
    /// #
    /// let angle = Angle::new(0_f64);
    /// assert_abs_diff_eq!(*angle.radiant(), 0_f64);
    ///
    /// let angle = Angle::new(-std::f64::consts::PI);
    /// assert_abs_diff_eq!(*angle.radiant(), std::f64::consts::PI);
    /// assert_abs_diff_eq!(angle.degree(), 180_f64);
    ///
    /// let angle = Angle::new(3_f64 * std::f64::consts::PI);
    /// assert_abs_diff_eq!(*angle.radiant(), std::f64::consts::PI);
    /// assert_abs_diff_eq!(angle.degree(), 180_f64);

    ///
    /// let angle = Angle::new(1_f64 + 2_f64 * std::f64::consts::PI);
    /// assert_abs_diff_eq!(*angle.radiant(), 1_f64);
    /// ```
    pub fn new(radiant: F) -> Self {
        Self {
            radiant: rem_euclidean(radiant, F::PI() * n_2()),
        }
    }

    /// Get the value wrapped. It is guarenteed to be in `[0, 2PI[`.
    pub const fn radiant(&self) -> &F {
        &self.radiant
    }

    /// Get a reference mutable to the wrapped value
    ///
    /// BE SURE TO ONLY PUT VALUE BETWEEN `[0, 2PI[`.
    // TODO make a container for mut ref that set mod auto when dropped ?
    fn radiant_mut(&mut self) -> &mut F {
        &mut self.radiant
    }

    /// Get the wrapped value in degree
    ///
    /// # Example
    /// ```
    /// # use raytracing_n::Angle;
    /// # use approx::assert_abs_diff_eq;
    /// #
    /// let angle = Angle::new(0_f64);
    /// assert_abs_diff_eq!(angle.degree(), 0_f64);
    /// let angle = Angle::new(0.5_f64 * std::f64::consts::PI);
    /// assert_abs_diff_eq!(angle.degree(), 90_f64);
    /// let angle = Angle::new(std::f64::consts::PI);
    /// assert_abs_diff_eq!(angle.degree(), 180_f64);
    /// ```
    pub fn degree(&self) -> F {
        self.radiant.to_degrees()
    }

    /// Crate an angle from degree
    ///
    /// # Example
    /// ```
    /// # use raytracing_n::Angle;
    /// # use approx::assert_abs_diff_eq;
    /// #
    /// let angle = Angle::new_degree(0_f64);
    /// assert_abs_diff_eq!(*angle.radiant(), 0_f64);
    ///
    /// let angle = Angle::new_degree(-180_f64);
    /// assert_abs_diff_eq!(*angle.radiant(), std::f64::consts::PI);
    /// assert_abs_diff_eq!(angle.degree(), 180_f64);
    ///
    /// let angle = Angle::new_degree(3_f64 * 180_f64);
    /// assert_abs_diff_eq!(*angle.radiant(), std::f64::consts::PI);
    /// assert_abs_diff_eq!(angle.degree(), 180_f64);
    ///
    /// let angle = Angle::new_degree(72_f64 + 2_f64 * 360_f64);
    /// assert_abs_diff_eq!(*angle.radiant(), 72_f64.to_radians());
    /// assert_abs_diff_eq!(angle.degree(), 72_f64);
    /// ```
    pub fn new_degree(degree: F) -> Self {
        Self::new(degree.to_radians())
    }

    /// Set the angle to the given value wrapping it if necessary
    ///
    /// # Example
    /// ```
    /// # use raytracing_n::Angle;
    /// # use approx::assert_abs_diff_eq;
    /// #
    /// let mut angle = Angle::new(0_f64);
    ///
    /// angle.set_radiant(std::f64::consts::PI);
    /// assert_abs_diff_eq!(*angle.radiant(), std::f64::consts::PI);
    ///
    /// angle.set_radiant(std::f64::consts::PI * 5_f64 / 2_f64);
    /// assert_abs_diff_eq!(*angle.radiant(), std::f64::consts::PI / 2_f64);
    /// ```
    pub fn set_radiant(&mut self, radiant: F) {
        self.radiant = rem_euclidean(radiant, F::PI() * n_2());
    }

    /// Wraps the wrapped value such that the it is between `[0, 2PI[`.
    ///
    /// # Example
    fn mod_radiant(&mut self) {
        self.radiant = rem_euclidean(self.radiant, F::PI() * n_2());
    }
}

impl<F: Float + FloatConst + Add> Add for Angle<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.radiant + other.radiant)
    }
}

impl<F: Float + FloatConst + AddAssign> AddAssign for Angle<F> {
    fn add_assign(&mut self, other: Self) {
        self.radiant += other.radiant;
        self.mod_radiant();
    }
}

impl<F: Float + FloatConst + FloatConst> Sub for Angle<F> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.radiant - other.radiant)
    }
}

impl<F: Float + FloatConst + SubAssign> SubAssign for Angle<F> {
    fn sub_assign(&mut self, other: Self) {
        self.radiant -= other.radiant;
        self.mod_radiant();
    }
}

impl<F: Float + FloatConst> Neg for Angle<F> {
    type Output = Angle<<F as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::new(-self.radiant)
    }
}

impl<F: Float + FloatConst + MulAssign> MulAssign for Angle<F> {
    fn mul_assign(&mut self, other: Angle<F>) {
        self.radiant *= other.radiant;
        self.mod_radiant();
    }
}

impl<F: Float + FloatConst + MulAssign> MulAssign<F> for Angle<F> {
    fn mul_assign(&mut self, other: F) {
        self.radiant *= other;
        self.mod_radiant();
    }
}

impl<F: Float + FloatConst> Mul for Angle<F> {
    type Output = Self;
    fn mul(self, other: Angle<F>) -> Self::Output {
        Self::new(self.radiant * other.radiant)
    }
}

impl<F: Float + FloatConst> Mul<F> for Angle<F> {
    type Output = Self;
    fn mul(self, other: F) -> Self::Output {
        Self::new(self.radiant * other)
    }
}

mod trait_f64 {

    use super::*;
    type F = f64;

    impl Mul<Angle<F>> for F {
        type Output = Angle<F>;
        fn mul(self, other: Angle<F>) -> Self::Output {
            other * self
        }
    }
}

mod trait_f32 {

    use super::*;
    type F = f32;

    impl Mul<Angle<F>> for F {
        type Output = Angle<F>;
        fn mul(self, other: Angle<F>) -> Self::Output {
            other * self
        }
    }
}

impl<F: Float + FloatConst> Zero for Angle<F> {
    fn zero() -> Self {
        Self::new(F::zero())
    }

    fn is_zero(&self) -> bool {
        self.radiant.is_zero()
    }
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn angle_mod_radiant() {
        let mut angle = Angle::new(0_f64);

        *angle.radiant_mut() = std::f64::consts::PI * 5_f64 / 2_f64;
        angle.mod_radiant();
        assert_abs_diff_eq!(*angle.radiant(), std::f64::consts::PI / 2_f64);

        *angle.radiant_mut() = std::f64::consts::PI.mul_add(6_f64, 1.5_f64);
        angle.mod_radiant();
        assert_abs_diff_eq!(*angle.radiant(), 1.5_f64);

        *angle.radiant_mut() = std::f64::consts::PI.mul_add(-2_f64, 0.6345_f64);
        angle.mod_radiant();
        assert_abs_diff_eq!(*angle.radiant(), 0.6345_f64);
    }
}

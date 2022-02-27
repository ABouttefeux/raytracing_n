use crate::utils::number::n_2;
use num_traits::{Float, FloatConst, Zero};
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Get the Euclidean reminder
fn rem_euclidean<F: Float>(num: F, div: F) -> F {
    let r = num % div;
    if r < F::zero() {
        r + div.abs()
    } else {
        r
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Angle<F: Float> {
    radiant: F,
}

impl<F: Float> Angle<F> {
    pub fn radiant(&self) -> &F {
        &self.radiant
    }

    fn radiant_mut(&mut self) -> &mut F {
        &mut self.radiant
    }

    pub fn degree(&self) -> F {
        self.radiant.to_degrees()
    }
}

impl<F: Float + FloatConst> Angle<F> {
    pub fn new(radiant: F) -> Self {
        Self {
            radiant: rem_euclidean(radiant, F::PI() * n_2()),
        }
    }

    pub fn new_degree(degree: F) -> Self {
        Self::new(degree.to_radians())
    }

    pub fn set_radiant(&mut self, radiant: F) {
        self.radiant = rem_euclidean(radiant, F::PI() * n_2());
    }

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

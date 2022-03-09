//use serde::{Deserialize, Serialize};

pub mod ray;

use std::iter::FusedIterator;
use std::mem::{forget, MaybeUninit};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{Float, FloatConst, Zero};
pub use ray::Ray;
use rayon::prelude::*;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::angle::Angle;
use crate::transformation::Transformable;
use crate::utils::number::n_2;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Vector<T, const N: usize> {
    #[cfg_attr(
        feature = "serde-serialize",
        serde(
            bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"),
            with = "serde_arrays"
        )
    )]
    data: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    pub const fn new(data: [T; N]) -> Self {
        Self { data }
    }

    pub const fn data(&self) -> &[T; N] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T; N] {
        &mut self.data
    }

    /// return true if and only if `N = 0`
    /// Example
    /// ```
    /// # use raytracing_n::vector::Vector;
    /// #
    /// let vec = Vector::<f64, 0>::new([]);
    /// assert!(vec.is_empty());
    ///
    /// let vec = Vector::new([1, 2, 3]);
    /// assert!(!vec.is_empty());
    ///
    /// let vec = Vector::new(['a', 'b', 'c']);
    /// assert!(!vec.is_empty());
    ///
    /// let vec = Vector::new(["v1", "banana", "energy"]);
    /// assert!(!vec.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + FusedIterator + ExactSizeIterator {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + FusedIterator + ExactSizeIterator {
        self.data.iter_mut()
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = <[T; N] as IntoIterator>::Item;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: Neg + Clone, const N: usize> Vector<T, N>
where
    <T as Neg>::Output: Default + Copy,
{
    pub fn old_neg(self) -> Vector<<T as Neg>::Output, N> {
        let mut vec = Vector::default();
        self.iter()
            .zip(vec.data.iter_mut())
            .for_each(|(el_1, el_2)| *el_2 = -el_1.clone());
        vec
    }
}

impl<T: Send + Sync, const N: usize> Vector<T, N> {
    pub fn par_iter(&self) -> impl ParallelIterator<Item = &T> + IndexedParallelIterator {
        self.data.par_iter()
    }

    pub fn par_iter_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut T> + IndexedParallelIterator {
        self.data.par_iter_mut()
    }

    pub fn into_par_iter(self) -> impl ParallelIterator<Item = T> + IndexedParallelIterator {
        self.data.into_par_iter()
    }
}

impl<T: Float + std::iter::Sum, const N: usize> Vector<T, N> {
    pub fn scalar_product(self, other: Self) -> T {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Float + Clone + std::iter::Sum,
{
    /// Gives the norm squared, i.e. the some of each elemqne squared
    pub fn norm_squared(&self) -> T {
        self.iter().map(|el| *el * *el).sum()
    }

    pub fn norm(&self) -> T {
        self.norm_squared().sqrt()
    }

    pub fn are_colinear(self, other: Self) -> bool {
        let norm_prod = self.norm() * other.norm();
        let scalar = self.scalar_product(other);
        let diff = (scalar.abs() - norm_prod).abs();

        diff < T::epsilon() * (scalar + norm_prod) / n_2() || !diff.is_normal()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Float + Clone + std::iter::Sum + FloatConst,
{
    pub fn angle_between(self, other: Self) -> Angle<T> {
        let norm_prod = self.norm() * other.norm();
        let scalar = self.scalar_product(other);
        Angle::new(T::acos(scalar / norm_prod))
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Float + Clone + std::iter::Sum + Sync + Send,
{
    pub fn par_norm_squared(&self) -> T {
        self.par_iter().map(|el| *el * *el).sum()
    }

    pub fn par_norm(&self) -> T {
        self.par_norm_squared().sqrt()
    }
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}

impl<T: AddAssign, const N: usize> Add for Vector<T, N> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl<T: AddAssign, const N: usize> AddAssign for Vector<T, N> {
    fn add_assign(&mut self, other: Self) {
        self.iter_mut()
            .zip(other.data.into_iter())
            .for_each(|(el_1, el_2)| *el_1 += el_2);
    }
}

impl<T: SubAssign, const N: usize> Sub for Vector<T, N> {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        self
    }
}

impl<T: SubAssign, const N: usize> SubAssign for Vector<T, N> {
    fn sub_assign(&mut self, other: Self) {
        self.iter_mut()
            .zip(other.data.into_iter())
            .for_each(|(el_1, el_2)| *el_1 -= el_2);
    }
}

// TODO review
impl<T: Neg, const N: usize> Neg for Vector<T, N> {
    type Output = Vector<<T as Neg>::Output, N>;

    fn neg(self) -> Self::Output {
        // copied for std lib
        let mut array = unsafe { MaybeUninit::<[MaybeUninit<_>; N]>::uninit().assume_init() };
        self.into_iter()
            .zip(array.iter_mut())
            .for_each(|(el_1, el_2)| {
                el_2.write(-el_1);
            });
        //Vector::new(unsafe { transmute::<_, [<T as Neg>::Output; N]>(array) })
        let ptr =
            &array as *const [MaybeUninit<<T as Neg>::Output>; N] as *const [<T as Neg>::Output; N];
        let result = Vector::new(unsafe { std::ptr::read(ptr) });
        forget(array);
        result
    }
}

impl<T: MulAssign + Clone, const N: usize> MulAssign<T> for Vector<T, N> {
    fn mul_assign(&mut self, other: T) {
        self.iter_mut().for_each(|el| *el *= other.clone());
    }
}

impl<T: MulAssign + Clone, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self; // TODO review
    fn mul(mut self, other: T) -> Self::Output {
        self *= other;
        self
    }
}

/*impl<T: MulAssign + Clone, const N: usize> Mul<Vector<T, N>> for T {
    type Output = Vector<T, N>;
    fn mul(self, other: Vector<T, N>) -> Self::Output {
        other * self
    }
}*/

// TODO review
mod mul_f64 {
    use super::*;
    type T = f64;
    impl<const N: usize> Mul<Vector<T, N>> for T {
        type Output = Vector<T, N>;
        fn mul(self, other: Vector<T, N>) -> Self::Output {
            other * self
        }
    }
}

mod mul_f32 {
    use super::*;
    type T = f32;
    impl<const N: usize> Mul<Vector<T, N>> for T {
        type Output = Vector<T, N>; // TODO review
        fn mul(self, other: Vector<T, N>) -> Self::Output {
            other * self
        }
    }
}

impl<T: DivAssign + Clone, const N: usize> DivAssign<T> for Vector<T, N> {
    fn div_assign(&mut self, other: T) {
        self.iter_mut().for_each(|el| *el /= other.clone());
    }
}

impl<T: DivAssign + Clone, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self; // TODO review
    fn div(mut self, other: T) -> Self::Output {
        self /= other;
        self
    }
}

impl<T: Zero + Copy + AddAssign, const N: usize> Zero for Vector<T, N> {
    fn zero() -> Self {
        Self {
            data: [T::zero(); N],
        }
    }

    fn is_zero(&self) -> bool {
        self.iter().all(|el| el.is_zero())
    }
}

impl<F: Float, const N: usize> Transformable<F, N> for Vector<F, N> {
    fn position(&self) -> &Vector<F, N> {
        self
    }
}

// TODO IntoParallelIterator IntoParallelRefIterator IntoParallelRefMutIterator

// TODO FromIterator IntoIterator

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn negative() {
        let v = Vector::new([1_i32, 2_i32, -3_i32]);
        assert_eq!(-v, Vector::new([-1_i32, -2_i32, 3_i32]));

        let v = Vector::new([1_f32, 2_f32, -3_f32]);
        assert_eq!(-v, Vector::new([-1_f32, -2_f32, 3_f32]));

        let v = Vector::new([0_i128, 1_231_i128, -3_342_432_i128]);
        assert_eq!(-v, Vector::new([0_i128, -1_231_i128, 3_342_432_i128]));
    }
}

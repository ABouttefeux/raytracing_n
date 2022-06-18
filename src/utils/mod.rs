//! utils function and structures

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::FpCategory;
use std::ops::Div;

use num_traits::{Float, Zero};
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::vector::Vector;

pub mod number;

/// A type that can never be (safely) initialized.
/// This is temporary, until
/// [`never`](https://doc.rust-lang.org/std/primitive.never.html)
/// is accepted into stable rust.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum Never {}

impl Display for Never {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Never {}

/// Error type return by set type functions.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum ErrorSet {
    /// Try to set a non valid value.
    NotValid,
}

impl Display for ErrorSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotValid => write!(f, "try to set a non valid value"),
        }
    }
}

impl Error for ErrorSet {}

/// A structure that guarantee that the stored that is positive, i.e `>0`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Positive<F: ?Sized + PartialOrd + Zero> {
    /// the data stored.
    data: F,
}

impl<F: ?Sized + PartialOrd + Zero> Positive<F> {
    /// test if the data is valid to be store inside the container.
    fn is_data_valid(data: &F) -> bool {
        data > &F::zero()
    }

    /// Create a new [`Positive`] with `data` if data is valid, otherwise return [`None`].
    pub fn new(data: F) -> Option<Self> {
        if Self::is_data_valid(&data) {
            Some(Self { data })
        } else {
            None
        }
    }

    /// Get the data stored.
    pub const fn data(&self) -> &F {
        &self.data
    }

    /// Try to set a new value inside the structure.
    ///
    /// # Errors
    /// Fails to set a new value and return [`ErrorSet::NotValid`] if `data <= 0`.
    ///
    /// # Examples
    /// ```
    /// # use raytracing_n::utils::{Positive, ErrorSet};
    /// #
    /// # fn main() -> Result<(), ErrorSet> {
    /// let mut a = Positive::new(1_i32).unwrap();
    /// a.set_data(2)?;
    /// assert_eq!(a.data(), &2);
    /// assert_eq!(a.set_data(0), Err(ErrorSet::NotValid));
    /// assert_eq!(a.set_data(-1), Err(ErrorSet::NotValid));
    /// # Ok(())
    /// # }
    pub fn set_data(&mut self, data: F) -> Result<(), ErrorSet> {
        if Self::is_data_valid(&data) {
            self.data = data;
            Ok(())
        } else {
            Err(ErrorSet::NotValid)
        }
    }
}

/// A structure that guarenteen that the stored that is not negative, i.e `>=0`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct NonNegative<F: ?Sized + PartialOrd + Zero> {
    /// the data stored.
    data: F,
}

impl<F: ?Sized + PartialOrd + Zero> NonNegative<F> {
    /// test if the data is valid to be store inside the container.
    fn is_data_valid(data: &F) -> bool {
        data >= &F::zero()
    }

    /// Create a new [`NonNegative`] with `data` if data is valid, otherwise return [`None`].
    pub fn new(data: F) -> Option<Self> {
        if Self::is_data_valid(&data) {
            Some(Self { data })
        } else {
            None
        }
    }

    /// Get the data stored.
    pub const fn data(&self) -> &F {
        &self.data
    }

    /// Try to set a new value inside the structure.
    ///
    /// # Errors
    /// Fails to set a new value and return [`ErrorSet::NotValid`] if `data < 0`.
    pub fn set_data(&mut self, data: F) -> Result<(), ErrorSet> {
        if Self::is_data_valid(&data) {
            self.data = data;
            Ok(())
        } else {
            Err(ErrorSet::NotValid)
        }
    }
}

/// A trait to get a norm a a element.
///
/// A norm p(x) is defined by the following properties:
/// 1. p(x + y) <= p(x) + p(y) for all x,y
/// 2. p(sx) = |s|p(x) for all x and scalar s
/// 3. p(x) = 0 then x = 0
pub trait Norm {
    /// The output type of the norm.
    type Output;

    /// Get the norm of the element.
    ///
    /// The norm should return a non negative number, (i,e `>= 0`). In the future
    /// I may change to [`NonNegative`].
    fn norm(&self) -> Self::Output;
}

impl<F: Float> Norm for F {
    type Output = F;

    fn norm(&self) -> Self::Output {
        self.abs()
    }
}

impl<F: Float + Clone + std::iter::Sum, const N: usize> Norm for Vector<F, N> {
    type Output = F; // TODO

    fn norm(&self) -> Self::Output {
        self.norm()
    }
}

/// A structure that guarenteen that the stored that is normed.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Normed<T>
where
    T: Norm + Div<<T as Norm>::Output, Output = T>,
{
    data: T,
}

impl<T: Norm + Div<<T as Norm>::Output, Output = T>> Normed<T> {
    /// Normalize the data and store it.
    ///
    /// # Panics
    /// panics if the norm is zero or if the division would panic.
    pub fn new_unchecked(data: T) -> Self {
        let norm = data.norm();
        Self { data: data / norm }
    }

    /// Get the data. It is guaranteed to be normalized.
    pub const fn data(&self) -> &T {
        &self.data
    }
}

impl<T> Normed<T>
where
    T: Norm + Div<<T as Norm>::Output, Output = T>,
    <T as Norm>::Output: Float,
{
    /// Try to normalize the data return [`None`] if the data is not [`FpCategory::Normal`].
    pub fn new(data: T) -> Option<Self> {
        let norm = data.norm();
        match norm.classify() {
            FpCategory::Normal => Some(Self { data: data / norm }),
            _ => None,
        }
    }
}

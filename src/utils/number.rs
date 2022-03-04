//! module containing generic number.

use num_traits::One;
use std::ops::Add;

pub fn n_2<O: One + Add<Output = O>>() -> O {
    O::one() + O::one()
}

pub fn n_4<O: One + Add<Output = O>>() -> O {
    n_2::<O>() * n_2::<O>()
}

pub fn n_8<O: One + Add<Output = O>>() -> O {
    n_4::<O>() * n_2::<O>()
}

pub fn n_16<O: One + Add<Output = O>>() -> O {
    n_8::<O>() * n_2::<O>()
}

pub fn n_32<O: One + Add<Output = O>>() -> O {
    n_16::<O>() * n_2::<O>()
}

pub fn n_64<O: One + Add<Output = O>>() -> O {
    n_32::<O>() * n_2::<O>()
}

pub fn n_128<O: One + Add<Output = O>>() -> O {
    n_64::<O>() * n_2::<O>()
}

/// Get 180.
pub fn n_180<O: One + Add<Output = O>>() -> O {
    n_4::<O>() + n_16::<O>() + n_32::<O>() + n_128::<O>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_180() {
        assert_eq!(n_180::<u32>(), 180_u32);
        assert!(n_180::<f64>() - 180_f64 <= 100_f64 * f64::EPSILON);
    }
}

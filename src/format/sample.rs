use litcontainers::{Scalar, ScalarType};
use std::mem::size_of;

pub trait Sample: Scalar {
	fn sample_type() -> ScalarType { Self::scalar_type() }

	fn sample_size() -> usize { size_of::<Self>() }
}

impl<T: Scalar> Sample for T {}
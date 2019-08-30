use litcontainers::{Scalar, ScalarType};

pub trait Sample: Scalar {
	fn sample_type() -> ScalarType { Self::scalar_type() }
}

impl<T: Scalar> Sample for T {}
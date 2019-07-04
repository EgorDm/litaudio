use litcontainers::Scalar;
use std::mem::size_of;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SampleType {
	Float,
	Double,
	UInt8,
	Int16,
	Int32,
	Int64,
}

pub trait Sample: Scalar
{
	fn get_sample_type() -> SampleType;

	fn get_sample_size() -> usize {
		size_of::<Self>()
	}

	fn from_usize(v: usize) -> Self { Self::from(v).unwrap() }
}

impl Sample for f32 {
	fn get_sample_type() -> SampleType {
		SampleType::Float
	}
}

impl Sample for f64 {
	fn get_sample_type() -> SampleType {
		SampleType::Double
	}
}

impl Sample for i32 {
	fn get_sample_type() -> SampleType {
		SampleType::Int32
	}
}

impl Sample for i64 {
	fn get_sample_type() -> SampleType { SampleType::Int64 }
}

impl Sample for i16 {
	fn get_sample_type() -> SampleType {
		SampleType::Int16
	}
}

impl Sample for u8 {
	fn get_sample_type() -> SampleType {
		SampleType::UInt8
	}
}
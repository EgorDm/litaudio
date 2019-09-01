use std::fmt::Debug;

pub trait SamplePackingType: Debug + Send + Sync {
	fn packing_type() -> SamplePacking;
}

#[derive(Debug)]
pub struct Interleaved;

impl SamplePackingType for Interleaved {
	fn packing_type() -> SamplePacking {
		SamplePacking::Interleaved
	}
}

#[derive(Debug)]
pub struct Deinterleaved;

impl SamplePackingType for Deinterleaved {
	fn packing_type() -> SamplePacking {
		SamplePacking::Deinterleaved
	}
}

pub enum SamplePacking {
	Interleaved,
	Deinterleaved,
}

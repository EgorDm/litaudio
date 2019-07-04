use litcontainers::{SizedStorage, Dim};
use crate::format::*;

pub trait SizedAudioStorage<T, C, L, P>: SizedStorage<C, L>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType
{
	#[inline]
	fn channel_dim(&self) -> C { self.row_dim() }

	#[inline]
	fn channel_count(&self) -> usize { self.channel_dim().value() }

	#[inline]
	fn sample_dim(&self) -> L { self.col_dim() }

	#[inline]
	fn sample_count(&self) -> usize { self.sample_dim().value() }

	#[inline]
	fn sample_packing(&self) -> SamplePacking { P::get_packing_type() }

	#[inline]
	fn sample_type(&self) -> SampleType { T::get_sample_type() }
}
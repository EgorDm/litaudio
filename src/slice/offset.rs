use litcontainers::{OffsetableRowSlice, OffsetableColSlice};
use crate::format::*;

pub trait OffsetableChannelSlice<T, C>: OffsetableRowSlice<T, C>
	where T: Sample, C: Dim
{
	#[inline]
	fn offset_channel(&mut self, v: usize) { self.offset_row(v) }

	#[inline]
	unsafe fn offset_channel_unchecked(&mut self, v: usize) { self.offset_row_unchecked(v) }
}

pub trait OffsetableSampleSlice<T, R>: OffsetableColSlice<T, R>
	where T: Sample, R: Dim
{
	#[inline]
	fn offset_sample(&mut self, v: usize) { self.offset_col(v) }

	#[inline]
	unsafe fn offset_sample_unchecked(&mut self, v: usize) { self.offset_col_unchecked(v) }
}

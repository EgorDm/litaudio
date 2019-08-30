use litcontainers::*;
use crate::format::*;
use crate::{OwnableAudio, AudioSliceable};

pub trait AudioStorage<T, P>: Storage<T> + OwnableAudio<T>
	where T: Sample, P: SamplePackingType
{
	#[inline]
	fn sample_rate(&self) -> i32;

	#[inline]
	fn as_channel_ptr(&self, p: usize) -> *const T { self.as_row_ptr(p) }

	#[inline]
	unsafe fn as_channel_ptr_unchecked(&self, p: usize) -> *const T { self.as_col_ptr_unchecked(p) }

	#[inline]
	fn as_sample_ptr(&self, p: usize) -> *const T { self.as_col_ptr(p) }

	#[inline]
	unsafe fn as_sample_ptr_unchecked(&self, p: usize) -> *const T { self.as_col_ptr_unchecked(p) }

	// Iterators
	fn as_channel_iter(&self) -> FullAxisIter<T, Self, RowAxis> { self.as_row_iter() }

	fn as_channel_slice_iter(&self) -> RowSliceIter<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { self.as_row_slice_iter() }

	fn as_channel_range_iter<R: SliceRange>(&self, range: R) -> FullIter<T, R::Size, Self::RowStride, Self::ColStride> { self.as_row_range_iter(range) }

	fn as_sample_iter(&self) -> FullAxisIter<T, Self, ColAxis> { self.as_col_iter() }

	fn as_sample_slice_iter(&self) -> ColSliceIter<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { self.as_col_slice_iter() }

	fn as_sample_range_iter<R: SliceRange>(&self, range: R) -> FullIter<T, R::Size, Self::ColStride, Self::RowStride> { self.as_col_range_iter(range) }
}


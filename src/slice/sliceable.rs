use litcontainers::slice::*;
use crate::format::*;

pub trait AudioSliceable<T: Sample>: Sliceable<T> {
	#[inline]
	fn slice_channels<R: SliceRange>(&self, range: R) -> Slice<T, R::Size, Self::RowStride, Self::Cols, Self::ColStride>
	{ self.slice_rows(range) }

	#[inline]
	fn slice_samples<R: SliceRange>(&self, range: R) -> Slice<T, Self::Rows, Self::RowStride, R::Size, Self::ColStride>
	{ self.slice_cols(range) }
}

pub trait AudioSliceableMut<T: Sample>: SliceableMut<T> {
	#[inline]
	fn slice_channels_mut<R: SliceRange>(&mut self, range: R) -> SliceMut<T, R::Size, Self::RowStride, Self::Cols, Self::ColStride>
	{ self.slice_rows_mut(range) }


	#[inline]
	fn slice_samples_mut<R: SliceRange>(&mut self, range: R) -> SliceMut<T, Self::Rows, Self::RowStride, R::Size, Self::ColStride>
	{ self.slice_cols_mut(range) }
}

impl<T: Sample, S: Sliceable<T>> AudioSliceable<T> for S {}

impl<T: Sample, S: SliceableMut<T>> AudioSliceableMut<T> for S {}

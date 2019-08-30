use litcontainers::*;
use crate::format::*;
use crate::{AudioStorage, AudioSliceableMut};


pub trait AudioStorageMut<T, P>: AudioStorage<T, P> + StorageMut<T>
	where T: Sample, P: SamplePackingType
{
	fn set_sample_rate(&mut self, sample_rate: i32);

	#[inline]
	fn as_channel_ptr_mut(&mut self, p: usize) -> *mut T { self.as_row_ptr_mut(p) }

	#[inline]
	unsafe fn as_channel_ptr_mut_unchecked(&mut self, p: usize) -> *mut T { self.as_row_ptr_mut_unchecked(p) }

	#[inline]
	fn as_sample_ptr_mut(&mut self, p: usize) -> *mut T { self.as_col_ptr_mut(p)}

	#[inline]
	unsafe fn as_sample_ptr_mut_unchecked(&mut self, p: usize) -> *mut T { self.as_col_ptr_mut_unchecked(p) }

	// Iterator
	fn as_channel_iter_mut(&mut self) -> FullAxisIterMut<T, Self, RowAxis> { self.as_row_iter_mut() }

	fn as_channel_slice_iter_mut(&mut self) -> RowSliceIterMut<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { self.as_row_slice_iter_mut() }

	fn as_channel_range_iter_mut<R: SliceRange>(&mut self, range: R) -> FullIterMut<T, R::Size, Self::RowStride, Self::ColStride>
	{
		self.as_row_range_iter_mut(range)
	}

	fn as_sample_iter_mut(&mut self) -> FullAxisIterMut<T, Self, ColAxis> { self.as_col_iter_mut() }

	fn as_sample_slice_iter_mut(&mut self) -> ColSliceIterMut<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { self.as_col_slice_iter_mut() }

	fn as_sample_range_iter_mut<R: SliceRange>(&mut self, range: R) -> FullIterMut<T, R::Size, Self::ColStride, Self::RowStride>
	{
		self.as_col_range_iter_mut(range)
	}
}

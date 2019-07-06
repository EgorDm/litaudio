use litcontainers::{SliceRange, StorageMut};
use crate::format::*;
use crate::iterator::*;
use crate::storage::{SizedAudioStorage, AudioStorage, AudioPtrMutStorage};
use crate::slice::AudioSliceMut;


pub trait AudioStorageMut<T, C, L, P>: SizedAudioStorage<T, C, L, P> + AudioStorage<T, C, L, P> + StorageMut<T, C, L>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType
{
	fn set_sample_rate(&mut self, sample_rate: i32);

	#[inline]
	fn as_mut_ptr(&mut self) -> *const T { unsafe { self.get_index_mut_ptr_unchecked(0) } }

	// Row Contigious Access Functions
	#[inline]
	fn as_channel_mut_slice<'b, 'a: 'b>(&'a mut self, v: usize) -> &'b mut [T] { self.as_row_mut_slice(v) }

	#[inline]
	fn as_channel_mut_ptr(&mut self, v: usize) -> *mut T { self.as_row_mut_ptr(v) }

	#[inline]
	unsafe fn as_channel_mut_ptr_unchecked(&mut self, v: usize) -> *mut T { self.as_row_mut_ptr_unchecked(v) }

	// Col Contigious Access Functions
	#[inline]
	fn as_sample_mut_slice<'b, 'a: 'b>(&'a mut self, v: usize) -> &'b mut [T] { self.as_col_mut_slice(v) }

	#[inline]
	fn as_sample_mut_ptr(&mut self, v: usize) -> *mut T { self.as_col_mut_ptr(v) }

	#[inline]
	unsafe fn as_sample_mut_ptr_unchecked(&mut self, v: usize) -> *mut T { self.as_col_mut_ptr_unchecked(v) }

	// Iterator
	fn as_channel_mut_iter<'a: 'b, 'b>(&'a mut self) -> ChannelIterMutPtr<'b, T, C, L, P, Self> {
		ChannelIterMutPtr::new(self)
	}

	fn as_channel_slice_mut_iter<'a: 'b, 'b, RR: SliceRange<C>>(&'a mut self, range: RR) -> ChannelIterMutPtr<'b, T, C, L, P, Self> {
		ChannelIterMutPtr::from_range(self, range.begin(), range.end())
	}

	fn as_sample_mut_iter<'a: 'b, 'b>(&'a mut self) -> SampleIterMutPtr<'b, T, C, L, P, Self> {
		SampleIterMutPtr::new(self)
	}

	fn as_sample_slice_mut_iter<'a: 'b, 'b, CR: SliceRange<L>>(&'a mut self, range: CR) -> SampleIterMutPtr<'b, T, C, L, P, Self> {
		SampleIterMutPtr::from_range(self, range.begin(), range.end())
	}

	// Slice
	#[inline]
	fn slice_channels_mut<'b: 'c, 'c, CR: SliceRange<C>>(&'b mut self, range: CR) -> AudioSliceMut<'c, T, CR::Size, Self::RStride, L, Self::CStride, P> {
		assert!(range.end() <= self.row_count(), "Slice is out of bounds!");
		AudioSliceMut::new(unsafe {
			AudioPtrMutStorage::new(
				self.as_channel_mut_ptr(range.begin()),
				range.size(),
				self.sample_dim(),
				self.channel_stride_dim(),
				self.sample_stride_dim(),
			)
		}, self.sample_rate())
	}

	#[inline]
	fn slice_samples_mut<'b: 'c, 'c, LR: SliceRange<L>>(&'b mut self, range: LR) -> AudioSliceMut<'c, T, C, Self::RStride, LR::Size, Self::CStride, P> {
		assert!(range.end() <= self.col_count(), "Slice is out of bounds!");
		AudioSliceMut::new(unsafe {
			AudioPtrMutStorage::new(
				self.as_sample_mut_ptr(range.begin()),
				self.channel_dim(),
				range.size(),
				self.channel_stride_dim(),
				self.sample_stride_dim(),
			)
		}, self.sample_rate())
	}
}
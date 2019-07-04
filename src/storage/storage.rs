use litcontainers::{Storage, SliceRange};
use crate::format::*;
use crate::iterator::*;
use crate::storage::{SizedAudioStorage, OwnableAudio, AudioPtrStorage};
use crate::slice::AudioSlice;

pub trait AudioStorage<T, C, L, P>: SizedAudioStorage<T, C, L, P> + Storage<T, C, L> + OwnableAudio<T, C, L>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType
{
	#[inline]
	fn channel_stride_dim(&self) -> Self::RStride { self.row_stride_dim() }

	#[inline]
	fn channel_stride(&self) -> usize { self.channel_stride_dim().value() }

	#[inline]
	fn channel_index(&self, p: usize) -> usize { p * self.channel_stride() }

	#[inline]
	fn sample_stride_dim(&self) -> Self::CStride { self.col_stride_dim() }

	#[inline]
	fn sample_stride(&self) -> usize { self.sample_stride_dim().value() }

	#[inline]
	fn sample_index(&self, p: usize) -> usize { p * self.sample_stride() }

	#[inline]
	fn sample_rate(&self) -> i32;

	// Channel Contigious Access Functions
	#[inline]
	fn channel_index_span(&self, channel: usize) -> usize { self.row_index_span(channel) }

	#[inline]
	fn as_channel_slice<'b, 'a: 'b>(&'a self, v: usize) -> &'b [T] { self.as_row_slice(v) }

	#[inline]
	fn as_channel_ptr(&self, v: usize) -> *const T { self.as_row_ptr(v) }

	#[inline]
	unsafe fn as_channel_ptr_unchecked(&self, v: usize) -> *const T { self.as_row_ptr_unchecked(v) }

	// Sample Contigious Access Functions
	#[inline]
	fn sample_index_span(&self, sample: usize) -> usize { self.col_index_span(sample) }

	#[inline]
	fn as_sample_slice<'b, 'a: 'b>(&'a self, v: usize) -> &'b [T] { self.as_col_slice(v) }

	#[inline]
	fn as_sample_ptr(&self, v: usize) -> *const T { self.as_col_ptr(v) }

	#[inline]
	unsafe fn as_sample_ptr_unchecked(&self, v: usize) -> *const T { self.as_col_ptr_unchecked(v) }

	// Iter
	fn as_channel_iter<'a: 'b, 'b>(&'a self) -> ChannelIterPtr<'b, T, C, L, P, Self> { ChannelIterPtr::new(self) }

	fn as_channel_slice_iter<'a: 'b, 'b, CR: SliceRange<C>>(&'a self, range: CR) -> ChannelIterPtr<'b, T, C, L, P, Self> {
		ChannelIterPtr::from_range(self, range.begin(), range.end())
	}

	fn as_sample_iter<'a: 'b, 'b>(&'a self) -> SampleIterPtr<'b, T, C, L, P, Self> { SampleIterPtr::new(self) }

	fn as_sample_slice_iter<'a: 'b, 'b, LR: SliceRange<L>>(&'a self, range: LR) -> SampleIterPtr<'b, T, C, L, P, Self> {
		SampleIterPtr::from_range(self, range.begin(), range.end())
	}

	// Slice
	#[inline]
	fn slice_channels<'b: 'c, 'c, CR: SliceRange<C>>(&'b self, range: CR) -> AudioSlice<'c, T, CR::Size, Self::RStride, L, Self::CStride, P> {
		assert!(range.end() <= self.row_count(), "Slice is out of bounds!");
		AudioSlice::new(unsafe {
			AudioPtrStorage::new(
				self.as_channel_ptr(range.begin()),
				range.size(),
				self.sample_dim(),
				self.channel_stride_dim(),
				self.sample_stride_dim(),
			)
		})
	}

	#[inline]
	fn slice_sample<'b: 'c, 'c, LR: SliceRange<L>>(&'b self, range: LR) -> AudioSlice<'c, T, C, Self::RStride, LR::Size, Self::CStride, P> {
		assert!(range.end() <= self.col_count(), "Slice is out of bounds!");
		AudioSlice::new(unsafe {
			AudioPtrStorage::new(
				self.as_sample_ptr(range.begin()),
				self.channel_dim(),
				range.size(),
				self.channel_stride_dim(),
				self.sample_stride_dim(),
			)
		})
	}
}

use litcontainers::*;
use crate::format::*;
use crate::storage::*;
use crate::slice::offset::*;
use std::marker::PhantomData;
use std::ops::{IndexMut, Index};


macro_rules! ptr_storage (
	($Name: ident, $Ptr: ty, $Storage: ty) => {
		#[repr(C)]
		#[dimension_fields(T, C, L)]
		#[derive(Eq, Debug, Clone, PartialEq, SizedStorage, Storage)]
		pub struct $Name<'a, T, C, CS, L, LS, P>
			where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
		{
			storage: $Storage,
			_phantoms: PhantomData<(&'a (), P)>
		}

		impl<'a, T, C, CS, L, LS, P> $Name<'a, T, C, CS, L, LS, P>
			where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
		{
			pub unsafe fn new(data: $Ptr, channel_dim: C, sample_dim: L, channel_stride: CS, sample_stride: LS) -> Self {
				Self {
					storage: <$Storage>::new(data, channel_dim, sample_dim, channel_stride, sample_stride),
					_phantoms: PhantomData
				}
			}
		}

		impl<'a, T, CS, L, LS, P> OffsetableRowSlice<T, L> for $Name<'a, T, Dynamic, CS, L, LS, P>
			where T: Sample, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
		{
			#[inline]
			unsafe fn offset_row_unchecked(&mut self, v: usize) {
				self.storage.offset_row_unchecked(v);
			}
		}

		impl<'a, T, C, CS, LS, P> OffsetableColSlice<T, C> for $Name<'a, T, C, CS, Dynamic, LS, P>
			where T: Sample, C: Dim, CS: Dim, LS: Dim, P: SamplePackingType
		{
			#[inline]
			unsafe fn offset_col_unchecked(&mut self, v: usize) {
				self.storage.offset_col_unchecked(v);
			}
		}

		impl<'a, T, C, CS, L, LS, P> Ownable<T, C, L> for $Name<'a, T, C, CS, L, LS, P>
			where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
		{
			type OwnedType = DeinterleavedStorage<T, C, L>;

			fn owned(self) -> Self::OwnedType { self.clone_owned() }

			fn clone_owned(&self) -> Self::OwnedType {
				let data = self.as_row_iter().cloned().collect();
				Self::OwnedType::from_data(self.row_dim(), self.col_dim(), data)
			}
		}

		impl<'a, T, C, CS, L, LS, P> SizedAudioStorage<T, C, L, P> for $Name<'a, T, C, CS, L, LS, P>
			where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
		{}

		impl<'a, T, C, CS, L, LS, P> AudioStorage<T, C, L, P> for $Name<'a, T, C, CS, L, LS, P>
			where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
		{
			fn sample_rate(&self) -> i32 { 1 }
		}

		impl<'a, T, C, CS, L, LS, P> OwnableAudio<T, C, L> for $Name<'a, T, C, CS, L, LS, P>
			where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
		{
			type OwnedPackingType = DeinterleavedPacking;
			type OwnedAudioType = DeinterleavedStorage<T, C, L>;

			fn owned_audio(self) -> Self::OwnedAudioType { self.clone_owned_audio() }

			fn clone_owned_audio(&self) -> Self::OwnedAudioType {
				let data = self.as_row_iter().cloned().collect();
				Self::OwnedAudioType::from_data(self.row_dim(), self.col_dim(), data)
			}
		}

		impl<'a, T, CS, L, LS, P> OffsetableChannelSlice<T, L> for $Name<'a, T, Dynamic, CS, L, LS, P>
			where T: Sample, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
		{}

		impl<'a, T, C, CS, LS, P> OffsetableSampleSlice<T, C> for $Name<'a, T, C, CS, Dynamic, LS, P>
			where T: Sample, C: Dim, CS: Dim, LS: Dim, P: SamplePackingType
		{}

		impl<'a, T, C, CS, L, LS, P> Index<usize> for $Name<'a, T, C, CS, L, LS, P>
			where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
		{
			type Output = T;

			fn index(&self, index: usize) -> &Self::Output {
				assert!(index < self.size());
				unsafe { &*self.get_index_ptr_unchecked(index) }
			}
		}
	}
);

ptr_storage!(AudioPtrStorage, *const T, PtrStorage<'a, T, C, CS, L, LS>);
ptr_storage!(AudioPtrMutStorage, *mut T, PtrMutStorage<'a, T, C, CS, L, LS>);

impl<'a, T, C, CS, L, LS, P> StorageMut<T, C, L> for AudioPtrMutStorage<'a, T, C, CS, L, LS, P>
	where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T {
		self.storage.get_index_mut_ptr_unchecked(i)
	}
}

impl<'a, T, C, CS, L, LS, P> AudioStorageMut<T, C, L, P> for AudioPtrMutStorage<'a, T, C, CS, L, LS, P>
	where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
{
	fn set_sample_rate(&mut self, _sample_rate: i32) { }
}

impl<'a, T, CS, L, LS, P> AudioPtrStorage<'a, T, Dynamic, CS, L, LS, P>
	where T: Sample, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
{
	#[inline]
	pub fn offset_channel(&mut self, v: usize) { self.storage.offset_row(v) }

	#[inline]
	pub unsafe fn offset_channel_unchecked(&mut self, v: usize) { self.storage.offset_row_unchecked(v) }
}

impl<'a, T, C, CS, LS, P> AudioPtrStorage<'a, T, C, CS, Dynamic, LS, P>
	where T: Sample, C: Dim, CS: Dim, LS: Dim, P: SamplePackingType
{
	#[inline]
	pub fn offset_sample(&mut self, v: usize) { self.storage.offset_col(v) }

	#[inline]
	pub unsafe fn offset_sample_unchecked(&mut self, v: usize) { self.storage.offset_col_unchecked(v) }
}

impl<'a, T, C, CS, LS, P> AudioPtrStorage<'a, T, C, CS, Dynamic, LS, P>
	where T: Sample, C: Dim, CS: Dim, LS: Dim, P: SamplePackingType
{
	pub fn shift_sample_to<S, CO, LO>(&mut self, storage: &S, sample_offset: usize, sample_count: usize)
		where CO: Dim, LO: Dim,
			S: Storage<T, CO, LO, RStride=<Self as Storage<T, C, Dynamic>>::RStride, CStride=<Self as Storage<T, C, Dynamic>>::CStride>
	{
		self.storage.shift_col_to(storage, sample_offset, sample_count)
	}
}

impl<'a, T, C, CS, LS, P> AudioPtrMutStorage<'a, T, C, CS, Dynamic, LS, P>
	where T: Sample, C: Dim, CS: Dim, LS: Dim, P: SamplePackingType
{
	pub fn shift_sample_to<S, CO, LO>(&mut self, storage: &mut S, sample_offset: usize, sample_count: usize)
		where CO: Dim, LO: Dim, S: Storage<T, CO, LO>
			+ StorageMut<T, CO, LO, RStride=<Self as Storage<T, C, Dynamic>>::RStride, CStride=<Self as Storage<T, C, Dynamic>>::CStride>
	{
		self.storage.shift_col_to(storage, sample_offset, sample_count)
	}
}

impl<'a, T, C, CS, L, LS, P> IndexMut<usize> for AudioPtrMutStorage<'a, T, C, CS, L, LS, P>
	where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert!(index < self.size());
		unsafe { &mut *self.get_index_mut_ptr_unchecked(index) }
	}
}

use litcontainers::*;
use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;


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
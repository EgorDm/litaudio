use litcontainers::storage::*;
use litcontainers::ops::*;
use litcontainers::{Container, OffsetableRowSlice, OffsetableColSlice};
use std::marker::PhantomData;
use crate::format::*;
use crate::storage::*;
use crate::container::AudioContainer;
use super::offset::*;
use std::fmt::{Display, Formatter, Error};
use std::ops::{Index, IndexMut};

pub type AudioSlice<'a, T, C, CS, L, LS, P> = AudioSliceBase<'a, T, C, L, P, AudioPtrStorage<'a, T, C, CS, L, LS, P>>;
pub type AudioSliceMut<'a, T, C, CS, L, LS, P> = AudioSliceBase<'a, T, C, L, P, AudioPtrMutStorage<'a, T, C, CS, L, LS, P>>;

#[repr(C)]
#[dimension_fields(T, C, L)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, SizedStorage, Storage)]
pub struct AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	storage: S,
	sample_rate: i32,
	_phantoms: PhantomData<(&'a (), T, C, L, P)>
}

impl<'a, T, C, L, P, S> AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	pub fn new(storage: S, sample_rate: i32) -> Self {
		AudioSliceBase { storage, sample_rate, _phantoms: PhantomData }
	}
}

// Litcontainers implementations
impl<'a, T, C, L, P, S> Ownable<T, C, L> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>,
{
	type OwnedType = Container<T, C, L, S::OwnedType>;

	fn owned(self) -> Self::OwnedType {
		Self::OwnedType::new(self.storage.owned())
	}

	fn clone_owned(&self) -> Self::OwnedType {
		Self::OwnedType::new(self.storage.clone_owned())
	}
}

impl<'a, T, C, L, P, S> StorageMut<T, C, L> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T { self.storage.get_index_mut_ptr_unchecked(i) }

	fn map_inplace<F: FnMut(&mut T)>(&mut self, f: F) { self.storage.map_inplace(f) }
}

impl<'a, T, L, P, S> OffsetableRowSlice<T, L> for AudioSliceBase<'a, T, Dynamic, L, P, S>
	where T: Sample, L: Dim, P: SamplePackingType, S: AudioStorage<T, Dynamic, L, P> + OffsetableRowSlice<T, L>
{
	#[inline]
	unsafe fn offset_row_unchecked(&mut self, v: usize) {
		self.storage.offset_row_unchecked(v);
	}
}

impl<'a, T, C, P, S> OffsetableColSlice<T, C> for AudioSliceBase<'a, T, C, Dynamic, P, S>
	where T: Sample, C: Dim, P: SamplePackingType, S: AudioStorage<T, C, Dynamic, P> + OffsetableColSlice<T, C>
{
	#[inline]
	unsafe fn offset_col_unchecked(&mut self, v: usize) {
		self.storage.offset_col_unchecked(v);
	}
}

// Litaudio implementations
impl<'a, T, C, L, P, S> SizedAudioStorage<T, C, L, P> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{}

impl<'a, T, C, L, P, S> AudioStorage<T, C, L, P> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	fn sample_rate(&self) -> i32 { self.sample_rate }
}

impl<'a, T, C, L, P, S> AudioStorageMut<T, C, L, P> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	fn set_sample_rate(&mut self, sample_rate: i32) { self.sample_rate = sample_rate; }
}

impl<'a, T, C, L, P, S> OwnableAudio<T, C, L> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	type OwnedPackingType = S::OwnedPackingType;
	type OwnedAudioType = AudioContainer<T, C, L, Self::OwnedPackingType, S::OwnedAudioType>;

	fn owned_audio(self) -> Self::OwnedAudioType {
		let sr = self.sample_rate();
		Self::OwnedAudioType::new(self.storage.owned_audio(), sr)
	}

	fn clone_owned_audio(&self) -> Self::OwnedAudioType {
		let sr = self.sample_rate();
		Self::OwnedAudioType::new(self.storage.clone_owned_audio(), sr)
	}
}

impl<'a, T, L, P, S> OffsetableChannelSlice<T, L> for AudioSliceBase<'a, T, Dynamic, L, P, S>
	where T: Sample, L: Dim, P: SamplePackingType, S: AudioStorage<T, Dynamic, L, P> + OffsetableRowSlice<T, L>
{}

impl<'a, T, C, P, S> OffsetableSampleSlice<T, C> for AudioSliceBase<'a, T, C, Dynamic, P, S>
	where T: Sample, C: Dim, P: SamplePackingType, S: AudioStorage<T, C, Dynamic, P> + OffsetableColSlice<T, C>
{}

impl<'a, T, C, CS, LS, P> AudioSlice<'a, T, C, CS, Dynamic, LS, P>
	where T: Sample, C: Dim, CS: Dim, LS: Dim, P: SamplePackingType
{
	pub fn shift_sample_to<S, CO, LO>(&mut self, storage: &S, sample_offset: usize, sample_count: usize)
		where CO: Dim, LO: Dim,
		      S: Storage<T, CO, LO, RStride=<Self as Storage<T, C, Dynamic>>::RStride, CStride=<Self as Storage<T, C, Dynamic>>::CStride>
	{
		self.storage.shift_sample_to(storage, sample_offset, sample_count)
	}
}

impl<'a, T, C, CS, LS, P> AudioSliceMut<'a, T, C, CS, Dynamic, LS, P>
	where T: Sample, C: Dim, CS: Dim, LS: Dim, P: SamplePackingType
{
	pub fn shift_sample_to<S, CO, LO>(&mut self, storage: &mut S, sample_offset: usize, sample_count: usize)
		where CO: Dim, LO: Dim, S: Storage<T, CO, LO>
		+ StorageMut<T, CO, LO, RStride=<Self as Storage<T, C, Dynamic>>::RStride, CStride=<Self as Storage<T, C, Dynamic>>::CStride>
	{
		self.storage.shift_sample_to(storage, sample_offset, sample_count)
	}
}

impl<'a, T, C, L, P, S> Display for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", Fmt(|f| print_storage(self, f)))
	}
}

impl<'a, T, C, L, P, S> Sum for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	type Output = T;

	fn sum(&self) -> Self::Output {
		let mut ret = T::default();
		for v in self.as_row_iter() { ret += *v }
		ret
	}
}

impl<'a, T, C, L, P, S> Mean for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	type Output = T;

	fn mean(&self) -> Self::Output {
		self.sum() / num_traits::cast(self.size()).unwrap()
	}
}

impl<'a, T, C, L, P, S> Index<usize> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		assert!(index < self.size());
		unsafe { &*self.get_index_ptr_unchecked(index) }
	}
}

impl<'a, T, C, L, P, S> IndexMut<usize> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert!(index < self.size());
		unsafe { &mut *self.get_index_mut_ptr_unchecked(index) }
	}
}
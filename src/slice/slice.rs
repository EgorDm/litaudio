use crate::format::*;
use crate::storage::*;
use litcontainers::storage::*;
use litcontainers::Container;
use std::marker::PhantomData;
use crate::container::AudioContainer;

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
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T { self. storage.get_index_mut_ptr_unchecked(i) }
}

// Litaudio implementations
impl<'a, T, C, L, P, S> SizedAudioStorage<T, C, L, P> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{}

impl<'a, T, C, L, P, S> AudioStorage<T, C, L, P> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	fn sample_rate(&self) -> i32 { 1 }
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
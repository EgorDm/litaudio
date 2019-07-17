use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use litcontainers::format::*;
use litcontainers::storage::*;
use litcontainers::Container;
use std::fmt::{Display, Formatter, Error};
use std::ops::{Index, IndexMut};

pub type AudioDeinterleaved<T, C, L> = AudioContainer<T, C, L, DeinterleavedPacking, DeinterleavedStorage<T, C, L>>;
pub type AudioInterleaved<T, C, L> = AudioContainer<T, C, L, InterleavedPacking, InterleavedStorage<T, C, L>>;

#[repr(C)]
#[dimension_fields(T, C, L)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, SizedStorage, Storage, StorageMut)]
pub struct AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	pub(crate) storage: S,
	sample_rate: i32,
	_phantoms: PhantomData<(T, C, L, P)>
}

impl<T, C, L, P, S> AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	pub fn new(storage: S, sample_rate: i32) -> Self { Self {storage, sample_rate, _phantoms: PhantomData} }
}

// Litcontainers implementations
impl<T, C, L, P, S> Ownable<T, C, L> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
{
	type OwnedType = Container<T, C, L, S::OwnedType>;

	fn owned(self) -> Self::OwnedType {
		Self::OwnedType::new(self.storage.owned())
	}

	fn clone_owned(&self) -> Self::OwnedType {
		Self::OwnedType::new(self.storage.clone_owned())
	}
}

impl<T, C, L, P, S> StorageConstructor<T, C, L> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P> + StorageConstructor<T, C, L>
{
	fn from_value(rows: C, cols: L, value: T) -> Self {
		AudioContainer::new(S::from_value(rows, cols, value), 1)
	}
}

impl<T, L, P, S> DynamicRowStorage<T, L> for AudioContainer<T, Dynamic, L, P, S>
	where T: Sample, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, Dynamic, L, P> + DynamicRowStorage<T, L>
{
	fn set_row_count(&mut self, count: usize) {
		self.storage.set_row_count(count)
	}
}

impl<T, C, P, S> DynamicColStorage<T, C> for AudioContainer<T, C, Dynamic, P, S>
	where T: Sample, C: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, Dynamic, P> + DynamicColStorage<T, C>
{
	fn set_col_count(&mut self, count: usize) {
		self.storage.set_col_count(count)
	}
}

// Litaudio implementations
impl<T, C, L, P, S> SizedAudioStorage<T, C, L, P> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{}

impl<T, C, L, P, S> AudioStorage<T, C, L, P> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	fn sample_rate(&self) -> i32 { self.sample_rate }
}

impl<T, C, L, P, S> AudioStorageMut<T, C, L, P> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	fn set_sample_rate(&mut self, sample_rate: i32) { self.sample_rate = sample_rate; }
}

impl<T, C, L, P, S> OwnableAudio<T, C, L> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
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

impl<T, L, P, S> DynamicChannelStorage<T, L> for AudioContainer<T, Dynamic, L, P, S>
	where T: Sample, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, Dynamic, L, P> + DynamicRowStorage<T, L>
{}

impl<T, C, P, S> DynamicSampleStorage<T, C> for AudioContainer<T, C, Dynamic, P, S>
	where T: Sample, C: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, Dynamic, P> + DynamicColStorage<T, C>
{}

impl<T, C, L, P, S> Display for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", Fmt(|f| print_storage(self, f)))
	}
}

impl<T, C, L, P, S> Index<usize> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		assert!(index < self.size());
		unsafe { &*self.get_index_ptr_unchecked(index) }
	}
}

impl<T, C, L, P, S> IndexMut<usize> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert!(index < self.size());
		unsafe { &mut *self.get_index_mut_ptr_unchecked(index) }
	}
}



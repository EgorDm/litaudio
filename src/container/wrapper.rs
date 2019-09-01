use litcontainers::*;
use crate::format::*;
use std::marker::PhantomData;
use std::fmt;
use std::ops::Index;
use crate::{AudioStorage, AudioStorageMut, OwnableAudio};

pub type DeinterleavedStorage<T, C, L> = VecStorageRM<T, C, L>;
pub type InterleavedStorage<T, C, L> = VecStorageCM<T, C, L>;
pub type AudioDeinterleaved<T, C, L> = AudioContainer<T, DeinterleavedPacking, VecStorageRM<T, C, L>>;
pub type AudioInterleaved<T, C, L> = AudioContainer<T, InterleavedPacking, VecStorageCM<T, C, L>>;
pub type AudioDeinterleavedC<T, C, L> = Container<T, AudioContainer<T, DeinterleavedPacking, VecStorageRM<T, C, L>>>;
pub type AudioInterleavedC<T, C, L> = Container<T, AudioContainer<T, InterleavedPacking, VecStorageCM<T, C, L>>>;

// Container storing scalar values. Wraps around given storage.
#[derive(Debug, Storage, StorageSize, Strided, Ownable, new)]
pub struct AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: Storage<T>
{
	storage: S,
	sample_rate: i32,
	_phantoms: PhantomData<(T, P)>,
}

impl<T, P, S> OwnableAudio<T> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: Storage<T>
{
	type OwnedPackingType = P;
	type OwnedAudioType = AudioContainer<T, P, Container<T, S::OwnedType>>;

	fn owned_audio(self) -> Container<T, Self::OwnedAudioType> {
		AudioContainer::new(self.storage.owned(), self.sample_rate).into()
	}

	fn clone_owned_audio(&self) -> Container<T, Self::OwnedAudioType> {
		AudioContainer::new(self.storage.clone_owned(), self.sample_rate).into()
	}
}

impl<T, P, S> AudioStorage<T, P> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: Storage<T>
{
	fn sample_rate(&self) -> i32 { self.sample_rate }
}

impl<T, P, S> AudioStorageMut<T, P> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: StorageMut<T>
{
	fn set_sample_rate(&mut self, sample_rate: i32) { self.sample_rate = sample_rate }
}

impl<T, P, S> From<S> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: AudioStorage<T, P>
{
	fn from(s: S) -> Self {
		let sr = s.sample_rate();
		AudioContainer::new(s, sr)
	}
}

// Litcontainers
impl<T, P, S> StorageMut<T> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: StorageMut<T>
{
	fn as_ptr_mut(&mut self) -> *mut T { self.storage.as_ptr_mut() }
}

impl<T, P, S> StorageConstructor<T> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: Storage<T> + StorageConstructor<T>
{
	fn from_value(s: Size<Self::Rows, Self::Cols>, value: T) -> Self { Self::new(S::from_value(s, value), 0)  }
}

impl<T, P, S> DynamicRowStorage<T> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: StorageMut<T> + DynamicRowStorage<T>
{
	fn set_rows(&mut self, count: usize) {
		self.storage.set_rows(count)
	}
}

impl<T, P, S> DynamicColStorage<T> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: StorageMut<T> + DynamicColStorage<T>
{
	fn set_cols(&mut self, count: usize) { self.storage.set_cols(count) }
}

impl<T, P, S> fmt::Display for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: Storage<T>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", Fmt(|f| print_storage(self, f)))
	}
}

impl<T, P, S> InplaceMap<T> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: StorageMut<T> + InplaceMap<T>
{
	fn map_inplace<F: FnMut(&mut T)>(&mut self, f: F) { self.storage.map_inplace(f) }
}

impl<T, P, S> InplaceMapOrdered<T> for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: StorageMut<T> + InplaceMapOrdered<T>
{
	fn map_inplace_ordered<F: FnMut(&mut T)>(&mut self, f: F) { self.storage.map_inplace_ordered(f) }
}

impl<T, P, S> IntoOperation for AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: Storage<T>
{
	type OpType = OwnedProvider<T, Self>;

	fn into_operation(self) -> Self::OpType { OwnedProvider::new(self) }
}

impl<'a, T, P, S> IntoOperation for &'a AudioContainer<T, P, S>
	where T: Sample, P: SamplePackingType, S: Storage<T>
{
	type OpType = BorrowedProvider<'a, T, AudioContainer<T, P, S>>;

	fn into_operation(self) -> Self::OpType { BorrowedProvider::new(self) }
}
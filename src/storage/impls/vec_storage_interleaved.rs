use litcontainers::storage::*;
use crate::storage::{SizedAudioStorage, AudioStorage, OwnableAudio, AudioStorageMut, DynamicChannelStorage, DynamicSampleStorage};
use crate::format::*;

pub type InterleavedStorage<T, C, L> = VecStorageCM<T, C, L>;

impl<T, C, L> SizedAudioStorage<T, C, L, InterleavedPacking> for InterleavedStorage<T, C, L>
	where T: Sample, C: Dim, L: Dim
{}

impl<T, C, L> AudioStorage<T, C, L, InterleavedPacking> for InterleavedStorage<T, C, L>
	where T: Sample, C: Dim, L: Dim
{
	fn sample_rate(&self) -> i32 { 1 }
}

impl<T, C, L> AudioStorageMut<T, C, L, InterleavedPacking> for InterleavedStorage<T, C, L>
	where T: Sample, C: Dim, L: Dim
{}

impl<T, C, L> OwnableAudio<T, C, L> for InterleavedStorage<T, C, L>
	where T: Sample, C: Dim, L: Dim
{
	type OwnedPackingType = InterleavedPacking;
	type OwnedAudioType = Self;

	fn owned_audio(self) -> Self::OwnedAudioType { self.owned() }

	fn clone_owned_audio(&self) -> Self::OwnedAudioType { self.clone_owned() }
}

impl<T, L> DynamicChannelStorage<T, L> for InterleavedStorage<T, Dynamic, L>
	where T: Sample, L: Dim
{}

impl<T, C> DynamicSampleStorage<T, C> for InterleavedStorage<T, C, Dynamic>
	where T: Sample, C: Dim
{}
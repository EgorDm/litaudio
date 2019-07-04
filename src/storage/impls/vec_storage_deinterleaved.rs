use litcontainers::storage::*;
use crate::storage::{SizedAudioStorage, AudioStorage, OwnableAudio, AudioStorageMut, DynamicChannelStorage, DynamicSampleStorage};
use crate::format::*;

pub type DeinterleavedStorage<T, C, L> = VecStorageRM<T, C, L>;

impl<T, C, L> SizedAudioStorage<T, C, L, DeinterleavedPacking> for DeinterleavedStorage<T, C, L>
	where T: Sample, C: Dim, L: Dim
{}

impl<T, C, L> AudioStorage<T, C, L, DeinterleavedPacking> for DeinterleavedStorage<T, C, L>
	where T: Sample, C: Dim, L: Dim
{
	fn sample_rate(&self) -> i32 { 1 }
}

impl<T, C, L> AudioStorageMut<T, C, L, DeinterleavedPacking> for DeinterleavedStorage<T, C, L>
	where T: Sample, C: Dim, L: Dim
{
	fn set_sample_rate(&mut self, _sample_rate: i32) { }
}

impl<T, C, L> OwnableAudio<T, C, L> for DeinterleavedStorage<T, C, L>
	where T: Sample, C: Dim, L: Dim
{
	type OwnedPackingType = DeinterleavedPacking;
	type OwnedAudioType = Self;

	fn owned_audio(self) -> Self::OwnedAudioType { self.owned() }

	fn clone_owned_audio(&self) -> Self::OwnedAudioType { self.clone_owned() }
}

impl<T, L> DynamicChannelStorage<T, L> for DeinterleavedStorage<T, Dynamic, L>
	where T: Sample, L: Dim
{}

impl<T, C> DynamicSampleStorage<T, C> for DeinterleavedStorage<T, C, Dynamic>
	where T: Sample, C: Dim
{}
use litcontainers::*;
use crate::{OwnableAudio, Sample, AudioStorage, SamplePackingType, AudioStorageMut};

impl<T, S> OwnableAudio<T> for Container<T, S>
	where T: Sample, S: Storage<T> + OwnableAudio<T>
{
	type OwnedPackingType = S::OwnedPackingType;
	type OwnedAudioType = S::OwnedAudioType;

	fn owned_audio(self) -> Container<T, Self::OwnedAudioType> { self.into_storage().owned_audio() }

	fn clone_owned_audio(&self) -> Container<T, Self::OwnedAudioType> { self.storage().clone_owned_audio() }
}

impl<T, P, S> AudioStorage<T, P> for Container<T, S>
	where T: Sample, P: SamplePackingType, S: Storage<T> + AudioStorage<T, P>
{
	fn sample_rate(&self) -> i32 { self.storage().sample_rate() }
}

impl<T, P, S> AudioStorageMut<T, P> for Container<T, S>
	where T: Sample, P: SamplePackingType, S: StorageMut<T> + AudioStorageMut<T, P>
{
	fn set_sample_rate(&mut self, sample_rate: i32) { self.storage_mut().set_sample_rate(sample_rate) }
}
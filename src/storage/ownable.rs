use crate::format::*;
use crate::storage::AudioStorageMut;
use litcontainers::Ownable;

/// Type can be turned or cloned into a container which owns its data.
pub trait OwnableAudio<T: Sample, C: Dim, L: Dim>: Ownable<T, C, L> {
	type OwnedPackingType: SamplePackingType;
	type OwnedAudioType: AudioStorageMut<T, C, L, Self::OwnedPackingType>;

	/// Converts itself to a container which owns its data. No guarantees that it wont be the same
	/// container if it is already owns its data.
	fn owned_audio(self) -> Self::OwnedAudioType;

	/// Clones it's data into a container which owns its data.
	fn clone_owned_audio(&self) -> Self::OwnedAudioType;
}
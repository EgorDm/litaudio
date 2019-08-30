use crate::format::*;
use crate::storage::AudioStorageMut;
use litcontainers::{Ownable, InplaceMap, Container};

/// Type can be turned or cloned into a container which owns its data.
pub trait OwnableAudio<T: Sample>: Ownable<T> {
	type OwnedPackingType: SamplePackingType;
	type OwnedAudioType: AudioStorageMut<T, Self::OwnedPackingType> + InplaceMap<T>;

	/// Converts itself to a container which owns its data. No guarantees that it wont be the same
	/// container if it is already owns its data.
	fn owned_audio(self) -> Container<T, Self::OwnedAudioType>;

	/// Clones it's data into a container which owns its data.
	fn clone_owned_audio(&self) -> Container<T, Self::OwnedAudioType>;
}
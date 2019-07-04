use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use litcontainers::format::*;
use litcontainers::storage::*;
use litcontainers::Container;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	pub(crate) storage: S,
	_phantoms: PhantomData<(T, C, L, P)>
}

// Litcontainers implementations
impl<T, C, L, P, S> SizedStorage<C, L> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	fn row_dim(&self) -> C { self.storage.row_dim() }

	fn col_dim(&self) -> L { self.storage.col_dim() }
}

impl<T, C, L, P, S> Storage<T, C, L> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	type RStride = S::RStride;
	type CStride = S::CStride;

	fn row_stride_dim(&self) -> Self::RStride { self.storage.row_stride_dim() }

	fn col_stride_dim(&self) -> Self::CStride { self.storage.col_stride_dim() }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.storage.get_index_ptr_unchecked(i) }
}

impl<T, C, L, P, S> StorageMut<T, C, L> for AudioContainer<T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T { self. storage.get_index_mut_ptr_unchecked(i) }
}

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
		AudioContainer {
			storage: S::from_value(rows, cols, value),
			_phantoms: PhantomData
		}
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


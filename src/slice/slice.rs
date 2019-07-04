use crate::format::*;
use crate::storage::*;
use litcontainers::format::*;
use litcontainers::storage::*;
use litcontainers::Container;
use std::marker::PhantomData;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	pub(crate) storage: S,
	pub(crate) _phantoms: PhantomData<(&'a (), T, C, L, P)>
}

// Litcontainers implementations
impl<'a, T, C, L, P, S> SizedStorage<C, L> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	fn row_dim(&self) -> C { self.storage.row_dim() }

	fn col_dim(&self) -> L { self.storage.col_dim() }
}

impl<'a, T, C, L, P, S> Storage<T, C, L> for AudioSliceBase<'a, T, C, L, P, S>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	type RStride = S::RStride;
	type CStride = S::CStride;

	fn row_stride_dim(&self) -> Self::RStride { self.storage.row_stride_dim() }

	fn col_stride_dim(&self) -> Self::CStride { self.storage.col_stride_dim() }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.storage.get_index_ptr_unchecked(i) }
}

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

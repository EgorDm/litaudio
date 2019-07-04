use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use std::mem;

macro_rules! iter_ptr_impl {
	(
		struct $Name: ident : $StorageType: ident as $StorageRef: ty {
			$ptr_fn: ident -> $ElementPtr: ty as $ElementRet: ty,
			primary: $prim_size_fn: ident, $span_fn: ident,
			secondary: $scnd_size_fn: ident, $scnd_stride_fn: ident
		} // TODO: use unsafe $ptr_fn which is safe since we stay within the bounds
	) => {
		pub struct $Name<'a, T, C, L, P, S>
			where T: Sample + 'a, C: Dim, L: Dim, P: SamplePackingType, S: $StorageType<T, C, L, P>
		{
			storage: $StorageRef,
			ptr: $ElementPtr,
			ptr_end: $ElementPtr,
			cursor: usize,
			cursor_end: usize,
			_phantoms: PhantomData<(C, L, P)>
		}

		impl<'a, T, C, L, P, S> $Name<'a, T, C, L, P, S>
			where T: Sample + 'a, C: Dim, L: Dim, P: SamplePackingType, S: $StorageType<T, C, L, P>
		{
			pub fn new(storage: $StorageRef) -> Self {
				Self::from_range(storage, 0, storage.$prim_size_fn())
			}

			pub fn from_range(storage: $StorageRef, cursor: usize, cursor_end: usize) -> Self {
				let ptr = storage.$ptr_fn(cursor);
				let ptr_end = unsafe { ptr.offset(storage.$span_fn(cursor) as isize) };
				Self {
					storage,
					ptr,
					ptr_end,
					cursor,
					cursor_end,
					_phantoms: PhantomData
				}
			}
		}

		impl<'a, T, C, L, P, S> Iterator for $Name<'a, T, C, L, P, S>
			where T: Sample + 'a, C: Dim, L: Dim, P: SamplePackingType, S: $StorageType<T, C, L, P>
		{
			type Item = $ElementRet;

			#[inline]
			fn next(&mut self) -> Option<Self::Item> {
				if self.ptr < self.ptr_end {
					let old = self.ptr;
					unsafe {
						self.ptr = self.ptr.offset(self.storage.$scnd_stride_fn() as isize);
						Some(mem::transmute(old))
					}
				} else if self.cursor < self.cursor_end - 1 {
					self.cursor += 1;
					self.ptr = self.storage.$ptr_fn(self.cursor);
					let size = self.storage.$span_fn(self.cursor);
					self.ptr_end = unsafe { self.ptr.offset(size as isize)};
					self.next()
				} else {
					None
				}
			}

			#[inline]
			fn count(self) -> usize {
				self.size_hint().0
			}

			#[inline]
			fn size_hint(&self) -> (usize, Option<usize>) {
				let size = (self.cursor_end - self.cursor) * self.storage.$scnd_size_fn();
				(size, Some(size))
			}
		}
	}
}

iter_ptr_impl! {
	struct ChannelIterPtr : AudioStorage as &'a S {
		as_channel_ptr -> *const T as &'a T,
		primary: channel_count, channel_index_span,
		secondary: sample_count, sample_stride
	}
}

iter_ptr_impl! {
	struct ChannelIterMutPtr : AudioStorageMut as &'a mut S {
		as_channel_mut_ptr -> *mut T as &'a mut T,
		primary: channel_count, channel_index_span,
		secondary: sample_count, sample_stride
	}
}

iter_ptr_impl! {
	struct SampleIterPtr : AudioStorage as &'a S {
		as_col_ptr -> *const T as &'a T,
		primary: col_count, col_index_span,
		secondary: row_count, row_stride
	}
}

iter_ptr_impl! {
	struct SampleIterMutPtr : AudioStorageMut as &'a mut S {
		as_sample_mut_ptr -> *mut T as &'a mut T,
		primary: sample_count, sample_index_span,
		secondary: channel_count, channel_stride
	}
}
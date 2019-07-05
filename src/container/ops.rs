use litcontainers::*;
use crate::*;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use crate::slice::{AudioSliceBase};

macro_rules! impl_binary_dual_op (
	($OpTrait: ident, $op_fn: ident, $OpAssignTrait: ident, $op_assign_fn: ident) => {
		// Add container
		impl<T, C, L, P, S, TR, RR, CR, SR> $OpTrait<&Container<TR, RR, CR, SR>> for AudioContainer<T, C, L, P, S>
			where
				T: Sample + $OpAssignTrait<TR>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Scalar, RR: Dim, CR: Dim, SR: StorageMut<TR, RR, CR>,
		{
			type Output = <Self as OwnableAudio<T, C, L>>::OwnedAudioType;

			fn $op_fn(self, rhs: &Container<TR, RR, CR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.owned_audio();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		impl<T, C, L, P, S, TR, RR, CR, SR> $OpTrait<&Container<TR, RR, CR, SR>> for &AudioContainer<T, C, L, P, S>
			where
				T: Sample + $OpAssignTrait<TR>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Scalar, RR: Dim, CR: Dim, SR: StorageMut<TR, RR, CR>,
		{
			type Output = AudioContainer<T, C, L, S::OwnedPackingType, S::OwnedAudioType>;

			fn $op_fn(self, rhs: &Container<TR, RR, CR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.clone_owned_audio();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		// Add audiocontainer
		impl<T, C, L, P, S, TR, CR, LR, PR, SR> $OpTrait<&AudioContainer<TR, CR, LR, PR, SR>> for AudioContainer<T, C, L, P, S>
			where
				T: Sample + $OpAssignTrait<TR>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Sample, CR: Dim, LR: Dim, PR: SamplePackingType, SR: AudioStorageMut<TR, CR, LR, PR>,
		{
			type Output = <Self as OwnableAudio<T, C, L>>::OwnedAudioType;

			fn $op_fn(self, rhs: &AudioContainer<TR, CR, LR, PR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.owned_audio();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		impl<T, C, L, P, S, TR, CR, LR, PR, SR> $OpTrait<&AudioContainer<TR, CR, LR, PR, SR>> for &AudioContainer<T, C, L, P, S>
			where
				T: Sample + $OpAssignTrait<TR>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Sample, CR: Dim, LR: Dim, PR: SamplePackingType, SR: AudioStorageMut<TR, CR, LR, PR>,
		{
			type Output = AudioContainer<T, C, L, S::OwnedPackingType, S::OwnedAudioType>;

			fn $op_fn(self, rhs: &AudioContainer<TR, CR, LR, PR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.clone_owned_audio();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		// Add slice
		impl<'a, T, C, L, P, S, TR, RR, CR, SR> $OpTrait<&SliceBase<'a, TR, RR, CR, SR>> for AudioContainer<T, C, L, P, S>
			where
				T: Sample + $OpAssignTrait<TR>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Scalar, RR: Dim, CR: Dim, SR: Storage<TR, RR, CR>
		{
			type Output = <Self as OwnableAudio<T, C, L>>::OwnedAudioType;

			fn $op_fn(self, rhs: &SliceBase<TR, RR, CR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.owned_audio();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		impl<'a, T, C, L, P, S, TR, RR, CR, SR> $OpTrait<&SliceBase<'a, TR, RR, CR, SR>> for &AudioContainer<T, C, L, P, S>
			where
				T: Sample + $OpAssignTrait<TR>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Scalar, RR: Dim, CR: Dim, SR: Storage<TR, RR, CR>
		{
			type Output = AudioContainer<T, C, L, S::OwnedPackingType, S::OwnedAudioType>;

			fn $op_fn(self, rhs: &SliceBase<'a, TR, RR, CR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.clone_owned_audio();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		// Add audioslice
		impl<'a, T, C, L, P, S, TR, CR, LR, PR, SR> $OpTrait<&AudioSliceBase<'a, TR, CR, LR, PR, SR>> for AudioContainer<T, C, L, P, S>
			where
				T: Sample + $OpAssignTrait<TR>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Sample, CR: Dim, LR: Dim, PR: SamplePackingType, SR: AudioStorage<TR, CR, LR, PR>,
		{
			type Output = <Self as OwnableAudio<T, C, L>>::OwnedAudioType;

			fn $op_fn(self, rhs: &AudioSliceBase<'a, TR, CR, LR, PR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.owned_audio();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		impl<'a, T, C, L, P, S, TR, CR, LR, PR, SR> $OpTrait<&AudioSliceBase<'a, TR, CR, LR, PR, SR>> for &AudioContainer<T, C, L, P, S>
			where
				T: Sample + $OpAssignTrait<TR>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Sample, CR: Dim, LR: Dim, PR: SamplePackingType, SR: AudioStorage<TR, CR, LR, PR>,
		{
			type Output = AudioContainer<T, C, L, S::OwnedPackingType, S::OwnedAudioType>;

			fn $op_fn(self, rhs: &AudioSliceBase<'a, TR, CR, LR, PR, SR>) -> Self::Output {
				assert!(self.equal_size(rhs), "Rhs must have the same size!");
				let mut ret = self.clone_owned_audio();
				for (o, s) in ret.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
				ret
			}
		}

		// Add scalar
		impl<T, C, L, P, S, TR> $OpTrait<TR> for AudioContainer<T, C, L, P, S>
			where
				T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Sample, T: $OpAssignTrait<TR>
		{
			type Output = <Self as OwnableAudio<T, C, L>>::OwnedAudioType;

			fn $op_fn(self, rhs: TR) -> Self::Output {
				let mut ret = self.owned_audio();
				for o in ret.as_row_mut_iter() { o.$op_assign_fn(rhs); }
				ret
			}
		}

		impl<T, C, L, P, S, TR> $OpTrait<TR> for &AudioContainer<T, C, L, P, S>
			where
				T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Sample, T: $OpAssignTrait<TR>
		{
			type Output = AudioContainer<T, C, L, S::OwnedPackingType, S::OwnedAudioType>;

			fn $op_fn(self, rhs: TR) -> Self::Output {
				let mut ret = self.clone_owned_audio();
				for o in ret.as_row_mut_iter() { o.$op_assign_fn(rhs); }
				ret
			}
		}

		// Add assign
		impl<T, C, L, P, S, TR, RR, CR, SR> $OpAssignTrait<&Container<TR, RR, CR, SR>> for AudioContainer<T, C, L, P, S>
			where
				T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Scalar, RR: Dim, CR: Dim, SR: StorageMut<TR, RR, CR>,
				T: $OpAssignTrait<TR>
		{
			fn $op_assign_fn(&mut self, rhs: &Container<TR, RR, CR, SR>) {
				for (o, s) in self.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
			}
		}

		impl<T, C, L, P, S, TR, CR, LR, PR, SR> $OpAssignTrait<&AudioContainer<TR, CR, LR, PR, SR>> for AudioContainer<T, C, L, P, S>
			where
				T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Sample, CR: Dim, LR: Dim, PR: SamplePackingType, SR: AudioStorageMut<TR, CR, LR, PR>,
				T: $OpAssignTrait<TR>
		{
			fn $op_assign_fn(&mut self, rhs: &AudioContainer<TR, CR, LR, PR, SR>) {
				for (o, s) in self.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
			}
		}

		impl<'a, T, C, L, P, S, TR, RR, CR, SR> $OpAssignTrait<&SliceBase<'a, TR, RR, CR, SR>> for AudioContainer<T, C, L, P, S>
			where
				T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Scalar, RR: Dim, CR: Dim, SR: Storage<TR, RR, CR>,
				T: $OpAssignTrait<TR>
		{
			fn $op_assign_fn(&mut self, rhs: &SliceBase<'a, TR, RR, CR, SR>) {
				for (o, s) in self.as_row_mut_iter().zip(rhs.as_row_iter()) {
					o.$op_assign_fn(*s);
				}
			}
		}

		impl<T, C, L, P, S, TR> $OpAssignTrait<TR> for AudioContainer<T, C, L, P, S>
			where
				T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
				TR: Sample, T: $OpAssignTrait<TR>
		{
			fn $op_assign_fn(&mut self, rhs: TR) {
				for o in self.as_row_mut_iter() {
					o.$op_assign_fn(rhs);
				}
			}
		}
	}
);

impl_binary_dual_op!(Add, add, AddAssign, add_assign);
impl_binary_dual_op!(Sub, sub, SubAssign, sub_assign);
impl_binary_dual_op!(Mul, mul, MulAssign, mul_assign);
impl_binary_dual_op!(Div, div, DivAssign, div_assign);

impl<T, C, L, P, S> Neg for AudioContainer<T, C, L, P, S>
	where T: Sample + Neg<Output=T>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	type Output = <Self as OwnableAudio<T, C, L>>::OwnedAudioType;

	fn neg(self) -> Self::Output {
		let mut ret = self.owned_audio();
		for o in ret.as_row_mut_iter() {
			*o = o.neg();
		}
		ret
	}
}

impl<T, C, L, P, S> Neg for &AudioContainer<T, C, L, P, S>
	where T: Sample + Neg<Output=T>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
{
	type Output = AudioContainer<T, C, L, S::OwnedPackingType, S::OwnedAudioType>;

	fn neg(self) -> Self::Output {
		let mut ret = self.clone_owned_audio();
		for o in ret.as_row_mut_iter() {
			*o = o.neg();
		}
		ret
	}
}
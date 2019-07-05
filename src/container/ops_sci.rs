use litcontainers::*;
use crate::*;
use num_traits::Float;

macro_rules! impl_unary_float_op (
	($OpTrait: ident, $op_fn: ident, $OpAssignTrait: ident, $op_assign_fn: ident) => {
		impl<T, C, L, P, S> $OpTrait for AudioContainer<T, C, L, P, S>
			where
				T: Sample + Float, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
		{
			type Output = <Self as OwnableAudio<T, C, L>>::OwnedAudioType;

			fn $op_fn(self) -> Self::Output {
				let mut ret = self.owned_audio();
				for o in ret.as_row_mut_iter() {
					*o = o.$op_fn();
				}
				ret
			}
		}

		impl<T, C, L, P, S> $OpTrait for &AudioContainer<T, C, L, P, S>
			where
				T: Sample + Float, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
		{
			type Output = AudioContainer<T, C, L, S::OwnedPackingType, S::OwnedAudioType>;

			fn $op_fn(self) -> Self::Output {
				let mut ret = self.clone_owned_audio();
				for o in ret.as_row_mut_iter() {
					*o = o.$op_fn();
				}
				ret
			}
		}

		impl<T, C, L, P, S> $OpAssignTrait for AudioContainer<T, C, L, P, S>
			where
				T: Sample + Float, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
		{
			fn $op_assign_fn(&mut self) {
				for o in self.as_row_mut_iter() {
					*o = o.$op_fn();
				}
			}
		}
	}
);

impl_unary_float_op!(ASin, asin, ASinAssign, asin_assign);
impl_unary_float_op!(Sin, sin, SinAssign, sin_assign);
impl_unary_float_op!(ACos, acos, ACosAssign, acos_assign);
impl_unary_float_op!(Cos, cos, CosAssign, cos_assign);
impl_unary_float_op!(Tan, tan, TanAssign, tan_assign);
impl_unary_float_op!(ATan, atan, ATanAssign, atan_assign);
impl_unary_float_op!(Exp, exp, ExpAssign, exp_assign);
impl_unary_float_op!(Ceil, ceil, CeilAssign, ceil_assign);
impl_unary_float_op!(Floor, floor, FloorAssign, floor_assign);
impl_unary_float_op!(Round, round, RoundAssign, round_assign);
impl_unary_float_op!(Abs, abs, AbsAssign, abs_assign);
impl_unary_float_op!(Sqrt, sqrt, SqrtAssign, sqrt_assign);
impl_unary_float_op!(Log2, log2, Log2Assign, log2_assign);
impl_unary_float_op!(Log10, log10, Log10Assign, log10_assign);
impl_unary_float_op!(Ln, ln, LnAssign, ln_assign);

impl<T, C, L, P, S, RT> Pow<RT> for AudioContainer<T, C, L, P, S>
	where
		T: Sample + num_traits::Pow<RT, Output=T>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>, RT: Sample
{
	type Output = <Self as OwnableAudio<T, C, L>>::OwnedAudioType;

	fn pow(self, rhs: RT) -> Self::Output {
		let mut ret = self.owned_audio();
		for o in ret.as_row_mut_iter() {
			*o = num_traits::Pow::pow(*o, rhs);
		}
		ret
	}
}

impl<T, C, L, P, S, RT> Pow<RT> for &AudioContainer<T, C, L, P, S>
	where
		T: Sample + num_traits::Pow<RT, Output=T>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>, RT: Sample
{
	type Output = AudioContainer<T, C, L, S::OwnedPackingType, S::OwnedAudioType>;

	fn pow(self, rhs: RT) -> Self::Output {
		let mut ret = self.clone_owned_audio();
		for o in ret.as_row_mut_iter() {
			*o = num_traits::Pow::pow(*o, rhs);
		}
		ret
	}
}

impl<T, C, L, P, S, RT> PowAssign<RT> for AudioContainer<T, C, L, P, S>
	where
		T: Sample + num_traits::Pow<RT, Output=T>, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>, RT: Sample
{
	fn pow_assign(&mut self, rhs: RT) {
		for o in self.as_row_mut_iter() {
			*o = num_traits::Pow::pow(*o, rhs);
		}
	}
}


macro_rules! impl_binary_float_op (
	($OpTrait: ident, $op_fn: ident, $OpAssignTrait: ident, $op_assign_fn: ident) => {
		impl<T, C, L, P, S> $OpTrait<T> for AudioContainer<T, C, L, P, S>
			where
				T: Sample + Float, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
		{
			type Output = <Self as OwnableAudio<T, C, L>>::OwnedAudioType;

			fn $op_fn(self, rhs: T) -> Self::Output {
				let mut ret = self.owned_audio();
				for o in ret.as_row_mut_iter() {
					*o = o.$op_fn(rhs);
				}
				ret
			}
		}

		impl<T, C, L, P, S> $OpTrait<T> for &AudioContainer<T, C, L, P, S>
			where
				T: Sample + Float, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>,
		{
			type Output = AudioContainer<T, C, L, S::OwnedPackingType, S::OwnedAudioType>;

			fn $op_fn(self, rhs: T) -> Self::Output {
				let mut ret = self.clone_owned_audio();
				for o in ret.as_row_mut_iter() {
					*o = o.$op_fn(rhs);
				}
				ret
			}
		}

		impl<T, C, L, P, S> $OpAssignTrait<T> for AudioContainer<T, C, L, P, S>
			where
				T: Sample + Float, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, L, P>
		{
			fn $op_assign_fn(&mut self, rhs: T) {
				for o in self.as_row_mut_iter() {
					*o = o.$op_fn(rhs);
				}
			}
		}
	}
);

impl_binary_float_op!(Log, log, LogAssign, log_assign);
impl_binary_float_op!(Max, max, MaxAssign, max_assign);
impl_binary_float_op!(Min, min, MinAssign, min_assign);

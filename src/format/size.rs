use litcontainers::format::*;
use litcontainers::StorageSize;

pub trait SizedAudio: StorageSize {
	#[inline]
	fn channels(&self) -> usize { self.channel_dim().value() }

	#[inline]
	fn channel_dim(&self) -> Self::Rows { self.row_dim() }

	#[inline]
	fn samples(&self) -> usize { self.sample_dim().value() }

	#[inline]
	fn sample_dim(&self) -> Self::Cols { self.col_dim() }
}

impl<T: StorageSize> SizedAudio for T {}
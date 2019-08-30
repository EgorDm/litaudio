use litcontainers::format::*;
use litcontainers::Strided;

pub trait AudioStrided: Strided {
	#[inline]
	fn channel_stride(&self) -> usize { self.channel_stride_dim().value() }

	#[inline]
	fn channel_stride_dim(&self) -> Self::RowStride { self.row_stride_dim() }

	#[inline]
	fn sample_stride(&self) -> usize { self.sample_stride_dim().value() }

	#[inline]
	fn sample_stride_dim(&self) -> Self::ColStride { self.col_stride_dim() }
}

impl<T: Strided> AudioStrided for T {}
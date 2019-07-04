use crate::format::*;
use litcontainers::{DynamicRowStorage, DynamicColStorage};

pub trait DynamicChannelStorage<T, L>: DynamicRowStorage<T, L>
	where T: Sample, L: Dim
{
	#[inline]
	fn set_channel_count(&mut self, count: usize) { self.set_row_count(count) }
}

pub trait DynamicSampleStorage<T, C>: DynamicColStorage<T, C>
	where T: Sample, C: Dim
{
	#[inline]
	fn set_sample_count(&mut self, count: usize) { self.set_col_count(count) }
}


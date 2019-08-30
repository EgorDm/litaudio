use crate::format::*;
use litcontainers::{DynamicRowStorage, DynamicColStorage};

pub trait DynamicChannelStorage<T: Sample>: DynamicRowStorage<T>
{
	#[inline]
	fn set_channels(&mut self, count: usize) { self.set_rows(count) }
}

pub trait DynamicSampleStorage<T: Sample>: DynamicColStorage<T>
{
	#[inline]
	fn set_samples(&mut self, count: usize) { self.set_cols(count) }
}


impl<T: Sample, S: DynamicRowStorage<T>> DynamicChannelStorage<T> for S {}
impl<T: Sample, S: DynamicColStorage<T>> DynamicSampleStorage<T> for S {}

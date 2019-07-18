use litcontainers::*;
use litaudio::*;

fn mock_container() -> AudioDeinterleaved<f64, U3, Dynamic> {
	AudioDeinterleaved::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.])
}

#[test]
fn sized() {
	let s = mock_container();
	let s2 = mock_container();

	assert_eq!(s.channel_count(), 3);
	assert_eq!(s.sample_count(), 2);
	assert_eq!(s.channel_stride(), s.sample_count());
	assert_eq!(s.sample_stride(), 1);
	assert!(s.equal_size(&s2));
}

#[test]
fn indexing() {
	let s = mock_container();

	assert_eq!(s.calc_index(1, 1), 3);
	assert_eq!(s.channel_index(2), 4);
	assert_eq!(s.sample_index(2), 2);
	assert_eq!(s.get(1, 1), 4.);
	assert_eq!(*s.get_ref(1, 1), 4.);
	assert_eq!(s.as_channel_slice(1), [3., 4.]);
	assert_eq!(s.as_sample_slice(1), [2., 3., 4., 5., 6.]);
}

#[test]
fn resizing_upsize() {
	let mut s = AudioDeinterleaved::from_vec(Dynamic::new(2), Dynamic::new(2), &[1., 2., 3., 4.]);
	s.resize_sample_count(3);
	assert_eq!(s.sample_count(), 3);
	assert_eq!(s.as_slice(), [1., 2., 0., 3., 4., 0.]);

	let mut s = AudioDeinterleaved::from_vec(Dynamic::new(2), Dynamic::new(2), &[1., 2., 3., 4.]);
	s.resize_channel_count(3);
	assert_eq!(s.channel_count(), 3);
	assert_eq!(s.as_slice(), [1., 2., 3., 4., 0., 0.]);

	let mut s = AudioInterleaved::from_vec(Dynamic::new(2), Dynamic::new(2), &[1., 2., 3., 4.]);
	s.resize_sample_count(3);
	assert_eq!(s.sample_count(), 3);
	assert_eq!(s.as_slice(), [1., 3., 2., 4., 0., 0.]);

	let mut s = AudioInterleaved::from_vec(Dynamic::new(2), Dynamic::new(2), &[1., 2., 3., 4.]);
	s.resize_channel_count(3);
	assert_eq!(s.channel_count(), 3);
	assert_eq!(s.as_slice(), [1., 3., 0., 2., 4., 0.]);
}

#[test]
fn resizing_downsize() {
	let mut s = AudioDeinterleaved::from_vec(Dynamic::new(2), Dynamic::new(2), &[1., 2., 3., 4.]);
	s.resize_sample_count(1);
	assert_eq!(s.sample_count(), 1);
	assert_eq!(s.as_slice(), [1., 3.]);

	let mut s = AudioDeinterleaved::from_vec(Dynamic::new(2), Dynamic::new(2), &[1., 2., 3., 4.]);
	s.resize_channel_count(1);
	assert_eq!(s.channel_count(), 1);
	assert_eq!(s.as_slice(), [1., 2.]);

	let mut s = AudioInterleaved::from_vec(Dynamic::new(2), Dynamic::new(2), &[1., 2., 3., 4.]);
	s.resize_sample_count(1);
	assert_eq!(s.sample_count(), 1);
	assert_eq!(s.as_slice(), [1., 3.]);

	let mut s = AudioInterleaved::from_vec(Dynamic::new(2), Dynamic::new(2), &[1., 2., 3., 4.]);
	s.resize_channel_count(1);
	assert_eq!(s.channel_count(), 1);
	assert_eq!(s.as_slice(), [1., 2.]);
}

#[test]
fn mutable() {
	let mut s = mock_container();

	assert_eq!(s.as_channel_mut_slice(1), [3., 4.]);
	assert_eq!(s.as_sample_mut_slice(1), [2., 3., 4., 5., 6.]);

	*s.get_mut(1, 1) = 1337.;
	assert_eq!(s.get(1, 1), 1337.);
}

#[test]
fn slice() {
	let s = mock_container();
	assert_eq!(s.slice_channels(1..3).channel_count(), 2);
	assert_eq!(s.slice_channels(1..3).as_slice(), [3., 4., 5., 6.]);
	assert_eq!(s.slice_samples(1).sample_count(), 1);
	assert_eq!(s.slice_samples(1).as_iter().cloned().collect::<Vec<_>>(), &[2., 4., 6.]);
	assert_eq!(s.slice_channels(1..3).slice_samples(1).as_iter().cloned().collect::<Vec<_>>(), &[4., 6.]);
	assert_eq!(s.slice_channels(1..3).slice_samples(1).slice_channels(1).as_iter().cloned().collect::<Vec<_>>(), &[6.]);
}
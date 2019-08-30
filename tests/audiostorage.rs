use litcontainers::*;
use litaudio::*;

fn mock_container() -> AudioDeinterleaved<f64, U3, Dynamic> {
	AudioDeinterleaved::from_vec(Size::new(U3, Dynamic::new(2)), &[1., 2., 3., 4., 5., 6.])
}

#[test]
fn sized() {
	let s = mock_container();
	let s2 = mock_container();

	assert_eq!(s.channels(), 3);
	assert_eq!(s.samples(), 2);
	assert_eq!(s.channel_stride(), s.samples());
	assert_eq!(s.sample_stride(), 1);
	assert!(s.equal_size(&s2));
}

#[test]
fn resizing_upsize() {
	let mut s = AudioDeinterleaved::from_vec(Size::new(Dynamic::new(2), Dynamic::new(2)), &[1., 2., 3., 4.]);
	s.set_samples(3);
	assert_eq!(s.samples(), 3);
	assert_eq!(s.as_slice(), [1., 2., 0., 3., 4., 0.]);

	let mut s = AudioDeinterleaved::from_vec(Size::new(Dynamic::new(2), Dynamic::new(2)), &[1., 2., 3., 4.]);
	s.set_channels(3);
	assert_eq!(s.channels(), 3);
	assert_eq!(s.as_slice(), [1., 2., 3., 4., 0., 0.]);

	let mut s = AudioInterleaved::from_vec(Size::new(Dynamic::new(2), Dynamic::new(2)), &[1., 2., 3., 4.]);
	s.set_samples(3);
	assert_eq!(s.samples(), 3);
	assert_eq!(s.as_slice(), [1., 3., 2., 4., 0., 0.]);

	let mut s = AudioInterleaved::from_vec(Size::new(Dynamic::new(2), Dynamic::new(2)), &[1., 2., 3., 4.]);
	s.set_channels(3);
	assert_eq!(s.channels(), 3);
	assert_eq!(s.as_slice(), [1., 3., 0., 2., 4., 0.]);
}

#[test]
fn resizing_downsize() {
	let mut s = AudioDeinterleaved::from_vec(Size::new(Dynamic::new(2), Dynamic::new(2)), &[1., 2., 3., 4.]);
	s.set_samples(1);
	assert_eq!(s.samples(), 1);
	assert_eq!(s.as_slice(), [1., 3.]);

	let mut s = AudioDeinterleaved::from_vec(Size::new(Dynamic::new(2), Dynamic::new(2)), &[1., 2., 3., 4.]);
	s.set_channels(1);
	assert_eq!(s.channels(), 1);
	assert_eq!(s.as_slice(), [1., 2.]);

	let mut s = AudioInterleaved::from_vec(Size::new(Dynamic::new(2), Dynamic::new(2)), &[1., 2., 3., 4.]);
	s.set_samples(1);
	assert_eq!(s.samples(), 1);
	assert_eq!(s.as_slice(), [1., 3.]);

	let mut s = AudioInterleaved::from_vec(Size::new(Dynamic::new(2), Dynamic::new(2)), &[1., 2., 3., 4.]);
	s.set_channels(1);
	assert_eq!(s.channels(), 1);
	assert_eq!(s.as_slice(), [1., 2.]);
}

#[test]
fn mutable() {
	let mut s = mock_container();

	*s.get_mut(1, 1) = 1337.;
	assert_eq!(s.get(1, 1), 1337.);
}

#[test]
fn slice() {
	let s = mock_container();
	assert_eq!(s.slice_channels(1..3).channels(), 2);
	assert_eq!(s.slice_channels(1..3).as_slice(), [3., 4., 5., 6.]);
	assert_eq!(s.slice_samples(1).samples(), 1);
	assert_eq!(s.slice_samples(1).as_iter().cloned().collect::<Vec<_>>(), &[2., 4., 6.]);
	assert_eq!(s.slice_channels(1..3).slice_samples(1).as_iter().cloned().collect::<Vec<_>>(), &[4., 6.]);
	assert_eq!(s.slice_channels(1..3).slice_samples(1).slice_channels(1).as_iter().cloned().collect::<Vec<_>>(), &[6.]);
}
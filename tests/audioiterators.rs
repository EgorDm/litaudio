use litcontainers::*;
use litaudio::*;

fn mock_container() -> AudioDeinterleaved<f64, U3, Dynamic> {
	AudioDeinterleaved::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.])
}

#[test]
fn iter() {
	let mut s = mock_container();
	assert_eq!(s.as_channel_slice_iter(1).cloned().collect::<Vec<_>>(), &[3., 4.]);
	assert_eq!(s.as_channel_slice_mut_iter(1).map(|x| *x).collect::<Vec<_>>(), &[3., 4.]);
	assert_eq!(s.slice_channels(1..3).as_channel_slice_iter(0).cloned().collect::<Vec<_>>(), &[3., 4.]);
	assert_eq!(s.slice_samples(1).as_channel_slice_iter(1).cloned().collect::<Vec<_>>(), &[4.]);

	assert_eq!(s.as_sample_slice_iter(1).cloned().collect::<Vec<_>>(), &[2., 4., 6.]);
	assert_eq!(s.as_sample_slice_mut_iter(1).map(|x| *x).collect::<Vec<_>>(), &[2., 4., 6.]);
	assert_eq!(s.slice_channels(1..3).as_sample_slice_iter(1).cloned().collect::<Vec<_>>(), &[4., 6.]);
	assert_eq!(s.slice_samples(1).as_sample_slice_iter(0).cloned().collect::<Vec<_>>(), &[2., 4., 6.]);

	assert_eq!(s.as_channel_slice_iter(2).cloned().collect::<Vec<_>>(), &[5., 6.]);
	assert_eq!(s.as_iter().cloned().collect::<Vec<_>>(), &[1., 2., 3., 4., 5., 6.]);
}

#[test]
fn size() {
	let s = mock_container();
	assert_eq!(s.as_channel_slice_iter(1).size_hint().0, s.sample_count());
	assert_eq!(s.as_channel_slice_iter(0..2).size_hint().0, 2 * s.sample_count());
	assert_eq!(s.as_sample_slice_iter(1).size_hint().0, s.channel_count());
	assert_eq!(s.as_sample_slice_iter(0..2).size_hint().0, 2 * s.channel_count());
}

#[test]
fn ops() {
	let s = AudioDeinterleaved::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.]);
	let s1 = AudioInterleaved::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.]);

	assert_eq!((&s + &s1).as_slice(), [2., 4., 6., 8., 10., 12.]);
	assert_eq!((&s * &s1).as_slice(), [1., 4., 9., 16., 25., 36.]);

	{
		let mut s = s.clone_owned_audio();
		s += &s1;
		assert_eq!(s.as_slice(), [2., 4., 6., 8., 10., 12.]);
	}

	assert_eq!((s.slice_channels(0..3) + &s1).as_slice(), [2., 4., 6., 8., 10., 12.]);
	let s2 = s1.slice_channels(0..3);
	assert_eq!((s.slice_channels(0..3) + &s2).as_slice(), [2., 4., 6., 8., 10., 12.]);

	assert_eq!((&s + 1.).as_slice(), [2., 3., 4., 5., 6., 7.]);
	assert_eq!((-&s).as_slice(), [-1., -2., -3., -4., -5., -6.]);
}

#[test]
fn ops_sci() {
	let s = AudioDeinterleaved::from_vec(U3, Dynamic::new(2), &[1., 2., 3., 4., 5., 6.]);

	assert_eq!((&s - 0.1).ceil().as_slice(), [1., 2., 3., 4., 5., 6.]);
	assert_eq!((&s - 0.1).floor().as_slice(), [0., 1., 2., 3., 4., 5.]);
	assert_eq!((&s).max(2.).as_slice(), [2., 2., 3., 4., 5., 6.]);
	assert_eq!((&s).pow(2).as_slice(), [1., 4., 9., 16., 25., 36.]);
}
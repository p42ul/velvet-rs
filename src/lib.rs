use dasp::{sample::FromSample, Sample};
use easyfft::prelude::*;
use hound;
use rand::{distributions::Standard, prelude::Distribution, random};


const SAMPLE_RATE: u32 = 44_100;

pub fn white_noise<S>(len: usize) -> Vec<S>
where
    S: Sample,
    Standard: Distribution<S>,
{
    return (0..len).map(|_| random::<S>()).collect();
}

pub fn naive_convolve(s1: Vec<f32>, s2: Vec<f32>) -> Vec<f32> {
    let (big, mut small) = if s1.len() > s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let signal_length = big.len();
    small.resize(signal_length, 0.0);
    let big_dft = &big[..].real_fft();
    let small_dft = &small[..].real_fft();
    let idft = big_dft * small_dft;
    let output = idft.real_ifft();
    return output
        .to_vec()
        .iter()
        .map(|x| x / signal_length as f32)
        .collect();
}

pub fn read_wav<S>(filename: String) -> Result<Vec<S>, hound::Error>
where
    S: Sample + FromSample<i16>,
{
    let mut reader = hound::WavReader::open(filename)?;
    return Ok(reader
        .samples::<i16>()
        .map(|s| s.unwrap().to_sample::<S>())
        .collect());
}

pub fn output_wav<S>(filename: String, buffer: &Vec<S>) -> Result<(), hound::Error>
where
    S: Sample,
    i16: FromSample<S>,
{
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec)?;

    for &sample in buffer.iter() {
        writer.write_sample(sample.to_sample::<i16>())?;
    }

    writer.finalize()
}

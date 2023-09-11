use dasp::{sample::FromSample, Sample};
use easyfft::{prelude::*, FftNum, dyn_size::realfft::DynRealDft};
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

pub fn naive_convolve<S>(s1: &Vec<S>, s2: &Vec<S>) -> Vec<S>
where
    S: Sample,
{
    let (big, small) = if s1.len() > s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let mut output: Vec<S> = vec![S::EQUILIBRIUM; big.len() + small.len() - 1];
    for i in 0..output.len() {
        for j in 0..small.len() {
            let x_index = match i.checked_sub(j) {
                Some(val) => val,
                None => continue,
            };
            let x = match big.get(x_index) {
                Some(val) => val,
                None => continue,
            };
            let h = match small.get(j) {
                Some(val) => val,
                None => continue,
            };
            let product = x.mul_amp(h.to_float_sample());
            output[i] = output[i].add_amp(product.to_sample::<S>().to_signed_sample());
        }
    }
    output
}

pub fn fft_convolve<S>(s1: &Vec<S>, s2: &Vec<S>) -> Vec<S>
where
    S: Sample + FftNum + Default + FromSample<f32>,
{
    let (big, small) = if s1.len() > s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let mut big = big.clone();
    let mut small = small.clone();
    let output_length = big.len() + small.len() - 1;
    big.resize(output_length, S::EQUILIBRIUM);
    small.resize(output_length, S::EQUILIBRIUM);
    let big_dft = &big[..].real_fft();
    let small_dft = &small[..].real_fft();
    let mult: DynRealDft<S> = big_dft * small_dft;
    let output = mult.real_ifft();
    let divisor = (output_length as f32).to_sample::<S>();
    output.to_vec().iter().map(|&x| x / divisor).collect()
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

use dasp::{sample::FromSample, Sample};
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
    let (len_big, len_small) = (big.len(), small.len());
    let mut output: Vec<S> = vec![S::EQUILIBRIUM; len_big];
    for i in 0..len_big {
        for j in 0..len_small {
            let big_signal = s1.get(i+j).unwrap_or(&S::EQUILIBRIUM).to_float_sample();
            let small_signal = s2.get(j).unwrap_or(&S::EQUILIBRIUM).to_float_sample();
            let product = big_signal.mul_amp(small_signal);
            output[i] = output[i].add_amp(product.to_signed_sample());
        }
    }
    output
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

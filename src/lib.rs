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
        writer.write_sample(sample.to_sample::<i16>()).unwrap();
    }

    writer.finalize()?;
    Ok(())
}

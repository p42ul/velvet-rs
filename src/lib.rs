use dasp::Sample;
use hound;
use rand::random;

// If it's good enough for STK, it's good enough for us.
type MySample = f64;

pub fn white_noise(len: usize) -> Vec<MySample> {
    return (0..len).map(|_| random::<MySample>()).collect();
}

pub fn output_wav(filename: String, buffer: &Vec<MySample>) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec)?;

    for &sample in buffer.iter() {
        writer.write_sample(sample.to_sample::<i16>()).unwrap();
    }

    writer.finalize()?;

    println!("WAV file created");
    Ok(())
}

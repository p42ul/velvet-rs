use easyfft::{prelude::*, dyn_size::realfft::DynRealDft};
use hound;
use dasp::{Sample, sample::FromSample};
use rand::Rng;

pub struct SparseVelvet {
    len: usize,
    // Vector of (pulse_location, pulse_type)
    // Pulse locations are monotonically increasing
    pulses: Vec<(usize, bool)>,
}

pub fn white_noise(len: usize) -> Vec<f32>
{
    let mut rng = rand::thread_rng();
    return (0..len).map(|_| rng.gen::<f32>()).collect();
}

pub fn convolve_velvet(signal: &Vec<f32>, velvet: &SparseVelvet) -> Vec<f32>
{
    println!("signal length: {} velvet length: {}", signal.len(), velvet.len);
    let mut output: Vec<f32> = vec![0.; signal.len() + velvet.len - 1];
    for n in 0..output.len() {
        for &(pulse_location, pulse_type) in velvet.pulses.iter() {
            let Some(index) = n.checked_sub(pulse_location) else {break;};
            if index >= signal.len() {
                continue;
            }
            match pulse_type {
                true =>  output[n] += signal[index],
                false => output[n] -= signal[index],
            };
        }
    }
    output
}

pub fn velvet_noise(velvet: &SparseVelvet) -> Vec<f32>
{
    let mut output: Vec<f32> = vec![0.0; velvet.len];
    for &(location, pulse_type) in velvet.pulses.iter() {
        output[location] = match pulse_type {
            true => 1.0,
            false => -1.0,
        }
    }
    output
}

pub fn gen_velvet(len: usize, density: u32, sample_rate: u32) -> SparseVelvet
{
    let mut rng = rand::thread_rng();
    let pulse_distance = sample_rate / density;
    let mut velvet = SparseVelvet {
        len: len,
        pulses: Vec::with_capacity(len / pulse_distance as usize),
    };
    //pulse locations: k(m) = round[mTd + r1(m)(Td − 1)]
    for m in 0..len / pulse_distance as usize {
        let location = (m * pulse_distance as usize) + (rng.gen::<f32>() * (pulse_distance - 1) as f32) as usize;
        velvet.pulses.push((location, rng.gen::<bool>()));
    }
    velvet
}

pub fn naive_convolve(s1: &Vec<f32>, s2: &Vec<f32>) -> Vec<f32>
{
    let (big, small) = if s1.len() > s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let mut output: Vec<f32> = vec![0.0; big.len() + small.len() - 1];
    for i in 0..output.len() {
        for j in 0..small.len() {
            let Some(x_index) = i.checked_sub(j) else { continue; };
            let Some(x) = big.get(x_index) else { continue; };
            let Some(h) = small.get(j) else { continue; };
            let product = x * h;
            output[i] += product;
        }
    }
    output
}

pub fn normalize(v: &Vec<f32>) -> Vec<f32> {
    let max = max_abs(&v);
    v.iter()
    .map(|e| e / max)
    .collect()
}

fn max_abs(v: &Vec<f32>) -> f32 {
    let mut result: f32 = 0.0;
    for e in v {
        result = result.max(e.abs());
    }
    result
}

pub fn convolve_fft(s1: &Vec<f32>, s2: &Vec<f32>) -> Vec<f32>
{
    let (big, small) = if s1.len() > s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let mut big = big.clone();
    let mut small = small.clone();
    let output_length = big.len() + small.len() - 1;
    big.resize(output_length, 0.0);
    small.resize(output_length, 0.0);
    let big_dft = &big[..].real_fft();
    let small_dft = &small[..].real_fft();
    let mult: DynRealDft<f32> = big_dft * small_dft;
    let output = mult.real_ifft();
    output.to_vec().iter().map(|&x| x / output_length as f32).collect()
}

pub fn read_wav<S>(filename: String) -> Result<Vec<S>, hound::Error>
where S: Sample + FromSample<i16>,
{
    let mut reader = hound::WavReader::open(filename)?;
    Ok(reader
        .samples()
        .map(|s: Result<i16, hound::Error>| s.unwrap().to_sample::<S>())
        .collect())
}

pub fn output_wav(filename: String, buffer: &Vec<i16>, sample_rate: u32) -> Result<(), hound::Error>
{
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec)?;

    for &sample in buffer.iter() {
        writer.write_sample(sample)?;
    }

    writer.finalize()
}

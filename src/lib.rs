use easyfft::{prelude::*, dyn_size::realfft::DynRealDft};
use hound;
use hound::Sample;
use rand::Rng;

const SAMPLE_RATE: u32 = 44_100;

// Vector of (location, pulse_type)
type SparseVelvet = Vec<(usize, bool)>;

pub fn white_noise(len: usize) -> Vec<f32>
{
    let mut rng = rand::thread_rng();
    return (0..len).map(|_| rng.gen::<f32>()).collect();
}

pub fn convolve_velvet(signal: &Vec<f32>, velvet_len: usize, density: u32, sample_rate: u32) -> Vec<f32>
{
    let velvet = gen_velvet(velvet_len, density, sample_rate);
    let mut output: Vec<f32> = vec![0.0; signal.len() + velvet_len - 1];
    for n in 0..output.len() {
        for &(location, pulse_type) in velvet.iter() {
            let location = match n.checked_sub(location) {
                Some(val) => val,
                None => break,
            };
            if location >= signal.len() {
                break;
            }
            match pulse_type {
                true =>  output[location] += signal[location],
                false => output[location] -= signal[location],
            };
        }
    }
    output
}

pub fn velvet_noise(len: usize, density: u32, sample_rate: u32) -> Vec<f32>
{
    let mut output: Vec<f32> = vec![0.0; len];
    let velvet = gen_velvet(len, density, sample_rate);
    for &(location, pulse_type) in velvet.iter() {
        output[location] = match pulse_type {
            true => 1.0,
            false => -1.0,
        }
    }
    output
}

fn gen_velvet(len: usize, density: u32, sample_rate: u32) -> SparseVelvet
{
    let mut rng = rand::thread_rng();
    let pulse_distance = sample_rate / density;
    let mut output: SparseVelvet = SparseVelvet::with_capacity(len / pulse_distance as usize);
    //pulse locations: k(m) = round[mTd + r1(m)(Td − 1)]
    for m in 0..len / pulse_distance as usize {
        let location = (m * pulse_distance as usize) + (rng.gen::<f32>() * (pulse_distance - 1) as f32) as usize;
        output.push( (location, rng.gen::<bool>()));
    }
    output
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
            let product = x * h;
            output[i] += product;
        }
    }
    output
}

pub fn fft_convolve(s1: &Vec<f32>, s2: &Vec<f32>) -> Vec<f32>
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
where S: Sample,
{
    let mut reader = hound::WavReader::open(filename)?;
    return Ok(reader
        .samples()
        .map(|s: Result<S, hound::Error>| s.unwrap())
        .collect());
}

pub fn output_wav(filename: String, buffer: &Vec<i16>) -> Result<(), hound::Error>
{
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec)?;

    for &sample in buffer.iter() {
        writer.write_sample(sample)?;
    }

    writer.finalize()
}

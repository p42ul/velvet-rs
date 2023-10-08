extern crate velvet_rs;

use dasp::Sample;

const SAMPLE_RATE: u32 = 44_100;

fn main() {
    let velvet = velvet_rs::gen_velvet((SAMPLE_RATE*7) as usize, 2205, SAMPLE_RATE);
    let noise = velvet_rs::velvet_noise(&velvet);
    let triangle = velvet_rs::read_wav::<f32>("triangle.wav".to_string()).unwrap();
    let fft_convolved = velvet_rs::convolve_fft(&noise, &triangle);
    let fft_convolved = velvet_rs::normalize(&fft_convolved);
    let fft_convolved = fft_convolved
        .iter()
        .map(|s| s.to_sample::<i16>())
        .collect();
    let _ = velvet_rs::output_wav("fft_convolved.wav".to_string(), &fft_convolved, SAMPLE_RATE);
    println!("output to fft_convolved.wav");
}

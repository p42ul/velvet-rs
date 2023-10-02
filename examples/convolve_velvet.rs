extern crate velvet_rs;

use dasp::Sample;

const SAMPLE_RATE: u32 = 44_100;

fn main() {
    let triangle = velvet_rs::read_wav::<i16>("triangle.wav".to_string()).unwrap()
        .iter()
        .map(|&s| s.to_sample::<f32>())
        .collect();
    let velvet = velvet_rs::gen_velvet((SAMPLE_RATE*7) as usize, 2205, SAMPLE_RATE);
    println!("running velvet noise convolution");
    let velvet_convolved = velvet_rs::convolve_velvet(&triangle, &velvet)
        .iter()
        .map(|&s| s.to_sample::<i16>())
        .collect();
    let _ = velvet_rs::output_wav("velvet_convolved.wav".to_string(), &velvet_convolved, SAMPLE_RATE);
    println!("output to velvet_convolved.wav");
}
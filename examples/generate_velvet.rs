extern crate velvet_rs;

use dasp::Sample;

const SAMPLE_RATE: u32 = 44_100;

fn main() {
    let noise = velvet_rs::velvet_noise(&velvet_rs::gen_velvet((SAMPLE_RATE * 5) as usize, 2205, SAMPLE_RATE));
    let noise = noise.iter().map(|&s| s.to_sample::<i16>()).collect();
    let _ = velvet_rs::output_wav("velvet_noise.wav".to_string(), &noise, SAMPLE_RATE);
    println!("output to velvet_noise.wav");
}

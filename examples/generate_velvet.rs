extern crate velvet_rs;

use dasp::Sample;

fn main() {
    let noise = velvet_rs::velvet_noise(&velvet_rs::gen_velvet(44_100 * 5, 2205, 44_100));
    let noise = noise.iter().map(|&s| s.to_sample::<i16>()).collect();
    let _ = velvet_rs::output_wav("velvet_noise.wav".to_string(), &noise);
    println!("output to velvet_noise.wav");
}

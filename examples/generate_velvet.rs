extern crate velvet_rs;

const SAMPLE_RATE: u32 = 44_100;

fn main() {
    let noise = velvet_rs::velvet_noise(&velvet_rs::gen_velvet((SAMPLE_RATE * 5) as usize, 2205, SAMPLE_RATE));
    let _ = velvet_rs::output_wav("velvet_noise.wav".to_string(), &noise, SAMPLE_RATE);
    println!("output to velvet_noise.wav");
}

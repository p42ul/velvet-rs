extern crate velvet_rs;

use dasp::Sample; 

const SAMPLE_RATE: u32 = 44_100;
fn main() {
    let noise = velvet_rs::white_noise((SAMPLE_RATE * 5) as usize);
    let noise = noise
        .iter()
        .map(|s| s.to_sample::<i16>())
        .collect();
    match velvet_rs::output_wav(String::from("white.wav"), &noise, SAMPLE_RATE) {
        Ok(_) => println!("created wav file!"),
        Err(e) => println!("error: {}", e),
    }
}

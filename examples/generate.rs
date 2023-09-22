extern crate velvet_rs;

use dasp::Sample; 

fn main() {
    let noise = velvet_rs::white_noise(44_100 * 5);
    let noise = noise
        .iter()
        .map(|s| s.to_sample::<i16>())
        .collect();
    match velvet_rs::output_wav(String::from("white.wav"), &noise) {
        Ok(_) => println!("created wav file!"),
        Err(e) => println!("error: {}", e),
    }
}

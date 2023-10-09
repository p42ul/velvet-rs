extern crate velvet_rs;

const SAMPLE_RATE: u32 = 44_100;
fn main() {
    let noise = velvet_rs::white_noise((SAMPLE_RATE * 5) as usize);
    match velvet_rs::output_wav(String::from("white.wav"), &noise, SAMPLE_RATE) {
        Ok(_) => println!("created wav file!"),
        Err(e) => println!("error: {}", e),
    }
}

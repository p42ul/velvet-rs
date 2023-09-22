extern crate velvet_rs;

fn main() {
    let noise = velvet_rs::white_noise(44_100 * 5);
    match velvet_rs::output_wav(String::from("white.wav"), &noise) {
        Ok(_) => println!("created wav file!"),
        Err(e) => println!("error: {}", e),
    }
}

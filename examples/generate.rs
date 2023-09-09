extern crate velvet;

fn main() {
    let noise = velvet::white_noise(44_100 * 5);
    match velvet::output_wav(String::from("white.wav"), &noise) {
        Ok(_) => println!("created wav file!"),
        Err(e) => println!("error: {}", e),
    }
}

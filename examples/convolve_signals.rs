extern crate velvet_rs;

fn main() {
    let noise = velvet_rs::white_noise(44_000 * 5);
    let triangle = velvet_rs::read_wav("triangle.wav".to_string()).unwrap();
    let fft_convolved = velvet_rs::fft_convolve(&noise, &triangle);
    let _ = velvet_rs::output_wav("fft_convolved.wav".to_string(), &fft_convolved);
    println!("output to fft_convolved.wav");
}

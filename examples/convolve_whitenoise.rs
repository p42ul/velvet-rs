extern crate velvet_rs;

use dasp::Sample;

fn main() {
    let noise = velvet_rs::white_noise(44_000 * 5);
    let triangle = velvet_rs::read_wav("triangle.wav".to_string()).unwrap();
    let fft_convolved = velvet_rs::fft_convolve(&noise, &triangle);
    let fft_convolved = fft_convolved
        .iter()
        .map(|s| s.to_sample::<i16>())
        .collect();
    let _ = velvet_rs::output_wav("fft_convolved.wav".to_string(), &fft_convolved);
    println!("output to fft_convolved.wav");
}

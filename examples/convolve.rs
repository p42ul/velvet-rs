extern crate velvet_rs;

fn main() {
    let noise = velvet_rs::white_noise::<f32>(44_000 * 5);
    let triangle = velvet_rs::read_wav("triangle.wav".to_string()).unwrap();
    let convolved = velvet_rs::naive_convolve(noise, triangle);
    let _ = velvet_rs::output_wav("convolved.wav".to_string(), &convolved);
}

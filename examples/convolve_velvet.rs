extern crate velvet_rs;

use dasp::Sample;

fn main() {
    let triangle = velvet_rs::read_wav::<i16>("triangle.wav".to_string()).unwrap();
    let sig_length = 44_100 * 5;
    let velvet = velvet_rs::gen_velvet(sig_length, 2205, 44_100);
    let convolved = velvet_rs::convolve_velvet(&triangle, &velvet);
    println!("converting from f32 to i16");
    let convolved = convolved
        .iter()
        .map(|&s| s.to_sample::<i16>())
        .collect();
    println!("outputting to wav");
    let _ = velvet_rs::output_wav("velvet_convolved.wav".to_string(), &convolved);
    println!("output to velvet_convolved.wav");
}

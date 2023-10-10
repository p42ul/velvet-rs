extern crate velvet_rs;

const SAMPLE_RATE: u32 = 44_100;

fn main() {
    let triangle = velvet_rs::read_wav::<f32>("triangle.wav".to_string()).unwrap();
    let velvet = velvet_rs::gen_velvet((SAMPLE_RATE*7) as usize, 2205, SAMPLE_RATE);
    println!("running velvet noise convolution");
    let velvet_convolved = velvet_rs::convolve_velvet_parallel(&triangle, &velvet);
    let velvet_convolved = velvet_rs::normalize(&velvet_convolved);
    let _ = velvet_rs::output_wav("velvet_convolved.wav".to_string(), &velvet_convolved, SAMPLE_RATE);
    println!("output to velvet_convolved.wav");
}
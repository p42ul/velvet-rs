#[macro_use]
extern crate timeit;
extern crate velvet_rs;

const SAMPLE_RATE: u32 = 44_100;

fn main() {
    println!("benchmarking convolution with 7 seconds of velvet noise");
    let triangle = velvet_rs::read_wav::<f32>("triangle.wav".to_string()).unwrap();
    let velvet = velvet_rs::gen_velvet((SAMPLE_RATE*7) as usize, 2205, SAMPLE_RATE);
    let noise = velvet_rs::velvet_noise(&velvet);
    println!("convolve_velvet");
    timeit!({
        let _ = velvet_rs::convolve_velvet(&triangle, &velvet);
    });
    println!("convolve_velvet_parallel");
    timeit!({
        let _ = velvet_rs::convolve_velvet_parallel(&triangle, &velvet);
    });
    println!("convolve_fft");
    timeit!({
        let _ = velvet_rs::convolve_fft(&noise, &triangle);
    });
}
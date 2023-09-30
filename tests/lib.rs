extern crate velvet_rs;

use dasp::Sample;

fn approx_eq(v1: &Vec<f32>, v2: &Vec<f32>, epsilon: f32) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    let mut ret = true;
    for i in 0..v1.len() {
        let delta = v1[i] - v2[i];
        if delta.abs() > epsilon {
            println!("x: {}, y: {}, delta: {}", v1[i], v2[i], delta);
            ret = false
        }
    }
    ret
}

#[ignore]
#[test]
fn signal_velvet_equivalence() {
    let s1 = velvet_rs::read_wav::<i16>("triangle.wav".to_string()).unwrap()
        .iter()
        .map(|&s| s.to_sample::<f32>())
        .collect();
    let velvet = velvet_rs::gen_velvet(44_100*7, 2205, 44_100);
    let s2 = velvet_rs::velvet_noise(&velvet);
    let fft_convolved = velvet_rs::convolve_fft(&s1, &s2);
    let velvet_convolved = velvet_rs::convolve_velvet(&s1, &velvet);
    assert_eq!(fft_convolved, velvet_convolved);
}

#[test]
fn naive_velvet_equivalence() {
    let s1: Vec<f32> = vec![1., 2., 3.,];
    let velvet = velvet_rs::gen_velvet(100, 10, 100);
    let s2 = velvet_rs::velvet_noise(&velvet);
    let naive_convolved = velvet_rs::naive_convolve(&s1, &s2);
    let velvet_convolved = velvet_rs::convolve_velvet(&s1, &velvet);
    assert_eq!(naive_convolved, velvet_convolved);
}

#[test]
fn naive_fft_equivalence() {
    let s1: Vec<f32> = vec![1., 2., 3.];
    let s2: Vec<f32> = vec![0., 0., 0., 1., 0., 0.];
    let expected: Vec<f32> = vec![0., 0., 0., 1., 2., 3., 0., 0.];
    let naive_convolved = velvet_rs::naive_convolve(&s1, &s2);
    let fft_convolved = velvet_rs::convolve_fft(&s1, &s2);
    assert_eq!(expected, naive_convolved);
    assert_eq!(expected, fft_convolved);
}

#[test]
fn fft_velvet_equivalence() {
    let s1: Vec<f32> = vec![1., 2., 3.,];
    let velvet = velvet_rs::gen_velvet(100, 10, 100);
    let fft_convolved = velvet_rs::convolve_fft(&s1, &velvet_rs::velvet_noise(&velvet));
    let velvet_convolved = velvet_rs::convolve_velvet(&s1, &velvet);
    assert!(approx_eq(&fft_convolved, &velvet_convolved, 1e-6));
}

#[test]
fn wav_io() {
    let output_filename = "triangle2.wav";
    let triangle: Vec<i16> = velvet_rs::read_wav::<i16>("triangle.wav".to_string()).unwrap();
    match velvet_rs::output_wav(output_filename.to_string(), &triangle) {
        Ok(_) => println!("created wav file {}", output_filename),
        Err(e) => println!("error: {}", e),
    }
    let triangle2: Vec<i16> = velvet_rs::read_wav("triangle2.wav".to_string()).unwrap();
    assert_eq!(triangle, triangle2);
}
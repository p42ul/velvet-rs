extern crate velvet_rs;

#[test]
fn convolve_simple() {
    let s1: Vec<f32> = vec![1., 2., 3.];
    let s2: Vec<f32> = vec![0., 0., 0., 1., 0., 0.];
    let expected: Vec<f32> = vec![0., 0., 0., 1., 2., 3., 0., 0.];
    let naive_convolved = velvet_rs::naive_convolve(&s1, &s2);
    let fft_convolved = velvet_rs::fft_convolve(&s1, &s2);
    assert_eq!(expected, fft_convolved);
    assert_eq!(expected, naive_convolved);
}

#[test]
fn duplicate_test() {
    let output_filename = "triangle2.wav";
    let triangle: Vec<i16> = velvet_rs::read_wav::<i16>("triangle.wav".to_string()).unwrap();
    match velvet_rs::output_wav(output_filename.to_string(), &triangle) {
        Ok(_) => println!("created wav file {}", output_filename),
        Err(e) => println!("error: {}", e),
    }
    let triangle2: Vec<i16> = velvet_rs::read_wav("triangle2.wav".to_string()).unwrap();
    assert_eq!(triangle, triangle2);
}
extern crate velvet_rs;

fn main() {
    let s1: Vec<f32> = vec![1., 2., 3.];
    let s2: Vec<f32> = vec![0.,0.,0.,1.,0.,0.];
    let fft_convolved = velvet_rs::fft_convolve(s1.clone(), s2.clone());
    let naive_convolved = velvet_rs::naive_convolve(&s1, &s2);
    println!("fft_convolved: {:?}", fft_convolved);
    println!("naive_convolved: {:?}", naive_convolved);
}

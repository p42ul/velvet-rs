extern crate velvet_rs;

fn main() {
    let output_filename = "triangle2.wav";
    let triangle: Vec<f32> = velvet_rs::read_wav::<f32>("triangle.wav".to_string()).unwrap();
    match velvet_rs::output_wav(output_filename.to_string(), &triangle) {
        Ok(_) => println!("created wav file {}", output_filename),
        Err(e) => println!("error: {}", e),
    }
}

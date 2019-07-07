pub mod perlin_noise_generator;

pub trait NoiseGenerator {
    fn new(octaves: u8, persistence: f64, seed: u32) -> Self;

    fn generate(&self, x: f64, y: f64) -> f64;
}
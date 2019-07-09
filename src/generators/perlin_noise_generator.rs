use crate::generators::NoiseGenerator;
use noise::{Perlin, NoiseFn, Seedable};

pub struct PerlinNG {

    _octaves: u8,
    _seed: u32,
    _persistence: f64,
}

impl NoiseGenerator for PerlinNG {

    fn new(octaves: u8, persistance: f64,  seed: u32) -> Self {

        PerlinNG {
            _octaves: octaves,
            _seed: seed,
            _persistence: persistance
        }
    }

    fn generate(&self, x: f64, y: f64) -> f64 {
        let mut perlin = Perlin::new();
        perlin = perlin.set_seed(self._seed);
        let mut max_value = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut noise_value = 0.0;

        for _octave in 0..self._octaves {
            let new_noise_value = perlin.get([x* frequency, y * frequency]) * amplitude;
//            println!("N {}, MAX {}, AM {}, FR {}", noise_value, max_value, amplitude, frequency);
            noise_value += new_noise_value;
            max_value += amplitude;
            amplitude *= self._persistence as f64;
            frequency *= 2.0;
        }

        (noise_value / max_value + 1.0 ) *50.0
    }

}
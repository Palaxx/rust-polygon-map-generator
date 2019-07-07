use image::jpeg::JPEGEncoder;
use std::net::Shutdown::Write;
use std::io::BufWriter;
use image::{RgbImage, ImageBuffer};
use map_generator::generators::perlin_noise_generator::PerlinNG;
use map_generator::generators::NoiseGenerator;
use map_generator::terrains::factory::Factory;
use map_generator::terrains::TerrainFactory;

extern crate map_generator;

fn main() {
    let size: u8 = 255;
    let octaves = 6;
    let mut noise = Vec::new();
    let perlin_generator: PerlinNG = PerlinNG::new(octaves, 0.5, 15);
    for x in 0..size {
        for y in 0..size {
            let pos_x = x as f64 / size as f64;
            let pos_y = y as f64 / size as f64 ;

            let noise_value = perlin_generator.generate(pos_x, pos_y);
            noise.push(noise_value as u64) ;
        }
    }
    println!("{:?}", noise);
    print_image(size as u32, noise, octaves);
}

fn print_image(size: u32, noise: Vec<u64>, octaves: u8) {
    let img: RgbImage = ImageBuffer::new(size, size);
    let terrain_factory: Factory = Factory {};
// Construct a new by repeated calls to the supplied closure.
    let mut img = ImageBuffer::from_fn(size , size , |x, y| {
//        if x % 2 == 0 {
//            image::Luma([0u8])
//        } else {
//            image::Luma([255u8])
//        }
        let index: u64 = (size * y + x) as u64;
        let noise: u64 = noise[index as usize];
        let rgb = terrain_factory.make_from_elevation(noise);
        image::Rgb(rgb.get_color())
    });
    let number = octaves.to_owned();
    img.save(format!("test{}.png", number)).unwrap();
}

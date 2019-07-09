#[macro_use] extern crate structopt;
extern crate map_generator;

use structopt::StructOpt;
use image::{RgbImage, ImageBuffer};
use map_generator::generators::perlin_noise_generator::PerlinNG;
use map_generator::generators::NoiseGenerator;
use map_generator::terrains::factory::Factory;
use map_generator::terrains::TerrainFactory;
use map_generator::options::Options;

fn main() {
    let opt: Options = Options::from_args();
    let size = opt.get_length();
    let octaves = opt.get_octaves();
    let seed = opt.get_seed();
    let mut noise = Vec::new();
    let elevation_generator: PerlinNG = PerlinNG::new(octaves, 0.5, seed);
    let moisture_generator: PerlinNG = PerlinNG::new(octaves, 0.5, seed + 50);

    for x in 0..size {
        for y in 0..size {
            let pos_x = x as f64 / size as f64;
            let pos_y = y as f64 / size as f64 ;

            let elevation_value = elevation_generator.generate(pos_x, pos_y);
            let moisture_value = moisture_generator.generate(pos_x, pos_y);

//            if elevation_value < 20.0 {
//                println!("{},{}", elevation_value, moisture_value)
//            }
            noise.push((elevation_value as u64, moisture_value as u64)) ;
        }
    }
//    println!("{:?}", noise);
    print_image(size as u32, noise, octaves);
}

fn print_image(size: u32, noise: Vec<(u64, u64)>, octaves: u8) {
    let img: RgbImage = ImageBuffer::new(size, size);
    let terrain_factory: Factory = Factory {};
    let mut img = ImageBuffer::from_fn(size * 10 , size * 10, |x, y| {
        let index: u64 = (size  * (y / 10) + x / 10) as u64;
        let elevation: u64 = noise[index as usize].0;
        let moisture: u64 = noise[index as usize].1;
        let rgb = terrain_factory.make_from_elevation_and_moisture(elevation, moisture);
        image::Rgb(rgb.get_color())
    });
    let number = octaves.to_owned();
    img.save(format!("test{}.png", number)).unwrap();
}

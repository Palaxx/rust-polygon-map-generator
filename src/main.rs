#[macro_use] extern crate structopt;
extern crate map_generator;

use structopt::StructOpt;
use image::{RgbImage, ImageBuffer};
use map_generator::generators::perlin_noise_generator::PerlinNG;
use map_generator::generators::noise_generator_trait::NoiseGenerator;
use map_generator::terrains::factory::Factory;
use map_generator::terrains::TerrainFactory;
use map_generator::options::Options;
use map_generator::terrains::terrain::Terrain;
use map_generator::drawers::drawer_trait::Drawer;
use map_generator::drawers::color_drawer::ColorDrawer;

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

            noise.push((elevation_value as u64, moisture_value as u64)) ;
        }
    }
    let terrains = build_terrains(noise);
    print_image(seed, size as u32, terrains);
}

fn build_terrains(noise: Vec<(u64, u64)>) -> Vec<Box<Terrain>> {
    let terrain_factory: Factory = Factory {};
    let mut terrains: Vec<Box<Terrain>> = Vec::new();
    for n in &noise {
        let elevation: u64 = n.0;
        let moisture: u64 = n.1;
        let terrain = terrain_factory.make_from_elevation_and_moisture(elevation, moisture);
        terrains.push(terrain);
    };
    terrains
}

fn print_image(seed: u32, size: u32, terrains: Vec<Box<Terrain>>) {
    let color_drawer: ColorDrawer = ColorDrawer {};
    let filename: String = format!("seed_{}.png", seed.to_owned());
    color_drawer.draw(filename, size, terrains);
}

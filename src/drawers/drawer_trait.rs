use crate::terrains::terrain::Terrain;

pub trait Drawer {
    fn draw(&self, filename: String, size: u32, terrains: Vec<Box<Terrain>>) -> String;
}
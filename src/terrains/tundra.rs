use crate::terrains::terrain::Terrain;

pub struct Tundra {

}

impl Terrain for Tundra {
    fn get_color(&self) -> [u8; 3] {
        [187, 187, 170]
    }

    fn get_name(&self) -> &'static str {
        "tundra"
    }
}
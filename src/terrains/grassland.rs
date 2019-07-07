use crate::terrains::terrain::Terrain;

pub struct Grassland {

}

impl Terrain for Grassland {
    fn get_color(&self) -> [u8; 3] {
        [136,170,85]
    }

    fn get_name(&self) -> &'static str {
        "grassland"
    }
}
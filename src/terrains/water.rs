use crate::terrains::terrain::Terrain;

pub struct Water {

}

impl Terrain for Water {
    fn get_color(&self) -> [u8; 3] {
        [0, 0, 128git]
    }

    fn get_name(&self) -> &'static str {
        "water"
    }
}
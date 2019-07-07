use crate::terrains::terrain::Terrain;

pub struct Snow {

}

impl Terrain for Snow {
    fn get_color(&self) -> [u8; 3] {
        [221,221,228]
    }

    fn get_name(&self) -> &'static str {
        "snow"
    }
}
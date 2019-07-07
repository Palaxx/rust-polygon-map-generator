use crate::terrains::terrain::Terrain;

pub struct BareEarth {

}

impl Terrain for BareEarth {
    fn get_color(&self) -> [u8; 3] {
        [136, 136, 136]
    }

    fn get_name(&self) -> &'static str {
        "bare-earth"
    }
}
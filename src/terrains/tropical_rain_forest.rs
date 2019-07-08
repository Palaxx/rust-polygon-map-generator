use crate::terrains::terrain::Terrain;

pub struct TropicalRainForest {

}

impl Terrain for TropicalRainForest {
    fn get_color(&self) -> [u8; 3] {
        [51,119,85]
    }

    fn get_name(&self) -> &'static str {
        "tropical-rain-forest"
    }
}
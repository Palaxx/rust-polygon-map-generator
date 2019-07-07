use crate::terrains::terrain::Terrain;

pub struct TemperateRainForest {

}

impl Terrain for TemperateRainForest {
    fn get_color(&self) -> [u8; 3] {
        [68,136,85]
    }

    fn get_name(&self) -> &'static str {
        "temperate-rain-forest"
    }
}
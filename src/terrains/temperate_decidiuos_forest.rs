use crate::terrains::terrain::Terrain;

pub struct TemperateDecidiousForest {

}

impl Terrain for TemperateDecidiousForest {
    fn get_color(&self) -> [u8; 3] {
        [103, 148, 109]
    }

    fn get_name(&self) -> &'static str {
        "temperate-decidous-forest"
    }
}
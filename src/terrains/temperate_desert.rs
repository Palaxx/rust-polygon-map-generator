use crate::terrains::terrain::Terrain;

pub struct TemperateDesert {

}

impl Terrain for TemperateDesert {
    fn get_color(&self) -> [u8; 3] {
        [201, 210, 155]
    }

    fn get_name(&self) -> &'static str {
        "temperate-desert"
    }
}
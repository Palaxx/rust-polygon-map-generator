use crate::terrains::terrain::Terrain;

pub struct SubtropicalDesert {

}

impl Terrain for SubtropicalDesert {
    fn get_color(&self) -> [u8; 3] {
        [210, 185, 139]
    }

    fn get_name(&self) -> &'static str {
        "subtropical-desert"
    }
}
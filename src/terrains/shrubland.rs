use crate::terrains::terrain::Terrain;

pub struct Shrubland {

}

impl Terrain for Shrubland {
    fn get_color(&self) -> [u8; 3] {
        [136, 153, 119]
    }

    fn get_name(&self) -> &'static str {
        "shrubland"
    }
}
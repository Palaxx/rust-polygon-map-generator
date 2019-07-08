use crate::terrains::terrain::Terrain;

pub struct Taiga {

}

impl Terrain for Taiga {
    fn get_color(&self) -> [u8; 3] {
        [153, 170, 119]
    }

    fn get_name(&self) -> &'static str {
        "taiga"
    }
}
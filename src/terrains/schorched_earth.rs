use crate::terrains::terrain::Terrain;

pub struct SchorchedEarth {

}

impl Terrain for SchorchedEarth {
    fn get_color(&self) -> [u8; 3] {
        [85,85,85]
    }

    fn get_name(&self) -> &'static str {
        "schorched-earth"
    }
}
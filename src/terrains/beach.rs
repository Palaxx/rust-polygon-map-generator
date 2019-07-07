use crate::terrains::terrain::Terrain;

pub struct Beach {

}

impl Terrain for Beach {
    fn get_color(&self) -> [u8; 3] {
        [244, 164, 96]
    }

    fn get_name(&self) -> &'static str {
        "sand"
    }
}
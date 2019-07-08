use crate::terrains::terrain::Terrain;

pub struct TropicalSeasonalForest {

}

impl Terrain for TropicalSeasonalForest {
    fn get_color(&self) -> [u8; 3] {
        [85,153,68]
    }

    fn get_name(&self) -> &'static str {
        "tropical-seasonal-forest"
    }
}
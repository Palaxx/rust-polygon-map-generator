use crate::terrains::terrain::Terrain;
use crate::terrains::bare_earth::BareEarth;
use crate::terrains::beach::Beach;
use crate::terrains::TerrainFactory;
use crate::terrains::grassland::Grassland;
use crate::terrains::snow::Snow;
use crate::terrains::temperate_rain_forest::TemperateRainForest;
use crate::terrains::temperate_decidiuos_forest::TemperateDecidiousForest;
use crate::terrains::water::Water;
use crate::terrains::temperate_desert::TemperateDesert;

pub struct Factory {


}

impl TerrainFactory for Factory {

    fn make_from_elevation(&self, elevation: u64) -> Box<Terrain> {
        if elevation < 36 {
            Box::new(Water {})
        } else if elevation < 72 {
            Box::new(Beach {})
        } else if elevation < 108 {
            Box::new(Grassland {})
        } else if elevation < 144 {
            Box::new(TemperateRainForest {})
        } else if elevation < 180 {
            Box::new(TemperateDecidiousForest {})
        } else if elevation < 216 {
            Box::new(TemperateDesert {})
        } else {
            Box::new(Snow {})
        }
    }

    fn make_from_elevation_and_moisture(&self, elevation: u64, moisture: u64) -> Box<Terrain> {
        Box::new(Beach {})
    }
}
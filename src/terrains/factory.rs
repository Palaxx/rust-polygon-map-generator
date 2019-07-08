use crate::terrains::terrain::Terrain;
use crate::terrains::bare_earth::BareEarth;
use crate::terrains::beach::Beach;
use crate::terrains::TerrainFactory;
use crate::terrains::grassland::Grassland;
use crate::terrains::snow::Snow;
use crate::terrains::schorched_earth::SchorchedEarth;
use crate::terrains::temperate_rain_forest::TemperateRainForest;
use crate::terrains::temperate_decidiuos_forest::TemperateDecidiousForest;
use crate::terrains::water::Water;
use crate::terrains::shrubland::Shrubland;
use crate::terrains::tundra::Tundra;
use crate::terrains::taiga::Taiga;
use crate::terrains::subtropicaldesert::SubtropicalDesert;
use crate::terrains::tropical_seasonal_forest::TropicalSeasonalForest;
use crate::terrains::tropical_rain_forest::TropicalRainForest;
use crate::terrains::temperate_desert::TemperateDesert;

pub struct Factory {


}

impl TerrainFactory for Factory {

    fn make_from_elevation(&self, elevation: u64) -> Box<Terrain> {
        if elevation < 10 {
            Box::new(Water {})
        } else if elevation < 20 {
            Box::new(Beach {})
        } else if elevation < 30 {
            Box::new(Grassland {})
        } else if elevation < 50 {
            Box::new(TemperateRainForest {})
        } else if elevation < 70 {
            Box::new(TemperateDecidiousForest {})
        } else if elevation < 90 {
            Box::new(SubtropicalDesert {})
        } else {
            Box::new(Snow {})
        }
    }

    fn make_from_elevation_and_moisture(&self, elevation: u64, moisture: u64) -> Box<Terrain> {
        if elevation < 10 {
            Box::new(Water {})
        } else if elevation < 12 {
            Box::new(Beach {})
        } else if elevation > 80 {
            if moisture < 10 {
                Box::new(SchorchedEarth {})
            } else if moisture < 20 {
                Box::new(BareEarth {})
            } else if moisture < 50 {
                Box::new(Tundra {})
            } else {
                Box::new(Snow {})
            }
        } else if elevation > 60 {
            if moisture < 33 {
                Box::new(TemperateDesert {})
            } else if moisture < 66 {
                Box::new(Shrubland {})
            } else {
                Box::new(Taiga {})
            }
        } else if elevation > 30 {
            if moisture < 16 {
                Box::new(TemperateDesert {})
            } else if moisture < 50 {
                Box::new(Grassland {})
            } else if moisture < 83 {
                Box::new(TemperateDecidiousForest {})
            } else {
                Box::new(TemperateRainForest {})
            }
        } else {
            if moisture < 16 {
                Box::new(SubtropicalDesert {})
            } else if moisture < 33 {
                Box::new(Grassland {})
            } else if moisture < 66 {
                Box::new(TropicalSeasonalForest {})
            } else {
                Box::new(TropicalRainForest {})

            }
        }
    }

}
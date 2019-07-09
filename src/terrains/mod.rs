use crate::terrains::terrain::Terrain;

mod bare_earth;
mod beach;
pub mod factory;
mod grassland;
mod schorched_earth;
mod shrubland;
mod snow;
mod subtropicaldesert;
mod taiga;
mod temperate_decidiuos_forest;
mod temperate_desert;
mod temperate_rain_forest;
pub mod terrain;
mod tropical_rain_forest;
mod tropical_seasonal_forest;
mod tundra;
mod water;

pub trait TerrainFactory {
    fn make_from_elevation(&self, elevation: u64) -> Box<Terrain>;

    fn make_from_elevation_and_moisture(&self, elevation: u64, moisture: u64) -> Box<Terrain>;
}


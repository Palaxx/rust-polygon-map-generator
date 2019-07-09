#[derive(Debug, StructOpt)]
#[structopt(name = "map-generator", about = "Map Generator Customizable parameters", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]

/// Map Generator Customizable parameters
pub struct Options {

    /// Define the seed for the map generation process
    #[structopt(short = "s", long = "seed", default_value="42")]
    seed: u32,

    /// Define how to map should be smoothed
    #[structopt(short = "o", long = "octaves", default_value="6")]
    octaves: u8,

    /// Define the map side length
    #[structopt(short = "l", long = "length", default_value="100")]
    length: u32
}

impl Options {

    pub fn get_seed(&self) -> u32 {
        self.seed
    }

    pub fn get_octaves(&self) -> u8 {
        self.octaves
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }
}
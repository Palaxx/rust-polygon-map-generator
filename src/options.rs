#[derive(Debug, StructOpt)]
#[structopt(name = "map-generator", about = "An example")]
/// Qualcosa
pub struct Options {

    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
}
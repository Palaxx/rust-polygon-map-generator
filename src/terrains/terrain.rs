
pub trait Terrain {

    fn get_color(&self) -> [u8; 3];

    fn get_name(&self) -> &'static str;
}
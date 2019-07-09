pub trait Drawer {
    fn draw(terrains: Vec<Box<Terrain>>) -> &str;
}
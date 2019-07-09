use image::ImageBuffer;
use crate::terrains::terrain::Terrain;
use crate::drawers::drawer_trait::Drawer;

pub struct ColorDrawer {

}


impl Drawer for ColorDrawer {

    fn draw(&self, filename: String, size: u32, terrains: Vec<Box<Terrain>>) -> String{
        let mut img = ImageBuffer::from_fn(size * 10 , size * 10, |x, y| {
            let index: usize = (size  * (y / 10) + x / 10) as usize;
            let terrain = &terrains[index];
            image::Rgb(terrain.get_color())
        });
        let path = filename;
        img.save(path.clone()).expect("Impossibile salvare immagine con path");

        path
    }

}
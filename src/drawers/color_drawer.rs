use image::ImageBuffer;

struct ColorDrawer {

}

impl Drawer for ColorDrawer {

    fn draw(terrains: Vec<Box<Terrain>>) -> &str {
        let mut img = ImageBuffer::from_fn(size * 10 , size * 10, |x, y| {
            let index: u64 = (size  * (y / 10) + x / 10) as u64;
            let terrain = terrains[index];
            image::Rgb(terrain.get_color())
        });
        let number = 100;
        img.save(format!("test{}.png", number)).unwrap();
    }

}
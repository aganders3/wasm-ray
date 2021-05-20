use crate::ray::Color;

pub struct Image {
    pub image_height: usize,
    pub image_width: usize,
    pub image: Vec<u8>,
}

impl Image {
    pub fn new(image_height: usize, image_width: usize) -> Image {
        Image {
            image_height,
            image_width,
            image: vec![0; 3 * image_height * image_width],
        }
    }

    pub fn write_color(&mut self, i: usize, j: usize, color: Color) {
        self.image[3 * self.image_width * j + 3 * i + 0] = (color.r * 255.) as u8;
        self.image[3 * self.image_width * j + 3 * i + 1] = (color.g * 255.) as u8;
        self.image[3 * self.image_width * j + 3 * i + 2] = (color.b * 255.) as u8;
    }
}

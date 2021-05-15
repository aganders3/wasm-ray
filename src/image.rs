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
            image: vec![0; 4 * image_height * image_width],
        }
    }

    pub fn write_color(&mut self, i: usize, j: usize, color: Color) {
        self.image[4 * self.image_width * j + 4 * i + 0] = color.r;
        self.image[4 * self.image_width * j + 4 * i + 1] = color.g;
        self.image[4 * self.image_width * j + 4 * i + 2] = color.b;
        self.image[4 * self.image_width * j + 4 * i + 3] = color.a;
    }
}

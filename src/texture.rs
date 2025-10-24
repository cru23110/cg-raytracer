use crate::vector::Color;
use image::RgbImage;

#[derive(Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Vec<Color>>,
}

impl Texture {
    pub fn from_image(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();

        let mut data = vec![vec![Color::zero(); width as usize]; height as usize];

        for y in 0..height {
            for x in 0..width {
                let pixel = rgb_img.get_pixel(x, y);
                data[y as usize][x as usize] = Color::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                );
            }
        }

        Ok(Texture {
            width,
            height,
            data,
        })
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let x = ((u * self.width as f32) as u32).min(self.width - 1);
        let y = ((v * self.height as f32) as u32).min(self.height - 1);

        self.data[y as usize][x as usize]
    }
}

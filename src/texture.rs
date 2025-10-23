use crate::vector::Color;
use image::{ImageBuffer, Rgb, RgbImage};

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

pub fn create_minecraft_grass_top() -> RgbImage {
    let size = 16;
    let mut img = ImageBuffer::new(size, size);

    let base_green = Rgb([106, 170, 64]);
    let dark_green = Rgb([76, 140, 34]);

    for y in 0..size {
        for x in 0..size {
            let pattern = (x + y) % 4;
            let color = if pattern == 0 || pattern == 3 {
                dark_green
            } else {
                base_green
            };
            img.put_pixel(x, y, color);
        }
    }

    img
}

pub fn create_minecraft_grass_side() -> RgbImage {
    let size = 16;
    let mut img = ImageBuffer::new(size, size);

    let dirt_color = Rgb([134, 96, 67]);
    let grass_green = Rgb([106, 170, 64]);
    let dark_green = Rgb([76, 140, 34]);

    for y in 0..size {
        for x in 0..size {
            if y < 3 {
                let pattern = (x + y) % 3;
                let color = if pattern == 0 {
                    dark_green
                } else {
                    grass_green
                };
                img.put_pixel(x, y, color);
            } else {
                img.put_pixel(x, y, dirt_color);
            }
        }
    }

    img
}

pub fn create_minecraft_dirt() -> RgbImage {
    let size = 16;
    let mut img = ImageBuffer::new(size, size);

    let dirt_base = Rgb([134, 96, 67]);
    let dirt_dark = Rgb([114, 76, 47]);

    for y in 0..size {
        for x in 0..size {
            let pattern = (x * 3 + y * 5) % 7;
            let color = if pattern < 2 {
                dirt_dark
            } else {
                dirt_base
            };
            img.put_pixel(x, y, color);
        }
    }

    img
}

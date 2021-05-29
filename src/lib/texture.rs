use crate::perlin::Perlin;
use crate::vec::Vec3;
use stb_image::image::{Image, LoadResult};
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color_value: Vec3,
}

impl SolidColor {
    pub fn new_color(color_value: Vec3) -> Self {
        SolidColor { color_value }
    }
    pub fn new_rgb(r: f64, b: f64, g: f64) -> Self {
        SolidColor {
            color_value: Vec3::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new_texture(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture { even, odd }
    }

    pub fn new_color(color1: Vec3, color2: Vec3) -> CheckerTexture {
        CheckerTexture {
            even: Arc::new(SolidColor::new_color(color1)),
            odd: Arc::new(SolidColor::new_color(color2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}
impl NoiseTexture {
    pub fn new() -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }
    pub fn new_scaled(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (p.z * self.scale + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    depth: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    //const BYTES_PER_PIXEL: usize = 3;

    pub fn new() -> Self {
        ImageTexture {
            data: Vec::new(),
            width: 0,
            height: 0,
            depth: 0,
            bytes_per_scanline: 0,
        }
    }

    pub fn new_from_file(filename: &str) -> Self {
        //let components_per_pixel = ImageTexture::BYTES_PER_PIXEL;

        let image = if let LoadResult::ImageU8(image) = stb_image::image::load(filename) {
            eprintln!(
                "Loaded {}, width: {}, height: {}, depth: {}, ",
                filename, image.width, image.height, image.depth
            );
            image
        } else {
            eprintln!("Unable to load image file");
            Image::new(1, 1, 1, vec![255, 0, 0])
        };

        ImageTexture {
            data: image.data,
            width: image.width,
            height: image.height,
            depth: image.depth,
            bytes_per_scanline: image.depth * image.width,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        if self.data.is_empty() {
            return Vec3::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.width as f64) as usize;
        let j = (v * self.height as f64) as usize;

        let color_scale = 1.0 / 255.0;

        let pixel_start = (j * self.bytes_per_scanline + i * self.depth) as usize;
        Vec3::new(
            color_scale * self.data[pixel_start] as f64,
            color_scale * self.data[pixel_start + 1] as f64,
            color_scale * self.data[pixel_start + 2] as f64,
        )
    }
}

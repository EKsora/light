extern crate image;
use crate::vec3::Vec3;
use crate::perlin::*;
use std::sync::Arc;
use std::convert::TryInto;
use image::*;
pub use image::{ImageBuffer, RgbImage};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct SolidColor {
    color_value: Vec3,
}
impl SolidColor {
    pub fn new(color_value: Vec3) -> Self {
        SolidColor { color_value }
    }
    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Vec3 {
        Vec3::new(red, green, blue)
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color_value.clone()
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
    pub fn new_rgb(even: Vec3, odd: Vec3) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(odd.clone())),
            even: Arc::new(SolidColor::new(even.clone())),
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (p.clone().x * 10.0).sin() * (p.clone().y * 10.0).sin() * (p.clone().z * 10.0).sin();
        if sines < 0.0 {
            self.odd.value(u, v, &p.clone())
        } else {
            self.even.value(u, v, &p.clone())
        }
    }
}

pub struct NoiseTexture {
    scale: f64,
    noise: Perlin,
}
impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            scale: scale,
            noise: Perlin::new(),
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        let n = 0.5*(1.0+(self.scale*p.z+10.0*self.noise.turb(p,7)).sin());
        Vec3::new(n,n,n)
    }
}
pub struct ImageTexture {
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: usize,
    height: usize,
}
impl Default for ImageTexture {
    fn default() -> Self {
        Self {
            data: ImageBuffer::new(0, 0),
            width: 0,
            height: 0,
        }
    }
}
impl ImageTexture {
    #[allow(dead_code)]
    pub fn new(filename: &str) -> Self {
        let data = open(filename).unwrap().into_rgb();
        let width = data.width() as usize;
        let height = data.height() as usize;
        Self {
            data,
            width,
            height,
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Vec3) -> Vec3 {
        u=clamp(u,0.0,1.0);
        v=1.0-clamp(v,0.0,1.0);
        let i = (u * self.width as f64).floor() as usize;
        let j = (v * self.height as f64).floor() as usize;
        if i < self.width && j < self.height {
            let pixel = self.data.get_pixel(i.try_into().unwrap(), j.try_into().unwrap()).to_rgb();
            Vec3::new(pixel[0] as f64 /255.0,pixel[1] as f64 /255.0,pixel[2] as f64 /255.0)
        } else {
            Vec3::new(1.0,1.0,1.0)
        }
    }
}
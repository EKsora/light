extern crate image;
use crate::vec3::Vec3;
use crate::perlin::*;
use std::sync::Arc;
use image::*;

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
        //let n =0.5*(1.0+self.noise.noise(&((*p).clone()*self.scale)));
        let n = self.noise.turb(&(*p * self.scale), 7);
        Vec3::new(n,n,n)
    }
}
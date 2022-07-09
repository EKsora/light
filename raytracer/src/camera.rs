use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) {
    degrees*PI/180.0;
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta: f64 = vfov*PI/180.0;
        let h: f64 = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        Self {
            origin: Vec3::zero(),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
            lower_left_corner: Vec3::new(-viewport_width / 2.0,-viewport_height / 2.0,-focal_length),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin.clone(),
            dir: self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v - self.origin.clone(),
        }
    }
}
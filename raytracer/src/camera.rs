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
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta: f64 = vfov*PI/180.0;
        let h: f64 = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = Vec3::unit(&(look_from.clone() - look_at.clone()));
        let x = Vec3::unit(&Vec3::cross(vup.clone(), w.clone()));
        let y = Vec3::cross(w.clone(), x.clone());
        Self {
            origin: look_from.clone(),
            horizontal: x.clone() * viewport_width,
            vertical: y.clone() * viewport_height,
            lower_left_corner: look_from - x.clone() * viewport_width / 2.0 - y.clone() * viewport_height / 2.0 - w,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin.clone(),
            dir: self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v - self.origin.clone(),
        }
    }
}
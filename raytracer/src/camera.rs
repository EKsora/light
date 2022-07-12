use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) {
    degrees*PI/180.0;
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(-1.0+(1.0+1.0)*rand::random::<f64>(),-1.0+(1.0+1.0)*rand::random::<f64>(),0.0);
        if p.squared_length() >= 1.0{
            continue;
        }
        return p;
    }
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}


pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u:Vec3,
    v:Vec3,
    w:Vec3,
    lens_radius:f64,
    time0: f64,
    time1: f64,
}
impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64,aperture:f64,focus_dist:f64,itime0: f64,itime1: f64) -> Self {
        let theta: f64 = vfov*PI/180.0;
        let h: f64 = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let iw = Vec3::unit(&(look_from.clone() - look_at.clone()));
        let iu = Vec3::unit(&Vec3::cross(vup.clone(), iw.clone()));
        let iv = Vec3::cross(iw.clone(), iu.clone());
        let ivertical= iv.clone() * viewport_height*focus_dist;
        let ihorizontal= iu.clone() * viewport_width*focus_dist;
        let iorigin= look_from.clone();
        Self {
            w : Vec3::unit(&(look_from.clone() - look_at.clone())),
            u : Vec3::unit(&Vec3::cross(vup.clone(), iw.clone())),
            v : Vec3::cross(iw.clone(), iu.clone()),
            origin: look_from.clone(),
            horizontal: iu.clone() * viewport_width*focus_dist,
            vertical: iv.clone() * viewport_height*focus_dist,
            lower_left_corner: iorigin - ihorizontal/2.0 - ivertical/2.0 - iw.clone()*focus_dist,
            lens_radius : aperture / 2.0,
            time0: itime0,
            time1: itime1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk()*self.lens_radius;
        let offset = self.u.clone() * rd.x + self.v.clone() * rd.y;
        Ray {
            orig: self.origin.clone()+offset.clone(),
            dir: self.lower_left_corner.clone() + self.horizontal.clone() * s + self.vertical.clone() * t - self.origin.clone()-offset.clone(),
            time: random_double_in_range(self.time0, self.time1),
        }
    }
}
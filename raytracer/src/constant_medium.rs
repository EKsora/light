extern crate rand;
use crate::hit::*;
use crate::aabb::AABB;
use crate::material::*;
use crate::texture::Texture;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::E;
use std::f64::INFINITY;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}
impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, color: Vec3) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new(color)),
            neg_inv_density: -1.0/density,
        }
    }
    pub fn new_texture(boundary: Arc<dyn Hittable>,density: f64,texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new_texture(texture)),
            neg_inv_density: -1. / density,
        }
    }
}
impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut rec1: HitRecord =  HitRecord::new(Arc::new(Lambertian::new(Vec3::zero())));
        let mut rec2: HitRecord =  HitRecord::new(Arc::new(Lambertian::new(Vec3::zero())));
        if !self.boundary.hit(ray, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }
        if !self.boundary.hit(ray, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }
        if rec1.t<t_min {rec1.t=t_min};
        if rec2.t>t_max {rec2.t=t_max};
        if rec1.t>=rec2.t {return false;};
        if rec1.t<0.0 {rec1.t=0.0;}
        let ray_length:f64 = ray.direction().length();
        let distance_inside_boundary:f64 = (rec2.t - rec1.t) * ray_length;
        let hit_distance: f64 = self.neg_inv_density * rand::random::<f64>().log(E);
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t=rec1.t+hit_distance/ray_length;
        rec.p=ray.at(rec.t);
        rec.normal=Vec3::new(1., 0., 0.); 
        rec.front_face=true;
        rec.material=self.phase_function.clone();
        true
    }
}
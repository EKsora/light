use crate::aabb::*;
use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct XYRectangle {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Arc<dyn Material>,
}
impl XYRectangle {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {x0,x1,y0,y1,k,material,}
    }
}
impl Hittable for XYRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t=(self.k-ray.origin().z)/ray.direction().z;
        if t<t_min||t>t_max{
            return false;
        }
        let x=ray.origin().x+t*ray.direction().x;
        let y=ray.origin().y+t*ray.direction().y;
        if x<self.x0||x>self.x1||y<self.y0||y>self.y1{
            return false;
        }
        rec.u=(x-self.x0)/(self.x1-self.x0);
        rec.v=(y-self.y0)/(self.y1-self.y0);
        rec.t=t;
        let outward_normal=Vec3::new(0.0,0.0,1.0);
        rec.set_face_normal(ray,outward_normal);
        rec.material=self.material.clone();
        rec.p=ray.at(t);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(Vec3::new(self.x0,self.y0,self.k-0.0001),Vec3::new(self.x1,self.y1,self.k+0.0001));
        true
    }
}

pub struct XZRectangle {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn Material>,
}
impl XZRectangle {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {x0,x1,z0,z1,k,material,}
    }
}
impl Hittable for XZRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t=(self.k-ray.origin().y)/ray.direction().y;
        if t<t_min || t>t_max {
            return false;
        }
        let x = ray.origin().x+t*ray.direction().x;
        let z = ray.origin().z+t*ray.direction().z;
        if x<self.x0 || x>self.x1 || z<self.z0 || z>self.z1 {
            return false;
        }
        rec.u=(x-self.x0) / (self.x1-self.x0);
        rec.v=(z-self.z0) / (self.z1-self.z0);
        rec.t=t;
        let outward_normal = Vec3::new(0.0,1.0,0.0);
        rec.set_face_normal(ray, outward_normal);
        rec.material=self.material.clone();
        rec.p=ray.at(t);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(Vec3::new(self.x0,self.k-0.0001,self.z0),Vec3::new(self.x1,self.k+0.0001,self.z1));
        true
    }
}

pub struct YZRectangle {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn Material>,
}
impl YZRectangle {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {y0,y1,z0,z1,k,material,}
    }
}
impl Hittable for YZRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t=(self.k-ray.origin().x)/ray.direction().x;
        if t<t_min || t>t_max {
            return false;
        }
        let y=ray.origin().y+t*ray.direction().y;
        let z=ray.origin().z+t*ray.direction().z;
        if y<self.y0 || y>self.y1 || z<self.z0 || z>self.z1 {
            return false;
        }
        rec.u=(y-self.y0)/(self.y1-self.y0);
        rec.v=(z-self.z0)/(self.z1-self.z0);
        rec.t=t;
        let outward_normal=Vec3::new(1.0,0.0,0.0);
        rec.set_face_normal(ray, outward_normal);
        rec.material=self.material.clone();
        rec.p=ray.at(t);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(Vec3::new(self.k-0.0001,self.y0,self.z0),Vec3::new(self.k+0.0001,self.y1,self.z1));
        true
    }
}
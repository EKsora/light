use crate::ray::Ray;
use crate::aabb::*;
use crate::vec3::Vec3;
use crate::material::Material;
use std::rc::Rc;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,      
    pub normal: Vec3, 
    pub material: Rc<dyn Material>, 
    pub t: f64,       
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(material: Rc<dyn Material>) -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: true,
            material:material,
        }
    }
    pub fn set_face_normal(&mut self, r:&Ray, outward_normal: Vec3) {
        self.front_face = (r.direction() * outward_normal.clone()) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
    pub fn clone(&self)->Self{
        Self {
            p:self.p.clone(),
            normal:self.normal.clone(),
            material:self.material.clone(),
            t:self.t,
            front_face:self.front_face,
        }
    }
}
pub trait Hittable {
    fn hit(&self, ray:&Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}
pub struct HitList {
    pub list: Vec<Rc<dyn Hittable>>,
}
impl HitList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.list.push(object);
    }
    pub fn clear(&mut self) {
        self.list.clear();
    }
}
impl Hittable for HitList {
    fn hit(&self, ray:&Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = rec.clone();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in self.list.iter() {
            if object.hit(&ray.clone(), t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.list.is_empty() {
            return false;
        }
        let mut temp_box: AABB = Default::default();
        let mut first_box = true;

        for object in self.list.iter() {
            if !object.bounding_box(time0, time1, &mut temp_box.clone()) {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                surrounding_box((*output_box).clone(), temp_box.clone())
            };
            first_box = false;
        }
        true
    }
}
use crate::ray::Ray;
use crate::aabb::*;
use crate::vec3::Vec3;
use crate::material::Material;
use std::sync::Arc;
use std::f64::consts::PI;
use std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64)->f64 {
    degrees * PI / 180.0
}

pub struct HitRecord {
    pub p: Vec3,      
    pub normal: Vec3, 
    pub material: Arc<dyn Material>, 
    pub t: f64,      
    pub u: f64,
    pub v: f64, 
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(material: Arc<dyn Material>) -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
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
            u:self.t,
            v:self.t,
            front_face:self.front_face,
        }
    }
}
pub trait Hittable {
    fn hit(&self, ray:&Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}

pub struct HitList {
    pub list: Vec<Arc<dyn Hittable>>,
}
impl HitList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
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

pub struct Translate{
    pub ptr: Arc<dyn Hittable>,
    pub offset:Vec3,
}
impl Translate{
    pub fn new(p:Arc<dyn Hittable>,displacement:Vec3)->Self{
        Self{
            ptr:p.clone(),
            offset:displacement.clone(),
        }
    }
}
impl Hittable for Translate{
    fn hit(&self, r:&Ray, t_min: f64, t_max: f64,mut rec: &mut HitRecord)->bool{
        let moved_r=Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if !self.ptr.hit(& moved_r,t_min,t_max,&mut rec) {
            return false;
        }
        rec.p += self.offset.clone();
        rec.set_face_normal(& moved_r, rec.normal.clone());
        return true;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB)->bool  {
        let mut temp_box:AABB=Default::default();
        if !(self.ptr).bounding_box(time0, time1,&mut temp_box){
            return false;
        }
        *output_box = AABB::new(temp_box.min() + self.offset,temp_box.max() + self.offset);
        return true;
    }
}

pub struct RotateY{
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta:f64,
    pub cos_theta:f64,
    pub hasbox:bool,
    pub bbox:AABB,
}
impl RotateY {
    pub fn new(ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox: AABB = Default::default();
        let hasbox = ptr.bounding_box(0.0,1.0,&mut bbox);
        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x=i as f64 * bbox.max().x+(1-i) as f64 *bbox.min().x;
                    let y=j as f64 * bbox.max().y+(1-j) as f64 *bbox.min().y;
                    let z=k as f64 * bbox.max().z+(1-k) as f64 *bbox.min().z;
                    let newx=cos_theta*x+sin_theta*z;
                    let newz=-sin_theta*x+cos_theta*z;
                    let tmp=Vec3::new(newx,y,newz);
                    min.x = min.x.min(tmp.x);
                    max.x = max.x.min(tmp.x);
                    min.y = min.y.min(tmp.y);
                    max.y = max.y.min(tmp.y);
                    min.z = min.z.min(tmp.z);
                    max.z = max.z.min(tmp.z);
                }
            }
        }
        bbox = AABB::new(min, max);
        Self {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}
impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time11: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();
        self.hasbox
    }
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = ray.origin();
        let mut direction = ray.direction();
        origin.x =ray.origin().x * self.cos_theta - ray.origin().z * self.sin_theta;
        origin.z =ray.origin().x * self.sin_theta + ray.origin().z * self.cos_theta;
        direction.x =ray.direction().x * self.cos_theta - ray.direction().z * self.sin_theta;
        direction.z =ray.direction().x * self.sin_theta + ray.direction().z * self.cos_theta;
        let rotated_r = Ray::new(origin, direction, ray.time());
        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }
        let mut p = rec.p;
        let mut normal = rec.normal;
        p.x = rec.p.x * self.cos_theta + rec.p.z * self.sin_theta;
        p.z = -rec.p.x * self.sin_theta + rec.p.z * self.cos_theta;
        normal.x = rec.normal.x * self.cos_theta + rec.normal.z * self.sin_theta;
        normal.z = -rec.normal.x * self.sin_theta + rec.normal.z * self.cos_theta;
        rec.p = p;
        rec.set_face_normal(&rotated_r, normal);

        true
    }
}
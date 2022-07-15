use crate::ray::Ray;
use crate::vec3::Vec3;
extern crate rand;
#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}
impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
    pub fn min(&self) -> Vec3 {
        self.min.clone()
    }
    pub fn max(&self) -> Vec3 {
        self.max.clone()
    }
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_dir = 1.0 / *r.direction().get(a);
            let mut t0 = (*self.min().clone().get(a) - *r.origin().clone().get(a)) * inv_dir;
            let mut t1 = (*self.max().clone().get(a) - *r.origin().clone().get(a)) * inv_dir;
            if inv_dir < 0. {
                let tmp=t0;
                t0=t1;
                t1=tmp;
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
    pub fn clone(&self)-> Self{
        Self::new(self.min.clone(),self.max.clone())
    }
}
impl Default for AABB {
    fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::zero())
    }
}
pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let min = Vec3::new(
        box0.min().clone().x.min(box1.min().clone().x),
        box0.min().clone().y.min(box1.min().clone().y),
        box0.min().clone().z.min(box1.min().clone().z),
    );
    let max = Vec3::new(
        box0.max().clone().x.max(box1.max().clone().x),
        box0.max().clone().y.max(box1.max().clone().y),
        box0.max().clone().z.max(box1.max().clone().z),
    );
    AABB { min, max }
}
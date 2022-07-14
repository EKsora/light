use crate::aabb::*;
use crate::hit::*;
use crate::Ray;
use std::cmp::Ordering;
use std::rc::Rc;
use std::usize;
use std::vec::Vec;
use crate::Vec3;
#[derive(Clone)]
pub struct BVHNode {
    pub bbox: AABB,
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
}
pub fn random_int_in_range(min: u32, max: u32) -> u32 {
    (min as f64 + (max as f64 +1.0 - min as f64) * rand::random::<f64>())as u32
}
pub fn box_cmp(a: f64, b: f64)->Ordering{
    if a<b {
        Ordering::Less
    }else if a>b{
        Ordering::Greater
    }else{
        Ordering::Equal
    }
}
impl BVHNode {
    pub fn new(
        objects: &mut Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = random_int_in_range(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        }; 
        let object_span = end - start;
        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left = Rc::new(Self::new(objects, start, mid, time0, time1));
            right = Rc::new(Self::new(objects, mid, end, time0, time1));
        }
        let mut box_left: AABB = Default::default();
        let mut box_right: AABB = Default::default();
        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            println!("No bounding box in BVHNode constructor.\n");
        }
        let bbox = surrounding_box(box_left, box_right);
        Self { bbox, left, right }
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let hit_right = self.right.hit(ray, t_min, t_max, rec);
        if hit_left || hit_right {
            return true;
        }
        false
    }
}
impl Hittable for BVHNode {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();
        true
    }
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bbox.clone().hit(ray, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let hit_right = self.right.hit(ray, t_min, if hit_left { rec.t } else { t_max }, rec);
        hit_left || hit_right
    }
}
pub fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: u32) -> Ordering {
    let mut box_a=AABB::new(Vec3::ones(),Vec3::ones());
    let mut box_b=AABB::new(Vec3::zero(),Vec3::zero());

    if !(*a).bounding_box(0.0,0.0,&mut box_a) || !(*b).bounding_box(0.0,0.0, &mut box_b) {
        println!("No bounding box in BVHNode constructor.\n");
    }
    box_cmp(box_a.min().clone().get(axis),box_b.min().clone().get(axis))
}

pub fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
pub fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) ->Ordering {
    box_compare(a, b, 1)
}
pub fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
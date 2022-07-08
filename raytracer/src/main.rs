extern crate rand;
mod vec3;
pub use vec3::Vec3;
mod ray;
pub use ray::Ray;
mod camera;
pub use camera::Camera;
mod hit;
pub use hit::*;
mod sphere;
type Point3=Vec3;
type Color=Vec3;
use std::f64::consts::PI;
use std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) {
    degrees * PI / 180.0;
}

pub fn random_double() -> f64 {
    rand::random::<f64>()
}
pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn write_color(pixel_color:Vec3,samples_per_pixel: u32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale*r).sqrt();
    g = (scale*g).sqrt();
    b = (scale*b).sqrt();
    let ir =( (256 as f64) * clamp(r, 0.0, 0.999)) as u32;
    let ig =( (256 as f64)* clamp(g, 0.0, 0.999)) as u32;
    let ib =( (256 as f64)* clamp(b, 0.0, 0.999)) as u32;
    print!("{} {} {}\n", ir, ig, ib);
}

pub fn hit_sphere(center:Vec3,radius:f64,r:&Ray) -> f64 {
    let oc:Vec3 = r.origin() - center.clone();
    let a = r.direction().squared_length();
    let half_b =  oc.clone()*r.direction();
    let c = oc.clone().squared_length() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        -1.0
    }else{
        (-half_b - discriminant.sqrt())  / a
    }
}

pub fn ray_color(r:&Ray,world:&hit::HitList,depth:u32)->Vec3{
    if depth<=0{
        return Vec3::new(0.0,0.0,0.0);
    }
    let mut rec = HitRecord::new(Vec3::zero(),Vec3::zero(),0.0,false);
    if world.hit((*r).clone(),0.0,INFINITY,&mut rec){
        let target = rec.clone().p + Vec3::random_in_hemisphere(&rec.clone().normal);
        return ray_color(&Ray::new(rec.clone().p, target - rec.clone().p),world,depth - 1) * 0.5;
    }else{
    let unit_direction=Vec3{
        x:r.direction().unit().x,
        y:r.direction().unit().y,
        z:r.direction().unit().z,
    };
    let t = 0.5*(unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
    }
}

fn main() {
    const aspect_ratio:f64 = 16.0 / 9.0;
    const image_width:u32 = 400;
    const image_height:u32 =(image_width as f64/ aspect_ratio)as u32;
    const samples_per_pixel:u32 = 100;
    const max_depth:u32 = 50;
    let mut world = hit::HitList::new();
    world.add(Box::new(sphere::Sphere::new(Vec3::new(0.0, 0.0, -1.0),0.5,)));
    world.add(Box::new(sphere::Sphere::new(Vec3::new(0.0, -100.5, -1.0),100.0,)));
    let cam=Camera::new(); 
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev(){
        for i in (0..image_width){
            let mut pixel_color = Vec3::zero();
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64);
                let v = (j as f64 + random_double()) / (image_height as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world,max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
}
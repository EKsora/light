extern crate rand;
mod vec3;
pub use vec3::Vec3;
mod ray;
pub use ray::Ray;
mod camera;
pub use camera::Camera;
mod material;
pub use material::*;
mod hit;
pub use hit::*;
mod sphere;
mod aabb;
mod bvh;
mod texture;
pub use texture::CheckerTexture;
pub use crate::bvh::BVHNode;
type Point3=Vec3;
type Color=Vec3;
use std::sync::Arc;
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
    let ir =( (256 as f64)* clamp(r, 0.0, 0.999)) as u32;
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
        (-half_b - discriminant.sqrt()) / a
    }
}

pub fn ray_color(r:&Ray,world:&Arc<BVHNode>,depth:u32)->Vec3{
    if depth<=0{
        return Vec3::new(0.0,0.0,0.0);
    }
    let mut rec = HitRecord::new(Arc::new(Lambertian::new(Vec3::new(0.0,0.0,0.0))));
    if world.hit(&(r).clone(),0.00001,INFINITY,&mut rec){
        let mut scattered = Ray::new(Vec3::new(0.0,0.0,0.0),Vec3::new(0.0,0.0,0.0),0.0);
        let mut attenuation = Vec3::new(0.0,0.0,0.0);
        if rec.material.scatter(&r, &rec, &mut attenuation, &mut scattered){
            return Vec3::elemul(attenuation, ray_color(&scattered, world, depth - 1));
        }
        return Vec3::new(0.0,0.0,0.0);
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

fn random_scene()->hit::HitList{
    let mut world = hit::HitList::new();
    //let material_ground = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let checker = Arc::new(CheckerTexture::new_rgb(Vec3::new(0.2, 0.3, 0.1),Vec3::new(0.9, 0.9, 0.9),));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0, -1000.0, 0.0),1000.0,Arc::new(Lambertian::new_texture(checker)))));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center=Vec3::new((a as f64)+0.9*random_double(),0.2,(b as f64) + 0.9*random_double());

            if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if (choose_mat < 0.8) {
                    let albedo =Vec3::elemul(Vec3::random(),Vec3::random());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center.clone()+Vec3::new(0.0,random_double_in_range(0.0,0.5),0.0);
                    world.add(Arc::new(sphere::MovingSphere::new(center.clone(),center2.clone(),0.0,1.0,0.2,sphere_material.clone())));
                    //world.add(Arc::new(sphere::Sphere::new(center.clone(),0.2,sphere_material.clone())));
                } else if (choose_mat < 0.95) {
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = random_double_in_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(sphere::Sphere::new(center.clone(),0.2,sphere_material.clone())));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(sphere::Sphere::new(center.clone(),0.2,sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1.clone())));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2.clone())));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3.clone())));

    world
}

pub fn two_spheres() -> HitList {
    let mut world = HitList::new();

    let checker = Arc::new(CheckerTexture::new_rgb(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,-10.0,0.0),10.0,Arc::new(Lambertian::new_texture(checker.clone())))));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,10.0,0.0),10.0,Arc::new(Lambertian::new_texture(checker.clone())))));
    world
}


fn main() {
    const aspect_ratio:f64 = 16.0 / 9.0;
    const image_width:u32 = 400;
    const image_height:u32 =(image_width as f64/ aspect_ratio)as u32;
    const samples_per_pixel:u32 = 100;
    const max_depth:u32 = 50;
    let r = (PI / 4.0).cos(); 
    //let hit_list = Arc::new(random_scene());
    let hit_list: Arc<HitList>;
    let lookfrom: Vec3;
    let lookat: Vec3;
    let vfov: f64;
    let mut aperture=0.0;
    match 0 {
        1 => {
            hit_list = Arc::new(random_scene());
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        _ => {
            hit_list = Arc::new(two_spheres());
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
    }

    let world = Arc::new(BVHNode::new(&mut hit_list.list.clone(),0,hit_list.list.len(),0.0,1.0,));
    //let lookfrom=Vec3::new(13.0,2.0,3.0);
    //let lookat=Vec3::new(0.0,0.0,0.0);
    let vup=Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    //let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus,0.0,1.0);

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
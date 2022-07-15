extern crate rand;
extern crate image;
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
mod cbox;
pub use cbox::*;
mod aarect;
pub use aarect::*;
mod texture;
pub use texture::*;
mod perlin;
pub use crate::bvh::BVHNode;
pub use image::*;
pub use image::{ImageBuffer, RgbImage};
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

pub fn ray_color(r:&Ray,background:&Vec3,world:&Arc<BVHNode>,depth:u32)->Vec3{
    if depth<=0{
        return Vec3::new(0.0,0.0,0.0);
    }
    let mut rec = HitRecord::new(Arc::new(Lambertian::new(Vec3::new(0.0,0.0,0.0))));
    if !world.hit(&r, 0.001, INFINITY, &mut rec) { return *background; }
    let mut scattered = Ray::new(Vec3::new(0.0,0.0,0.0),Vec3::new(0.0,0.0,0.0),0.0);
    let mut attenuation = Vec3::new(0.0,0.0,0.0);let emitted =rec.material.emitted(rec.u,rec.v,&rec.p);
    if !rec.material.scatter(&r, &rec, &mut attenuation, &mut scattered){
        return emitted;
    }
    /*else{
    let unit_direction=Vec3{
        x:r.direction().unit().x,
        y:r.direction().unit().y,
        z:r.direction().unit().z,
    };
    let t = 0.5*(unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
    }*/
    emitted+Vec3::elemul(attenuation,ray_color(&scattered,background,world,depth-1))
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

pub fn two_perlin_spheres() -> HitList {
    let mut world = HitList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,-1000.0, 0.0),1000.0,Arc::new(Lambertian::new_texture(pertext.clone())))));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,2.0,0.0),2.0,Arc::new(Lambertian::new_texture(pertext.clone())))));
    world
}

pub fn earth() -> HitList {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_texture(earth_texture));
    let globe = Arc::new(sphere::Sphere::new(Vec3::new(0.0,0.0,0.0),2.0,earth_surface));
    let mut world = HitList::new();
    world.add(globe);
    world
}

pub fn simple_light() -> HitList {
    let mut world = HitList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,Arc::new(Lambertian::new_texture(pertext.clone())))));
    world.add(Arc::new(sphere::Sphere::new(Vec3::new(0.0,2.0,0.0),2.0,Arc::new(Lambertian::new_texture(pertext)).clone())));
    let diff_light = Arc::new(DiffuseLight::new(Vec3::new(4.0,4.0,4.0)));
    world.add(Arc::new(XYRectangle::new(3.0,5.0,1.0,3.0,-2.0,diff_light)));
    world
}

pub fn cornell_box() -> HitList {
    let mut world = HitList::new();
    let red = Arc::new(Lambertian::new(Vec3::new(0.65,0.05,0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73,0.73,0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12,0.45,0.15)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(15.0,15.0,15.0)));

    world.add(Arc::new(YZRectangle::new(0.0,555.0,0.0,555.0,555.0,green)));
    world.add(Arc::new(YZRectangle::new(0.0,555.0,0.0,555.0,0.0,red)));
    world.add(Arc::new(XZRectangle::new(213.0,343.0,227.0,332.0,554.0, light)));
    world.add(Arc::new(XZRectangle::new(0.0,555.0,0.0,555.0,0.0,white.clone())));
    world.add(Arc::new(XZRectangle::new(0.0,555.0,0.0,555.0,555.0,white.clone())));
    world.add(Arc::new(XYRectangle::new(0.0,555.0,0.0,555.0,555.0,white.clone())));
    world.add(Arc::new(CornellBox::new(Vec3::new(130.0,0.0,65.0),Vec3::new(295.0,165.0,230.0),white.clone())));
    world.add(Arc::new(CornellBox::new(Vec3::new(265.0,0.0,295.0),Vec3::new(430.0,330.0,460.0),white.clone())));
    world
}

fn main() {
    let mut aspect_ratio: f64 = 16.0 / 9.0;
    let mut image_width: usize = 400;
    const max_depth:u32 = 50;
    let mut samples_per_pixel:u32 = 100;
    let r = (PI / 4.0).cos(); 
    let hit_list: Arc<HitList>;
    let lookfrom: Vec3;
    let lookat: Vec3;
    let background: Vec3;
    let vfov: f64;
    let mut aperture=0.0;
    match 4 {
        1 => {
            hit_list=Arc::new(random_scene());
            background=Vec3::new(0.7,0.8,1.0);
            lookfrom=Vec3::new(13.0,2.0,3.0);
            lookat=Vec3::new(0.0,0.0,0.0);
            vfov=20.0;
            aperture=0.1;
        }
        2 => {
            hit_list=Arc::new(two_spheres());
            background=Vec3::new(0.7,0.8,1.0);
            lookfrom=Vec3::new(13.0,2.0,3.0);
            lookat=Vec3::new(0.0,0.0,0.0);
            vfov=20.0;
        }
        3 => {
            hit_list=Arc::new(two_perlin_spheres());
            background=Vec3::new(0.7,0.8,1.0);
            lookfrom=Vec3::new(13.0,2.0,3.0);
            lookat=Vec3::new(0.0,0.0,0.0);
            vfov=20.0;
        }
        4 => {
            hit_list=Arc::new(earth());
            background=Vec3::new(0.7,0.8,1.0);
            lookfrom=Vec3::new(13.0,2.0,3.0);
            lookat=Vec3::new(0.0,0.0,0.0);
            vfov=20.0;
        }
        5 => {
            hit_list=Arc::new(simple_light());
            samples_per_pixel=400;
            background=Vec3::zero();
            lookfrom=Vec3::new(26.0,3.0,6.0);
            lookat=Vec3::new(0.0,2.0,0.0);
            vfov=20.0;
        }
        6 => {
            hit_list=Arc::new(cornell_box());
            aspect_ratio=1.0;
            image_width=600;
            samples_per_pixel=200;
            background=Vec3::zero();
            lookfrom=Vec3::new(278.0,278.0,-800.0);
            lookat=Vec3::new(278.0,278.0,0.0);
            vfov=40.0;
        }
        _ => {
            hit_list=Arc::new(HitList::new());
            background=Vec3::new(0.0,0.0,0.0);
            lookfrom=Vec3::new(0.0,0.0,0.0);
            lookat=Vec3::new(0.0,0.0,0.0);
            vfov=40.0;
        }
    }

    let mut image_height:usize =(image_width as f64/ aspect_ratio)as usize;
    let world = Arc::new(BVHNode::new(&mut hit_list.list.clone(),0,hit_list.list.len(),0.0,1.0,));
    let vup=Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus,0.0,1.0);

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev(){
        for i in (0..image_width){
            let mut pixel_color = Vec3::zero();
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64);
                let v = (j as f64 + random_double()) / (image_height as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r,&background,&world,max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
}
//#![allow(clippy::float_cmp)]
//#![feature(box_syntax)]
/*
mod material;
mod scene;
*/
mod vec3;
pub use vec3::Vec3;
//use crate::vec3::Vec3;
/*
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rusttype::Font;
use scene::example_scene;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
const AUTHOR: &str = "EKsora";
*/
type Point3=Vec3;
type Color=Vec3;

fn write_color(pixel_color:Vec3) {
    let ir =( 255.999 * pixel_color.x) as u32;
    let ig =( 255.999 * pixel_color.y) as u32;
    let ib =( 255.999 * pixel_color.z) as u32;
    print!("{} {} {}\n", ir, ig, ib);
}

pub struct Ray{
    pub orig:Vec3,
    pub dir:Vec3,
}
impl Ray {
    pub fn origin(&self)->Vec3{
        Vec3::new(self.orig.x,self.orig.y,self.orig.z)
    }
    pub fn direction(&self)->Vec3{
        Vec3::new(self.dir.x,self.dir.y,self.dir.z)
    }
    pub fn at(&self,t:f64)->Vec3{
        Vec3::new(self.orig.x+t*self.dir.x,self.orig.y+t*self.dir.y,self.orig.z+t*self.dir.z)
    }
}


pub fn hit_sphere(center:Vec3,radius:f64,r:&Ray) -> bool {
    let oc:Vec3 = r.origin() - center.clone();
    let a = r.direction()*r.direction();
    let b =  oc.clone()*r.direction()*2.0;
    let c = oc.clone()*oc.clone() - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

pub fn ray_color(r:&Ray)->Vec3{
    if hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, r){
        Vec3::new(1.0,0.0,0.0)
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

/*
pub struct World {
    pub height: u32,
}

impl World {
    pub fn color(&self, _: u32, y: u32) -> u8 {
        (y * 256 / self.height) as u8
    }
}

fn get_text() -> String {
    // GITHUB_SHA is the associated commit ID
    // only available on GitHub Action
    let github_sha = option_env!("GITHUB_SHA")
        .map(|x| "@".to_owned() + &x[0..6])
        .unwrap_or_default();
    format!("{}{}", AUTHOR, github_sha)
}

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn render_text(image: &mut RgbImage, msg: &str) {
    let font_file = if is_ci() {
        "EncodeSans-Regular.ttf"
    } else {
        "/System/Library/Fonts/Helvetica.ttc"
    };
    let font_path = std::env::current_dir().unwrap().join(font_file);
    let data = std::fs::read(&font_path).unwrap();
    let font: Font = Font::try_from_vec(data).unwrap_or_else(|| {
        panic!(format!(
            "error constructing a Font from data at {:?}",
            font_path
        ));
    });

    imageproc::drawing::draw_text_mut(
        image,
        Rgb([255, 255, 255]),
        10,
        10,
        rusttype::Scale::uniform(24.0),
        &font,
        msg,
    );
}

*/
fn main() {
    /*
    // get environment variable CI, which is true for GitHub Action
    let is_ci = is_ci();

    // jobs: split image into how many parts
    // workers: maximum allowed concurrent running threads
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 2) };

    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );

    let height = 512;
    let width = 1024;

    // create a channel to send objects between threads
    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);

    let bar = ProgressBar::new(n_jobs as u64);

    // use Arc to pass one instance of World to multiple threads
    let world = Arc::new(example_scene());

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
        pool.execute(move || {
            // here, we render some of the rows of image in one thread
            let row_begin = height as usize * i / n_jobs;
            let row_end = height as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(width, render_height );
            for x in 0..width {
                // img_y is the row in partial rendered image
                // y is real position in final image
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y ;
                    let pixel = img.get_pixel_mut(x, img_y );
                    let color = world_ptr.color(x, y);
                    *pixel = Rgb([color, color, color]);
                }
            }
            // send row range and rendered image to main thread
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }

    let mut result: RgbImage = ImageBuffer::new(width, height);

    for (rows, data) in rx.iter().take(n_jobs) {
        // idx is the corrsponding row in partial-rendered image
        for (idx, row) in rows.enumerate() {
            for col in 0..width {
                let row = row ;
                let idx = idx ;
                *result.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
            }
        }
        bar.inc(1);
    }
    bar.finish();

    // render commit ID and author name on image
    let msg = get_text();
    println!("Extra Info: {}", msg);

    render_text(&mut result, msg.as_str());
    result.save("output/test.png").unwrap();
*/
    const aspect_ratio:f64 = 16.0 / 9.0;
    const image_width:u32 = 400;
    const image_height:u32 =(image_width as f64/ aspect_ratio)as u32;

    let viewport_height:f64 = 2.0;
    let viewport_width:f64 = aspect_ratio * viewport_height;
    let focal_length:f64 = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone() - horizontal.clone()/2.0 - vertical.clone()/2.0 - Vec3::new(0.0, 0.0, focal_length);
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev(){
        for i in (0..image_width){
            let u:f64=i as f64 / (image_width-1)as f64;
            let v:f64=j as f64 / (image_height-1)as f64;
            let r=Ray{
                orig:origin.clone(), 
                dir:lower_left_corner.clone() + horizontal.clone()*u + vertical.clone()*v - origin.clone(),
            };
            let pixel_color=ray_color(&r);
            write_color(pixel_color);
        }
    }
}
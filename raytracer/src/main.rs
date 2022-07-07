mod vec3;
pub use vec3::Vec3;
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


pub fn hit_sphere(center:Vec3,radius:f64,r:&Ray) -> f64 {
    let oc:Vec3 = r.origin() - center.clone();
    let a = r.direction()*r.direction();
    let b =  oc.clone()*r.direction()*2.0;
    let c = oc.clone()*oc.clone() - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        -1.0
    }else{
        (-b - discriminant.sqrt())  / (2.0*a)
    }
}

pub fn ray_color(r:&Ray)->Vec3{
    let mut t = hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, r);
    if t>0.0 {
        let n = (r.at(t) - Vec3::new(0.0,0.0,-1.0)).unit();
        Vec3::new(n.x+1.0, n.y+1.0, n.z+1.0)*0.5
    }else{
    let unit_direction=Vec3{
        x:r.direction().unit().x,
        y:r.direction().unit().y,
        z:r.direction().unit().z,
    };
    t = 0.5*(unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
    }
}

fn main() {
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
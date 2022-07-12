use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]

pub struct Ray{
    pub orig:Vec3,
    pub dir:Vec3,
    pub time: f64,
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
    pub fn time(&self) -> f64 {
        self.time
    }
    pub fn clone(&self)->Self{
        Self {
            orig: self.orig.clone() ,
            dir: self.dir.clone() ,
            time:self.time,
        }
    }
    pub fn new(orig:Vec3,dir:Vec3, time: f64)->Self{
        Self {
            orig:Vec3::new(0.0,0.0,0.0),
            dir:Vec3::new(0.0,0.0,0.0),
            time:0.0,
        }
    }
}

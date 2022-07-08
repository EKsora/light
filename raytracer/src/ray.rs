use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]

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
    pub fn clone(&self)->Self{
        Self {
            orig: self.orig.clone() ,
            dir: self.dir.clone() ,
        }
    }
}

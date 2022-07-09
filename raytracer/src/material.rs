use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
pub trait Material {
    fn scatter(&self,r_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self,_r_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal.clone() + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        *scattered = Ray::new(rec.p.clone(), scatter_direction);
        *attenuation = self.albedo.clone();
        true
    }
}
pub struct Metal {
    albedo: Vec3,
}
impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}
impl Material for Metal {
    fn scatter(&self,r_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit(&r_in.direction()), &rec.normal.clone());
        *scattered = Ray::new(rec.p.clone(), reflected);
        *attenuation = self.albedo.clone();
        scattered.direction() * rec.normal.clone() > 0.0
    }
}
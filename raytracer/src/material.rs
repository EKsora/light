extern crate rand;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn fmin(a: f64, b: f64) -> f64 {
    if a>b {b} else {a}
}

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
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self { 
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },}
    }
}
impl Material for Metal {
    fn scatter(&self,r_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit(&r_in.direction()), &rec.normal.clone());
        *scattered = Ray::new(rec.p.clone(), reflected+ Vec3::random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo.clone();
        scattered.direction() * rec.normal.clone() > 0.0
    }
}

pub struct Dielectric {
    ref_idx: f64,
}
impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0+(1.0 - r0)*(1.0 - cosine).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = Vec3::unit(&r_in.direction());
        let cos_theta = fmin(-unit_direction.clone() * rec.normal.clone(), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>() {
                Vec3::reflect(&unit_direction, &rec.normal)
            } else {
                Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };
        *scattered = Ray::new(rec.p.clone(), direction);
        true
    }
}
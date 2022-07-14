use crate::vec3::Vec3;
extern crate rand;
static point_count: usize = 256;

pub fn random_int_in_range(min: usize, max: usize) -> usize {
    (min as f64 + (max as f64 +1.0 - min as f64) * rand::random::<f64>())as usize
}

pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}
impl Perlin {
    pub fn new() -> Self {
        let mut ran_vec = vec![Vec3::zero(); point_count];
        for item in ran_vec.iter_mut().take(point_count) {
            *item = Vec3::unit(&Vec3::random_in_range(-1.0, 1.0));
        }
        Self {
            ran_vec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let u=p.x-p.x.floor();
        let v=p.y-p.y.floor();
        let w=p.z-p.z.floor();
        /*
        u=u*u*(3.0-2.0*u);
        v=v*v*(3.0-2.0*v);
        w=w*w*(3.0-2.0*w);
        */
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = vec![[[Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_vec[(self.perm_x[((i + di as i32) & 255) as usize]^ self.perm_y[((j + dj as i32) & 255) as usize]^ self.perm_z[((k + dk as i32) & 255) as usize])as usize].clone();
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = vec![0; point_count];
        for (i, item) in p.iter_mut().enumerate().take(point_count) {
            *item = i as i32;
        }
        Self::permute(&mut p, point_count);
        p
    }
    fn permute(p: &mut[i32], n: usize) {
        for i in (1..n).rev() {
            let target = random_int_in_range(0, i) as usize;
            p.swap(i, target);
        }
    }
    pub fn turb(&self, p: &Vec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }
        accum.abs()
    }
    fn trilinear_interp(c: Vec<[[f64; 2]; 2]>, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.;
        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1. - i as f64) * (1. - u))* (j as f64 * v + (1. - j as f64) * (1. - v as f64))* (k as f64 * w as f64 + (1. - k as f64) * (1. - w as f64))* c[i][j][k];
                }
            }
        }
        accum
    }
    fn perlin_interp(c: Vec<[[Vec3; 2]; 2]>, u: f64, v: f64, w: f64) -> f64 {
        let iu=u*u*(3.0-2.0*u);
        let iv=v*v*(3.0-2.0*v);
        let iw=w*w*(3.0-2.0*w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum+=(i as f64*iu+(1.0-i as f64)*(1.0-iu))*(j as f64*iv+(1.0-j as f64)*(1.0-iv as f64))*(k as f64 *iw as f64+(1.0-k as f64)*(1.0-iw as f64))* (c[i][j][k].clone() * weight_v.clone());
                }
            }
        }
        accum
    }
}
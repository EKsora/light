use crate::vec3::Vec3;
extern crate rand;
static point_count: usize = 256;

pub fn random_int_in_range(min: usize, max: usize) -> usize {
    (min as f64 + (max as f64 +1.0 - min as f64) * rand::random::<f64>())as usize
}

pub struct Perlin {
    ran_float: Vec<f64>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}
impl Perlin {
    pub fn new() -> Self {
        let mut ran_float = vec![0.0; point_count];
        for item in ran_float.iter_mut().take(point_count) {
            *item = rand::random::<f64>();
        }
        Self {
            ran_float,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let i = ((p.x * 4.) as u32 & 255) as usize;
        let j = ((p.y * 4.) as u32 & 255) as usize;
        let k = ((p.z * 4.) as u32 & 255) as usize;
        self.ran_float[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
    fn perlin_generate_perm() -> Vec<u32> {
        let mut p: Vec<u32> = vec![0; point_count];
        for (i, item) in p.iter_mut().enumerate().take(point_count) {
            *item = i as u32;
        }
        Self::permute(&mut p, point_count);
        p
    }
    fn permute(p: &mut[u32], n: usize) {
        for i in (1..n).rev() {
            let target = random_int_in_range(0, i);
            p.swap(i, target);
        }
    }
}
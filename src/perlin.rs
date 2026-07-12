use crate::rtweekend::{random_double, random_int};
use crate::vec3::Point3;

pub struct Perlin {
    randfloat: [f64; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut randfloat = [0.0; Self::POINT_COUNT];
        for val in &mut randfloat {
            *val = random_double();
        }

        let mut perm_x = [0; Self::POINT_COUNT];
        let mut perm_y = [0; Self::POINT_COUNT];
        let mut perm_z = [0; Self::POINT_COUNT];

        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);

        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.x) as i32 & 255) as usize;
        let j = ((4.0 * p.y) as i32 & 255) as usize;
        let k = ((4.0 * p.z) as i32 & 255) as usize;

        self.randfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm(p: &mut [usize]) {
        for (i, val) in p.iter_mut().enumerate() {
            *val = i;
        }

        Self::permute(p);
    }

    fn permute(p: &mut [usize]) {
        for i in (1..p.len()).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
    }
}

use std::array;

use crate::rtweekend::random_int;
use crate::vec3::{Point3, Vec3, dot};

pub struct Perlin {
    randvec: [Vec3; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

#[allow(dead_code)]
impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let randvec = array::from_fn(|_| Vec3::random_vec3_range(-1.0, 1.0).unit_vector());

        let mut perm_x = [0; Self::POINT_COUNT];
        let mut perm_y = [0; Self::POINT_COUNT];
        let mut perm_z = [0; Self::POINT_COUNT];

        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for (di, arr_di) in c.iter_mut().enumerate() {
            for (dj, arr_dj) in arr_di.iter_mut().enumerate() {
                for (dk, val) in arr_dj.iter_mut().enumerate() {
                    let idx = self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize];
                    *val = self.randvec[idx];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
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

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, arr_i) in c.iter().enumerate() {
            for (j, arr_j) in arr_i.iter().enumerate() {
                for (k, val) in arr_j.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * dot(val, &weight_v);
                }
            }
        }

        accum
    }
}

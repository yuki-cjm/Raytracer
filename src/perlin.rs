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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[0.0; 2]; 2]; 2];

        // 修复：使用 iter_mut().enumerate() 替代 0..2 索引，兼容 Clippy 规则
        for (di, arr_di) in c.iter_mut().enumerate() {
            for (dj, arr_dj) in arr_di.iter_mut().enumerate() {
                for (dk, val) in arr_dj.iter_mut().enumerate() {
                    let idx = self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize];
                    *val = self.randfloat[idx];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
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

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        // 修复：使用 iter().enumerate() 替代 0..2 索引，兼容 Clippy 规则
        for (i, arr_i) in c.iter().enumerate() {
            for (j, arr_j) in arr_i.iter().enumerate() {
                for (k, &val) in arr_j.iter().enumerate() {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * val;
                }
            }
        }

        accum
    }
}

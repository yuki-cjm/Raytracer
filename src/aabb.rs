use std::ops;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn default() -> Self {
        Self {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }

    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
        let mut ans = Self {
            x: *x,
            y: *y,
            z: *z,
        };
        ans.pad_to_minimums();
        ans
    }

    pub fn new_from_points(a: &Point3, b: &Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        let mut ans = Self { x, y, z };
        ans.pad_to_minimums();
        ans
    }

    pub fn new_from_boxes(box0: &Self, box1: &Self) -> Self {
        Self {
            x: Interval::new_from_intervals(&box0.x, &box1.x),
            y: Interval::new_from_intervals(&box0.y, &box1.y),
            z: Interval::new_from_intervals(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        if n == 1 {
            self.y
        } else if n == 2 {
            self.z
        } else {
            self.x
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let mut ray_t = ray_t;
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.dir[axis];

            let t0 = (ax.min - r.orig[axis]) * adinv;
            let t1 = (ax.max - r.orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> i32 {
        // Returns the index of the longest axis of the bounding box.

        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Self = Self {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    fn pad_to_minimums(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.

        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}

impl ops::Add<Vec3> for Aabb {
    type Output = Aabb;

    fn add(self, offset: Vec3) -> Aabb {
        Aabb {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}
impl ops::Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, bbox: Aabb) -> Aabb {
        bbox + self
    }
}
